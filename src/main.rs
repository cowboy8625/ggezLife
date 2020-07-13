mod game;
mod grid;
mod commands;
mod game_event;

use ggez::{input::mouse::MouseButton, GameResult, Context, event, event::KeyCode, graphics};

use game::Game;
use commands::{key_mapper, Mapper};
use grid::Grid;

const WINDOW: CellPoint<i32> = CellPoint { x:1400, y:800 };
const GRID: i32 = 2;
const CELL: CellPoint<i32> = CellPoint { 
    x: WINDOW.x / GRID,
    y: WINDOW.y / GRID, 
};
const LIMITS: CellPoint<i32> = CellPoint { x: 2, y: 100 };
const STEP: i32 = 2;

#[derive(Clone)]
struct CellPoint<T> { x: T, y: T, }

struct Mouse {
    left: bool,
    middle: bool,
}

struct GameState {
    game: Game,
    key: Mapper,
    mouse: Mouse,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            game: Game::new(),
            key: key_mapper(),
            mouse: Mouse { left: false, middle: false },
        }
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.game.make_screen();

        if self.mouse.left {
            let pos = ggez::input::mouse::position(ctx);
            self.game.place_cell(pos.x as i32, pos.y as i32);
            // TODO: make line pattern.
            self.game.add_point((pos.x, pos.y));
        }

        if self.mouse.middle {
            let pos = ggez::input::mouse::position(ctx);
            dbg!(pos);
        }

        self.game.update();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [1.0, 1.0, 1.0, 1.0].into());

        display_board(ctx, &self.game.screen, self.game.grid)?;

        graphics::present(ctx)?;
        ggez::timer::yield_now();
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymod: ggez::input::keyboard::KeyMods, _repeat: bool) {
        if let Some(event) = self.key.get(keycode) {
            self.game.key_event(event);
        }
        if let KeyCode::Escape = keycode {
            event::quit(ctx);
        }
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        _y: f32
    ) {
        if let MouseButton::Left = button {
            self.mouse.left = true;
        }
        if let MouseButton::Middle = button {
            self.mouse.middle = true;
        }

    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        _y: f32
    ) {
        if let MouseButton::Left = button {
            self.mouse.left = false;
        }
        if let MouseButton::Middle = button {
            self.mouse.middle = false;
        }

    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, y: f32) {
        use game_event::GameEvent::{ZoomIn, ZoomOut};
        if y > 0.0 {
            self.game.key_event(ZoomOut);
        } else if y < 0.0 {
            self.game.key_event(ZoomIn);
        }
    }        
}


fn main() -> GameResult {

    let (mut ctx, mut event_loop) = ggez::ContextBuilder::new("life", "Cowboy8625")
        .window_setup(ggez::conf::WindowSetup::default().title("Game Of Life"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(WINDOW.x as f32, WINDOW.y as f32))
        .build().expect("Failed to build ggez context");

    let state = &mut GameState::new();
    event::run(&mut ctx, &mut event_loop, state)?;
    // game.place_cell(mouse.x(), mouse.y(), mouse.left());
    Ok(())
}

fn display_board(ctx: &mut Context, cells: &Grid, grid: i32) -> GameResult {
    let mut meshbuilder = graphics::MeshBuilder::new();
    let mut count = 0;
    for y in 0..cells.height {
        for x in 0..cells.width {
            if cells[y][x] {
                meshbuilder.rectangle(
                        graphics::DrawMode::fill(),
                        graphics::Rect::new_i32(x as i32 * grid, y as i32 * grid, grid, grid),
                        [0.0, 0.0, 0.0, 1.0].into(),
                    );
                count += 1;
            }
        }
    }
    if count > 0 {
        let mesh = meshbuilder.build(ctx)?;
        graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
    }
    Ok(())
}

fn alive(x: i32, y: i32, v: &Grid) -> bool {

    let n = cell_count(x as usize, y as usize, v);
    let curr = v[y as usize][x as usize] as i32;

    match (curr,  n) { (1, 0..=1) => false, (1, 4..=8) => false,
        (1, 2..=3) => true,
        (0, 3)     => true,
        (0, 0..=2) => false,
        (0, 4..=8) => false,
        _ => panic!("alive: error in match"),
    }
}

fn inc_x(n: usize) ->  usize {
    (n + 1) % CELL.x as usize
}

fn dec_x(n: usize) -> usize {
    if n == 0 { CELL.x as usize - 1 } else { (n - 1) as usize }
}

fn inc_y(n: usize) ->  usize {
    (n + 1) % CELL.y as usize
}

fn dec_y(n: usize) -> usize {
    if n == 0 { CELL.y as usize - 1 } else { n - 1 }
}

fn cell_count(x: usize, y: usize, v: &Grid) -> i32 {
    v[dec_y(y)][x] as i32 +
    v[inc_y(y)][x] as i32 +
    v[y][dec_x(x)] as i32 +
    v[y][inc_x(x)] as i32 +
    v[dec_y(y)][dec_x(x)] as i32 +
    v[dec_y(y)][inc_x(x)] as i32 +
    v[inc_y(y)][inc_x(x)] as i32 +
    v[inc_y(y)][dec_x(x)] as i32
}

