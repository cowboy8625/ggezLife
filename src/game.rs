use crate::{CELL, WINDOW, LIMITS, STEP, CellPoint};
use crate::grid::Grid;
use crate::game_event::GameEvent;
pub struct Game {
    pub grid: i32,
    win: CellPoint<i32>,
    loc: CellPoint<i32>,
    speed: std::time::Duration,
    block: bool,
    is_paused: bool,
    is_running: bool,
    cells: Grid,
    pub screen: Grid,
    time: std::time::Instant,
    points: Vec<(f32, f32)>,
}

// Public
impl Game {
    pub fn new() -> Self {
        let cells = Grid::from_fn(CELL.x as usize, CELL.y as usize, || false);
        let grid: i32 = 20; 
        let win: CellPoint<i32> = CellPoint { x: WINDOW.x / grid, y: WINDOW.y / grid };
        let loc: CellPoint<i32> = CellPoint { x: 0, y: 0 };
        let screen = cells.sub_grid(loc.x as usize, loc.y as usize, win.x as usize, win.y as usize);
        let speed = std::time::Duration::from_millis(50);
        Self {
            grid,
            win,
            loc,
            speed,
            block: true,
            is_paused: true,
            is_running: true,
            cells,
            screen,
            time: std::time::Instant::now() + speed,
            points: Vec::new(),
        }
    }

    pub fn make_screen(&mut self) {
        self.clamp();
        self.screen = self.cells.sub_grid(self.loc.x as usize, self.loc.y as usize, self.win.x as usize,self. win.y as usize);
    }

    pub fn update(&mut self) {
        if self.is_paused && self.time < std::time::Instant::now(){
            self.next_gen();
        }
    }

    pub fn key_event(&mut self, event: GameEvent) {
        use GameEvent::*;
        match event {
            Quit => self.quit(),
            SpeedUp => self.speed_up(),
            SpeedDown => self.speed_down(),
            SpawnRandom => self.spawn_random(),
            ClearBoard => self.clear_board(),
            StartStop => self.start_stop(),
            ScreenUp => self.screen_up(),
            ScreenDown => self.screen_down(),
            ScreenLeft => self.screen_left(),
            ScreenRight => self.screen_right(),
            ZoomOut => self.zoom_out(),
            ZoomIn => self.zoom_in(),
            SwapBlock => self.swap_block(),
        }
    }

    pub fn place_cell(&mut self, x: i32, y: i32) {
        self.pause();
        let screen_x: usize = (x / self.grid) as usize;
        let screen_y: usize = (y / self.grid) as usize;
        let (world_x, world_y) = (self.loc.x as usize + screen_x, self.loc.y as usize + screen_y);
        self.cells[world_y][world_x] = self.block;
    }


    pub fn add_point(&mut self, point: (f32, f32)) {
        self.points.push(point);
    }

    pub fn start_stop(&mut self) {
        if self.is_paused { self.pause() } else { self.unpause() };
    }

    pub fn zoom_in(&mut self) {
        if self.grid != LIMITS.x {
            self.grid -= STEP;
            self.win.x = WINDOW.x / self.grid;
            self.win.y = WINDOW.y / self.grid;
        }
    }

    pub fn zoom_out(&mut self) {
        if self.grid != LIMITS.y {
            self.grid += STEP;
            self.win.x = WINDOW.x / self.grid;
            self.win.y = WINDOW.y / self.grid;
        }
    }
}

// Privet
impl Game {
    // def gen_points(x0, y0, x1, y1):
    //     dx = x1 - x0
    //     dy = y1 - y0
    //
    //     xsign = 1 if dx > 0 else -1
    //     ysign = 1 if dy > 0 else -1
    //
    //     dx = abs(dx)
    //     dy = abs(dy)
    //
    //     if dx > dy:
    //         xx, xy, yx, yy = xsign, 0, 0, ysign
    //     else:
    //         dx, dy = dy, dx
    //         xx, xy, yx, yy = 0, ysign, xsign, 0
    //
    //     D = 2 * dy - dx
    //     y = 0
    //
    //     for x in range(dx + 1):
    //         yield x0 + x * xx + y * yx, y0 + x * xy + y * yy
    //         if D >= 0:
    //             y += 1
    //             D -= 2 * dx
    //         D += 2 * dy
    fn clamp(&mut self) {
        self.loc.x = if self.loc.x + self.win.x > CELL.x as i32 {
            CELL.x - self.win.x
        } else {
            self.loc.x
        };

        self.loc.y = if self.loc.y + self.win.y > CELL.y as i32 {
            CELL.y - self.win.y
        } else {
            self.loc.y
        };
    }

    fn next_gen(&mut self) {
        self.cells.next_gen();
    }

    fn pause(&mut self) {
        self.is_paused = false;
    }

    fn unpause(&mut self) {
        self.is_paused = true;
    }

    fn quit(&mut self) {
        self.is_running = false;
    }

    fn speed_up(&mut self) {
        use std::time::Duration;
        if self.speed > Duration::from_millis(10) { self.speed -= Duration::from_millis(10); }
    }

    fn speed_down(&mut self) {
        use std::time::Duration;
        if self.speed < Duration::from_millis(400) { self.speed += Duration::from_millis(10) }
    }

    fn spawn_random(&mut self) {
        self.cells = Grid::from_fn(CELL.x as usize, CELL.y as usize, rand::random);
    }

    fn clear_board(&mut self) {
        self.cells = Grid::from_fn(CELL.x as usize, CELL.y as usize, || false);
    }

    fn screen_up(&mut self) {
        self.loc.y = if self.loc.y != 0 { self.loc.y - 1 } else { self.loc.y } 
    }

    fn screen_down(&mut self) {
        self.loc.y = if self.loc.y + self.win.x < CELL.y as i32 { self.loc.y + 1 } else { self.loc.y };
    }

    fn screen_left(&mut self) {
        self.loc.x = if self.loc.x != 0 { self.loc.x - 1 } else { self.loc.x };
    }

    fn screen_right(&mut self) {
        self.loc.x = if self.loc.x + self.win.x < CELL.x as i32 { self.loc.x + 1 } else { self.loc.x };
    }

    fn swap_block(&mut self) {
        self.block = if self.block { false } else { true };
    }
}
