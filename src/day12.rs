use std::num::ParseIntError;
use std::ops::{Add, AddAssign, SubAssign};
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl AddAssign<i32> for Direction {
    fn add_assign(&mut self, rhs: i32) {
        let curr = match self {
            Direction::North => 0,
            Direction::East => 90,
            Direction::South => 180,
            Direction::West => 270,
        };
        *self = match (rhs + curr + 360) % 360 {
            0 => Direction::North,
            90 => Direction::East,
            180 => Direction::South,
            270 => Direction::West,
            _ => panic!(format!(
                "{} is not 0 mod 90 on input self: {:?}, rhs: {}",
                (rhs + curr + 360) % 360,
                self,
                rhs
            )),
        };
    }
}

impl SubAssign<i32> for Direction {
    fn sub_assign(&mut self, rhs: i32) {
        *self += -rhs
    }
}

#[derive(Debug, Copy, Clone)]
enum Action {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

impl FromStr for Action {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_at(1) {
            ("N", i) => Ok(Action::North(i.parse()?)),
            ("S", i) => Ok(Action::South(i.parse()?)),
            ("E", i) => Ok(Action::East(i.parse()?)),
            ("W", i) => Ok(Action::West(i.parse()?)),
            ("L", i) => Ok(Action::Left(i.parse()?)),
            ("R", i) => Ok(Action::Right(i.parse()?)),
            ("F", i) => Ok(Action::Forward(i.parse()?)),
            _ => panic!("heck"),
        }
    }
}

#[derive(Debug)]
struct Ship {
    position: (i32, i32),
    direction: Direction,
    waypoint: (i32, i32),
}

impl Ship {
    fn manhattan_distance(&self) -> i32 {
        self.position.0.abs() + self.position.1.abs()
    }
}

impl Add<Action> for Ship {
    type Output = Ship;

    fn add(self, rhs: Action) -> Self::Output {
        let mut position = self.position;
        let mut direction = self.direction;
        let mut waypoint = self.waypoint;
        match rhs {
            Action::North(v) => {
                waypoint.0 += v;
            }
            Action::South(v) => {
                waypoint.0 -= v;
            }
            Action::East(v) => {
                waypoint.1 += v;
            }
            Action::West(v) => {
                waypoint.1 -= v;
            }
            Action::Right(v) => waypoint = rotate(waypoint, v),
            Action::Left(v) => waypoint = rotate(waypoint, -v),
            Action::Forward(v) => {
                position = (position.0 + waypoint.0 * v, position.1 + waypoint.1 * v)
            }
        }
        //println!("due to {:?}, the ship moved from {:?} with bearing {:?} to {:?} with bearing {:?} and moved the waypoint from {:?} to {:?}", rhs, self.position, self.direction, position, direction, self.waypoint, waypoint);
        Ship {
            position,
            direction,
            waypoint,
        }
    }
}

fn rotate(waypoint: (i32, i32), degrees: i32) -> (i32, i32) {
    match (degrees + 360) % 360 {
        0 => (waypoint.0, waypoint.1),
        90 => (-waypoint.1, waypoint.0),
        180 => (-waypoint.0, -waypoint.1),
        270 => (waypoint.1, -waypoint.0),
        _ => panic!("heck"),
    }
}

#[cfg(test)]
mod tests {
    use crate::day12::{Action, Direction, Ship};
    use crate::loader::file_to_vec;

    #[test]
    fn test_parse() {
        file_to_vec::<Action>("inputs/day12small.txt");
    }

    #[test]
    fn test_give_ship_direction() {
        Ship {
            position: (0, 0),
            direction: Direction::North,
            waypoint: (1, 10),
        } + Action::Forward(10);
    }

    #[test]
    fn test_small_instr() {
        let ship = Ship {
            position: (0, 0),
            direction: Direction::East,
            waypoint: (1, 10),
        };
        assert_eq!(
            file_to_vec::<Action>("inputs/day12small.txt")
                .iter()
                .fold(ship, |acc, x| { acc + *x })
                .manhattan_distance(),
            286
        );
    }

    #[test]
    fn test_instr() {
        let ship = Ship {
            position: (0, 0),
            direction: Direction::East,
            waypoint: (1, 10),
        };
        assert_eq!(
            file_to_vec::<Action>("inputs/day12.txt")
                .iter()
                .fold(ship, |acc, x| { acc + *x })
                .manhattan_distance(),
            319
        );
    }
}
