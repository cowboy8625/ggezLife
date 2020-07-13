use crate::{CELL, WINDOW, LIMITS, STEP, CellPoint};
use crate::grid::Grid;
use crate::game_event::GameEvent;
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Point(i32, i32);

impl Point {
    fn unwrap(&self) -> (i32, i32) {
        (self.0, self.1)
    }
}

impl From<(f32, f32)> for Point {
    fn from(pt: (f32, f32)) -> Self {
        Self(pt.0 as i32, pt.1 as i32)
    }
}

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
    points: HashSet<Point>,
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
            block: true, is_paused: true, is_running: true, cells, screen, time: std::time::Instant::now() + speed, points: HashSet::new(), } } pub fn make_screen(&mut self) {
        self.clamp();
        self.screen = self.cells.sub_grid(self.loc.x as usize, self.loc.y as usize, self.win.x as usize,self. win.y as usize);
    }

    pub fn update(&mut self) {
        if self.points.len() > 1 {
            let mut pt1: Option<&Point> = None;
            for pt0 in self.points.iter() {
                if pt1.is_some() {
                    for (x, y) in Self::gen_points(pt0, pt1.unwrap()) {
                        // self.place_cell(x, y);
                        // let screen_x: usize = (x / self.grid) as usize;
                        // let screen_y: usize = (y / self.grid) as usize;
                        // let (world_x, world_y) = (self.loc.x as usize + screen_x, self.loc.y as usize + screen_y);
                        // if world_x < CELL.x as usize && world_y < CELL.y as usize {
                        //     self.cells[world_y][world_x] = self.block;
                        // }
                    }
                }
                pt1 = Some(&pt0);
            }
            self.points.clear();
        }

        if self.is_paused && self.time < std::time::Instant::now(){
            self.next_gen();
            self.time = std::time::Instant::now() + self.speed;
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
        if world_x < CELL.x as usize && world_y < CELL.y as usize {
            self.cells[world_y][world_x] = self.block;
        }
    }


    pub fn add_point(&mut self, point: (f32, f32)) {
        self.points.insert(point.into());
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
    fn gen_points(pt0: &Point, pt1: &Point) -> Vec<(i32, i32)> {
        let (x0, y0) = pt0.unwrap();
        let (x1, y1) = pt1.unwrap();
        let dx = x1 as i32 - x0 as i32;
        let dy = y1 as i32 - y0 as i32;

        let xsign = if dx > 0 { 1 } else { -1 };
        let ysign = if dy > 0 { 1 } else { -1 };
 
        let mut dx = dx.abs();
        let mut dy = dy.abs();

        let xx: i32;
        let xy: i32;
        let yx: i32;
        let yy: i32;
        if dx > dy {
            xx = xsign; //, xy, yx, yy) = (xsign, 0, 0, ysign);
            xy = 0;
            yx = 0;
            yy = ysign;
        } else {
            //(dx, dy) = (dy, dx);
            dx = dy; 
            dy = dx;
            // (xx, xy, yx, yy) = (0, ysign, xsign, 0);
            xx = 0;
            xy = ysign;
            yx = xsign;
            yy = 0;
        }
        let mut d = 2 * dy - dx;
        let mut y = 0;

        let mut points = Vec::new();
        for x in 0..dx + 1 {
             points.push(
                 (
                     (x0 as i32 + x * xx + y * yx),
                     (y0 as i32 + x * xy + y * yy)
                 )
             );
             if d >= 0 {
                y += 1;
                d -= 2 * dx;
             }
             d += 2 * dy;
        }
        points
    }

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
        if self.speed < Duration::from_millis(4000) { self.speed += Duration::from_millis(10) }
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
