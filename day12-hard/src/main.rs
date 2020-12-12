use std::io::{self, BufRead};
use std::error::Error;
use std::str::FromStr;

enum Command {
    East(i64),
    West(i64),
    North(i64),
    South(i64),
    Left(i64),
    Right(i64),
    Forward(i64),
}

impl FromStr for Command {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let first_char = match s.chars().next() {
            Some(ch) => ch,
            _ => {
                return Err(Box::from("string is empty"));
            }
        };

        let remainder: String = s.chars().skip(1).collect();
        let value: i64 = remainder.parse()?;

        Ok(match first_char {
            'E' => Self::East(value),
            'W' => Self::West(value),
            'N' => Self::North(value),
            'S' => Self::South(value),
            'L' => Self::Left(value),
            'R' => Self::Right(value),
            'F' => Self::Forward(value),
            _ => {
                return Err(Box::from("unsupported first character"));
            }
        })
    }
}

struct State {
    ship_offset_north: i64,
    ship_offset_east: i64,
    waypoint_ship_offset_north: i64,
    waypoint_ship_offset_east: i64,
}

impl State {
    fn get_angle_cos(angle: i64) -> i64 {
        match angle {
            0 => 1,
            90 => 0,
            180 => -1,
            270 => 0,
            _ => panic!("Unsupported angle"),
        }
    }

    fn get_angle_sin(angle: i64) -> i64 {
        match angle {
            0 => 0,
            90 => 1,
            180 => 0,
            270 => -1,
            _ => panic!("Unsupported angle"),
        }
    }

    pub fn new() -> Self {
        Self {
            ship_offset_north: 0,
            ship_offset_east: 0,
            waypoint_ship_offset_north: 1,
            waypoint_ship_offset_east: 10,
        }
    }

    pub fn apply_command(&self, command: Command) -> Self {
        match command {
            Command::East(distance) => Self {
                waypoint_ship_offset_east: self.waypoint_ship_offset_east + distance,
                ..*self
            },
            Command::West(distance) => Self {
                waypoint_ship_offset_east: self.waypoint_ship_offset_east - distance,
                ..*self
            },
            Command::North(distance) => Self {
                waypoint_ship_offset_north: self.waypoint_ship_offset_north + distance,
                ..*self
            },
            Command::South(distance) => Self {
                waypoint_ship_offset_north: self.waypoint_ship_offset_north - distance,
                ..*self
            },
            Command::Left(angle) => Self {
                waypoint_ship_offset_east: self.waypoint_ship_offset_east * Self::get_angle_cos(angle) - self.waypoint_ship_offset_north * Self::get_angle_sin(angle),
                waypoint_ship_offset_north: self.waypoint_ship_offset_east * Self::get_angle_sin(angle) + self.waypoint_ship_offset_north * Self::get_angle_cos(angle),
                ..*self
            },
            Command::Right(angle) => Self {
                waypoint_ship_offset_east: self.waypoint_ship_offset_east * Self::get_angle_cos(angle) + self.waypoint_ship_offset_north * Self::get_angle_sin(angle),
                waypoint_ship_offset_north: -self.waypoint_ship_offset_east * Self::get_angle_sin(angle) + self.waypoint_ship_offset_north * Self::get_angle_cos(angle),
                ..*self
            },
            Command::Forward(times) => Self {
                ship_offset_east: self.ship_offset_east + self.waypoint_ship_offset_east * times,
                ship_offset_north: self.ship_offset_north + self.waypoint_ship_offset_north * times,
                ..*self
            },
        }
    }

    pub fn get_manhattan_norm(&self) -> i64 {
        self.ship_offset_east.abs() + self.ship_offset_north.abs()
    }
}

fn main() {
    let stdin = io::stdin();
    let mut state = State::new();
    for line in stdin.lock().lines() {
        state = state.apply_command(line.unwrap().parse().unwrap());
    }
    
    println!("{}", state.get_manhattan_norm());
}
