use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

use colored_text::Colorize;
use rand::{random_bool, random_range};

#[derive(PartialEq, Eq)]
enum Mode {
    Sand,
    Water,
}

const MODE: Mode = Mode::Sand;

struct State {
    lines: Vec<Vec<u8>>,
    streams: Vec<usize>,
    width: usize,
    windows: Vec<Window>,
    scaling_x: f32,
    scaling_y: f32,
    monitor: String,
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
            streams.push(random_range(0..width));
        }
        Self {
            lines: vec![vec![32; width]; height],
            streams,
            width,
            windows: vec![],
            scaling_x,
            scaling_y,
            monitor: get_active().1,
        }
    }

    fn print(&self) -> String {
        let mut res = String::new();
        let mut period = random_range(4..=10);
        let mut active = false;
        for line in &self.lines {
            period -= 1;
            if period == 0 {
                active = !active;
                period = random_range(4..=10);
            }
            if active && MODE == Mode::Water {
                res += String::from_utf8(line.clone())
                    .unwrap()
                    .bright_blue()
                    .as_str();
            } else {
                if MODE == Mode::Water {
                    res += String::from_utf8(line.clone()).unwrap().blue().as_str();
                } else {
                    res += String::from_utf8(line.clone()).unwrap().yellow().as_str();
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
        self.streams.push(random_range(0..self.width));
        for i in (0..self.lines.len()).rev() {
            if i <= 3 {
                self.lines[3] = vec![32; self.width];
                for j in 0..self.streams.len() {
                    let pos = self.streams[j];
                    self.lines[3][pos] = random_char();
                }
            } else {
                let mut new_line = vec![32; self.width];
                let mut locked = vec![false; self.width];
                for j in 0..self.width {
                    let mut found_col = false;
                    for window in &self.windows {
                        if window.check_col(j, i, self.scaling_x, self.scaling_y) {
                            found_col = true;
                            break;
                        }
                    }
                    if found_col {
                        if !locked[j] {
                            new_line[j] = 32;
                        }
                        continue;
                    }
                    for window in &self.windows {
                        if window.check_col(j, i + 1, self.scaling_x, self.scaling_y) {
                            found_col = true;
                            break;
                        }
                    }
                    if found_col && self.lines[i][j] != 32 && MODE == Mode::Water {
                        let idx = if self.lines[i][j] << 7 >> 7 == 0 {
                            j as i32 - 1
                        } else {
                            j as i32 + 1
                        };
                        if idx < 0 || idx >= self.width as i32 {
                            if !locked[j] {
                                new_line[j] = 32;
                            }
                            continue;
                        }
                        if !locked[idx as usize] {
                            new_line[idx as usize] = self.lines[i][j];
                            locked[idx as usize] = true;
                        }
                    } else {
                        if !locked[j] {
                            let free = (self.lines[i][j] == 32
                                || i == self.lines.len() - 1
                                || (self.lines[i + 1][j] == 32 && {
                                    let mut res = true;
                                    for window in &self.windows {
                                        if window.check_col(
                                            j,
                                            i + 1,
                                            self.scaling_x,
                                            self.scaling_y,
                                        ) {
                                            res = false;
                                        }
                                    }
                                    res
                                }))
                                && MODE == Mode::Sand;
                            if self.lines[i - 1][j] != 32
                                && if MODE == Mode::Water { true } else { free }
                            {
                                new_line[j] = self.lines[i - 1][j];
                                locked[j] = true;
                                self.lines[i - 1][j] = 32;
                            } else if j > 0
                                && self.lines[i - 1][j - 1] != 32
                                && self.lines[i][j - 1] != 32
                                && free
                            {
                                new_line[j] = self.lines[i - 1][j - 1];
                                locked[j] = true;
                                self.lines[i - 1][j - 1] = 32;
                            } else if j < self.width - 1
                                && self.lines[i - 1][j + 1] != 32
                                && self.lines[i][j + 1] != 32
                                && free
                            {
                                new_line[j] = self.lines[i - 1][j + 1];
                                locked[j] = true;
                                self.lines[i - 1][j + 1] = 32;
                            } else if i < self.lines.len() - 1 {
                                new_line[j] = self.lines[i][j];
                            } else {
                                new_line[j] = 32;
                            }
                        }
                    }
                }
                self.lines[i] = new_line;
            }
        }
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
        let x_scaled = if self.x as f32 > scaling_x {
            self.x as f32 / scaling_x - 1.0
        } else {
            0.0
        };
        let y_scaled = self.y as f32 / scaling_y;
        let width = self.width as f32 / scaling_x + 1.0;
        let height = self.height as f32 / scaling_y;
        if x as f32 <= x_scaled || x as f32 >= x_scaled + width {
            return false;
        }
        if y as f32 <= y_scaled || y as f32 >= y_scaled + height {
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
        sleep(Duration::from_millis(30));
    }
}
