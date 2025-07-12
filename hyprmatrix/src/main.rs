mod kernel;

use std::thread::sleep;
use std::time::Duration;
use std::{env, process::Command};

use colored_text::Colorize;
use kernel::kernel;
use rand::distr::{Distribution, StandardUniform};
use rand::{random, random_bool, random_range, Rng};

#[derive(Clone, Copy, PartialEq, Eq)]
enum TileType {
    Empty,
    Col,
    Sand,
    Water,
    WetSand,
}

impl Distribution<TileType> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TileType {
        match rng.random_range(0..=1) {
            0 => TileType::Sand,
            _ => TileType::Water,
        }
    }
}

#[derive(Clone, Copy)]
struct Tile {
    tile_type: TileType,
    value: u8,
}

impl Tile {
    fn new_empty() -> Self {
        Self {
            tile_type: TileType::Empty,
            value: 0,
        }
    }

    fn new_filled(tile_type: TileType) -> Self {
        Self {
            tile_type,
            value: random_char(),
        }
    }

    fn falls(&self) -> bool {
        match self.tile_type {
            TileType::Sand | TileType::Water | TileType::WetSand => true,
            TileType::Empty | TileType::Col => false,
        }
    }

    fn slides(&self) -> bool {
        match self.tile_type {
            TileType::Sand | TileType::Water => true,
            TileType::WetSand | TileType::Empty | TileType::Col => false,
        }
    }

    fn runs(&self) -> bool {
        match self.tile_type {
            TileType::Water => true,
            TileType::Sand | TileType::WetSand | TileType::Empty | TileType::Col => false,
        }
    }

    fn density(&self) -> f64 {
        match self.tile_type {
            TileType::Water => 0.2,
            TileType::Sand => 0.5,
            TileType::WetSand => 0.7,
            _ => 0.0,
        }
    }

    fn empty(&self) -> bool {
        self.tile_type == TileType::Empty
    }

    fn real(&self) -> bool {
        match self.tile_type {
            TileType::Empty | TileType::Col => false,
            _ => true,
        }
    }
}

#[derive(Clone, Copy)]
struct Stream {
    pos: usize,
    tile_type: TileType,
}

impl Stream {
    fn new(width: usize, frame: u32) -> Self {
        let tile_type = if frame % 300 < 175 {
            TileType::Sand
        } else if frame < 225 {
            random()
        } else {
            TileType::Water
        };
        Self {
            pos: random_range(0..width),
            tile_type,
        }
    }
}

struct State {
    lines: Vec<Vec<Tile>>,
    streams: Vec<Stream>,
    width: usize,
    height: usize,
    windows: Vec<Window>,
    scaling_x: f32,
    scaling_y: f32,
    monitor: String,
    frame: u32,
}

impl State {
    fn new(
        width: usize,
        height: usize,
        num_streams: usize,
        scaling_x: f32,
        scaling_y: f32,
    ) -> Self {
        let mut streams = vec![];
        for _ in 0..num_streams {
            streams.push(Stream::new(width, 0));
        }
        let args = env::args().collect::<Vec<String>>();
        println!("{:?}", args);
        Self {
            lines: vec![vec![Tile::new_empty(); width]; height],
            streams,
            width,
            height,
            windows: vec![],
            scaling_x,
            scaling_y,
            monitor: get_active().1,
            frame: 0,
        }
    }

    fn print(&self) -> String {
        let mut res = String::new();
        for line in &self.lines {
            for tile in line {
                match tile.tile_type {
                    TileType::Sand => {
                        res += (tile.value as char).yellow().as_str();
                    }
                    TileType::Water => {
                        res += (tile.value as char).blue().as_str();
                    }
                    TileType::WetSand => {
                        res += (tile.value as char).hex("#987c2f").as_str();
                    }
                    TileType::Empty | TileType::Col => {
                        res += " ";
                    }
                }
            }
            res += "\n";
        }
        res
    }

    fn step(&mut self) {
        match get_windows(self.monitor.clone()) {
            Some(x) => self.windows = x,
            None => {}
        };
        self.streams.remove(random_range(0..self.streams.len()));
        self.streams.push(Stream::new(self.width, self.frame));
        for i in 1..self.height {
            for j in 0..self.width {
                let mut found_col = false;
                for window in &self.windows {
                    found_col |= window.check_col(j, i - 1, self.scaling_x, self.scaling_y);
                }
                if found_col {
                    self.lines[i][j].tile_type = TileType::Col;
                    continue;
                } else if self.lines[i][j].tile_type == TileType::Col {
                    self.lines[i][j].tile_type = TileType::Empty;
                }
            }
        }
        for i in (1..self.height).rev() {
            if i <= 3 {
                self.lines[3] = vec![Tile::new_empty(); self.width];
                for j in 0..self.streams.len() {
                    let stream = self.streams[j];
                    self.lines[3][stream.pos] = Tile::new_filled(stream.tile_type);
                }
            } else {
                for j in 0..self.width - 1 {
                    let ll = self.lines[i][j];
                    let lr = self.lines[i][j + 1];
                    let ul = self.lines[i - 1][j];
                    let ur = self.lines[i - 1][j + 1];
                    let (ll, lr, ul, ur) = kernel(ll, lr, ul, ur);
                    self.lines[i][j] = ll;
                    self.lines[i][j + 1] = lr;
                    self.lines[i - 1][j] = ul;
                    self.lines[i - 1][j + 1] = ur;
                }
            }
        }
        self.frame += 1;
    }
}

