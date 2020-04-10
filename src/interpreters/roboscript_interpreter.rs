//! write an interpreter for roboscript which accepts one argument `code`, the
//! program to be executed, and returns a string representation of the smallest
//! 2D grid containing the full path that the robot has walked on
//!
//! `code` commands:
//! -   `F` move the robot forward
//! -   `L` rotate the robot left
//! -   `R` rotate the robot right
//! -   any number of digits after any of the previous commands to repeat that
//!     command a certain number of times

use regex::Regex;

#[derive(Copy, Clone)]
enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

struct Robot<'a> {
    x: usize,
    y: usize,
    direction: Direction,
    path: Vec<Vec<&'a str>>,
}

impl<'a> Robot<'a> {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            direction: Direction::East,
            path: vec![vec!["*"]],
        }
    }

    fn forward(&mut self, count: i32) {
        for _ in 0..count {
            match self.direction {
                Direction::North => {
                    if self.y == 0 {
                        self.path.insert(0, vec![" "; self.path[0].len()]);
                    } else {
                        self.y -= 1;
                    }
                }
                Direction::East => {
                    if self.x == self.path[0].len() - 1 {
                        for row in &mut self.path {
                            row.push(" ");
                        }
                    }
                    self.x += 1;
                }
                Direction::South => {
                    if self.y == self.path.len() - 1 {
                        self.path.push(vec![" "; self.path[0].len()]);
                    }
                    self.y += 1;
                }
                Direction::West => {
                    if self.x == 0 {
                        for row in &mut self.path {
                            row.insert(0, " ");
                        }
                    } else {
                        self.x -= 1;
                    }
                }
            }

            self.path[self.y][self.x] = "*";
        }
    }

    fn rotate(&mut self, delta: i32) {
        self.direction = match (self.direction as i32 + delta) % 4 {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            _ => Direction::West,
        };
    }
}

#[allow(dead_code)]
pub fn execute(code: &str) -> String {
    let mut robot = Robot::new();

    for captures in Regex::new(r"(?P<instruction>[FLR])(?P<count>\d*)")
        .unwrap()
        .captures_iter(code)
    {
        let count = match &captures["count"] {
            "" => 1,
            n => n.parse::<i32>().unwrap(),
        };
        match &captures["instruction"] {
            "F" => robot.forward(count),
            "L" => robot.rotate(-count),
            _ => robot.rotate(count),
        }
    }

    robot
        .path
        .into_iter()
        .map(|row| row.join(""))
        .collect::<Vec<String>>()
        .join("\r\n")
}