fn random_char() -> u8 {
    random_range(33..127)
}

#[derive(Clone)]
struct Window {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Window {
    fn check_col(&self, x: usize, y: usize, scaling_x: f32, scaling_y: f32) -> bool {
        let x_scaled = self.x as f32 / scaling_x;
        let y_scaled = self.y as f32 / scaling_y;
        let width = self.width as f32 / scaling_x;
        let height = self.height as f32 / scaling_y;
        if (x as f32) < x_scaled || (x as f32) > x_scaled + width {
            return false;
        }
        if (y as f32) < y_scaled || (y as f32) > y_scaled + height {
            return false;
        }
        return true;
    }
}

fn get_windows(monitor: String) -> Option<Vec<Window>> {
    let output = Command::new("hyprctl")
        .arg("clients")
        .output()
        .expect("Failed to get hyprland window data!");
    let output_string = String::from_utf8(output.stdout).expect("Output was invalid UTF-8!");
    let mut windows = vec![];
    let mut window = Window {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    let mut skip_window = true;
    let (workspace_string, current_monitor) = get_active();
    if current_monitor != monitor && !monitor.is_empty() {
        return None;
    }
    let workspace = workspace_string.as_str();

    for line in output_string.lines() {
        if line.len() == 0 {
            continue;
        }
        let mut trimmed = line.trim().chars();
        let first = trimmed.nth(0).unwrap();
        if first == 'W' {
            if skip_window {
                skip_window = false;
                continue;
            }
            windows.push(window.clone());
            continue;
        }
        if first == 'a' {
            let pos = line.trim().split(' ').collect::<Vec<&str>>()[1];
            let segs = pos.split(',').collect::<Vec<&str>>();
            let x = segs[0].parse().unwrap();
            let y = segs[1].parse().unwrap();
            window.x = x;
            window.y = y;
        }
        if first == 'h' {
            let val = line.trim().split(' ').collect::<Vec<&str>>()[1];
            if val == "1" {
                skip_window = true;
            }
        }
        if first == 'w' {
            let val = line.trim().split(' ').collect::<Vec<&str>>()[1];
            if val != workspace {
                skip_window = true;
            }
        }
        let second = trimmed.nth(0).unwrap();
        if first == 's' && second == 'i' {
            let pos = line.trim().split(' ').collect::<Vec<&str>>()[1];
            let segs = pos.split(',').collect::<Vec<&str>>();
            let width = segs[0].parse().unwrap();
            let height = segs[1].parse().unwrap();
            window.width = width;
            window.height = height;
        }
        if first == 'm' && second == 'o' && !skip_window {
            let monitor = line.trim().split(' ').collect::<Vec<&str>>()[1]
                .parse::<i32>()
                .unwrap();
            if monitor > 0 {
                window.x -= 1920;
                window.y += 10;
            }
        }
    }
    if !skip_window {
        windows.push(window.clone());
    }
    Some(windows)
}

fn get_active() -> (String, String) {
    let output = Command::new("hyprctl")
        .arg("activeworkspace")
        .output()
        .expect("Failed to get hyprland window data!");
    let output_string = String::from_utf8(output.stdout).expect("Output was invalid UTF-8!");
    let mut lines = output_string.lines();
    let id_line = lines.next().unwrap();
    let monitor_line = lines.next().unwrap().trim();
    let monitor = monitor_line.split(' ').collect::<Vec<&str>>()[1];
    let id = id_line.split(' ').nth(2).unwrap();
    (id.to_string(), monitor.to_string())
}

fn main() -> std::io::Result<()> {
    let (x, y) = term_size::dimensions().unwrap();
    let display = get_active().1;
    let x_dimen = 1920.0;
    let y_dimen = if display.as_str() == "0" {
        1200.0
    } else {
        1080.0
    };
    let mut state = State::new(x, y, 10, x_dimen / x as f32, y_dimen / y as f32);
    loop {
        state.step();
        println!("{}", state.print());
        sleep(Duration::from_millis(10));
    }
}
