use crate::error::{AocError, Result};
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Dir {
    North = 0,
    South = 180,
    East = 90,
    West = 270,
}

impl Dir {
    pub fn from_heading(heading: i64) -> Result<Self> {
        match (heading + 360) % 360 {
            0 => Ok(Dir::North),
            180 => Ok(Dir::South),
            90 => Ok(Dir::East),
            270 => Ok(Dir::West),
            _ => Err(AocError::InvalidInput(format!(
                "Unsupported heading {}",
                heading
            ))),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Action {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

impl Action {
    pub fn from_char(c: char) -> Result<Self> {
        match c {
            'N' => Ok(Action::North),
            'S' => Ok(Action::South),
            'E' => Ok(Action::East),
            'W' => Ok(Action::West),
            'L' => Ok(Action::Left),
            'R' => Ok(Action::Right),
            'F' => Ok(Action::Forward),
            _ => Err(AocError::InvalidInput(format!(
                "Invalid action type: {}",
                c
            ))),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Instruction {
    action: Action,
    value: i64,
}

impl Instruction {
    pub fn new(action: Action, value: i64) -> Self {
        Instruction { action, value }
    }
}

impl FromStr for Instruction {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self> {
        let mut chars = s.chars();

        if let Some(ch) = chars.next() {
            return Ok(Self::new(
                Action::from_char(ch)?,
                chars.collect::<String>().parse::<i64>()?,
            ));
        }

        return Err(AocError::InvalidInput("Empty input".to_string()));
    }
}

pub trait Moveable {
    fn perform(&mut self, instruction: &Instruction) -> Result<()>;
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Ship {
    x: i64,
    y: i64,
    dir: Dir,
}

impl Ship {
    pub fn new() -> Self {
        Ship {
            x: 0,
            y: 0,
            dir: Dir::East,
        }
    }

    pub fn manhattan_distance(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }
}

impl Moveable for Ship {
    fn perform(&mut self, instruction: &Instruction) -> Result<()> {
        match instruction.action {
            Action::North => self.y += instruction.value,
            Action::South => self.y -= instruction.value,
            Action::East => self.x += instruction.value,
            Action::West => self.x -= instruction.value,
            Action::Left => self.dir = Dir::from_heading(self.dir as i64 - instruction.value)?,
            Action::Right => self.dir = Dir::from_heading(self.dir as i64 + instruction.value)?,
            Action::Forward => match self.dir {
                Dir::North => self.y += instruction.value,
                Dir::South => self.y -= instruction.value,
                Dir::East => self.x += instruction.value,
                Dir::West => self.x -= instruction.value,
            },
        };
        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Waypoint(Ship);

impl Waypoint {
    pub fn rotate_left(&mut self, degrees: i64) -> Result<()> {
        if degrees % 90 != 0 {
            return Err(AocError::InvalidInput(format!(
                "Cannot perform roation of '{}'",
                degrees
            )));
        }

        if degrees == 0 {
            return Ok(());
        }

        let y = self.0.y;
        self.0.y = self.0.x;
        self.0.x = -y;

        return self.rotate_left((degrees - 90) % 360);
    }

    pub fn rotate_right(&mut self, degrees: i64) -> Result<()> {
        if degrees % 90 != 0 {
            return Err(AocError::InvalidInput(format!(
                "Cannot perform roation of '{}'",
                degrees
            )));
        }

        if degrees <= 0 {
            return Ok(());
        }

        let y = self.0.y;
        self.0.y = -self.0.x;
        self.0.x = y;

        return self.rotate_right((degrees - 90) % 360);
    }
}

impl Moveable for Waypoint {
    fn perform(&mut self, instruction: &Instruction) -> Result<()> {
        self.0.perform(instruction)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct WaypointShip {
    x: i64,
    y: i64,
    waypoint: Waypoint,
}

impl WaypointShip {
    pub fn new() -> Self {
        WaypointShip {
            x: 0,
            y: 0,
            waypoint: Waypoint(Ship {
                x: 10,
                y: 1,
                dir: Dir::East,
            }),
        }
    }

    pub fn manhattan_distance(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }
}

impl Moveable for WaypointShip {
    fn perform(&mut self, instruction: &Instruction) -> Result<()> {
        match instruction.action {
            Action::North | Action::South | Action::East | Action::West => {
                self.waypoint.perform(instruction)?
            }
            Action::Left => self.waypoint.rotate_left(instruction.value)?,
            Action::Right => self.waypoint.rotate_right(instruction.value)?,
            Action::Forward => {
                for _ in 0..instruction.value {
                    self.x += self.waypoint.0.x;
                    self.y += self.waypoint.0.y;
                }
            }
        };
        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Plan {
    instructions: Vec<Instruction>,
}

impl Plan {
    pub fn from_input(input: &[String]) -> Result<Self> {
        Ok(Plan {
            instructions: input
                .iter()
                .map(|line| Instruction::from_str(line))
                .collect::<Result<Vec<Instruction>>>()?,
        })
    }

    pub fn len(&self) -> usize {
        self.instructions.len()
    }

    pub fn execute<T: Moveable>(&self, ship: &mut T) -> Result<()> {
        self.instructions
            .iter()
            .map(|ins| ship.perform(ins))
            .collect::<Result<Vec<()>>>()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::test_input;

    mod dir {
        use super::*;

        #[test]
        fn from_heading() {
            assert_eq!(Dir::from_heading(0).unwrap(), Dir::North);
            assert_eq!(Dir::from_heading(360).unwrap(), Dir::North);
            assert_eq!(Dir::from_heading(90).unwrap(), Dir::East);
            assert_eq!(Dir::from_heading(180).unwrap(), Dir::South);
            assert_eq!(Dir::from_heading(270).unwrap(), Dir::West);
            assert_eq!(Dir::from_heading(-90).unwrap(), Dir::West);
            assert!(Dir::from_heading(-91).is_err());
        }
    }

    mod action {
        use super::*;

        #[test]
        fn from_str() {
            assert_eq!(Action::from_char('N').unwrap(), Action::North);
            assert_eq!(Action::from_char('S').unwrap(), Action::South);
            assert_eq!(Action::from_char('E').unwrap(), Action::East);
            assert_eq!(Action::from_char('W').unwrap(), Action::West);
            assert_eq!(Action::from_char('L').unwrap(), Action::Left);
            assert_eq!(Action::from_char('R').unwrap(), Action::Right);
            assert_eq!(Action::from_char('F').unwrap(), Action::Forward);

            assert!(Action::from_char('n').is_err());
            assert!(Action::from_char('B').is_err());
        }
    }

    mod instruction {
        use super::*;

        #[test]
        fn parsing() {
            assert_eq!(
                Instruction::from_str("F10").unwrap(),
                Instruction::new(Action::Forward, 10)
            );
            assert_eq!(
                Instruction::from_str("R270").unwrap(),
                Instruction::new(Action::Right, 270)
            );
            assert_eq!(
                Instruction::from_str("N5").unwrap(),
                Instruction::new(Action::North, 5)
            );
            assert!(Instruction::from_str("B10").is_err());
            assert!(Instruction::from_str("10").is_err());
            assert!(Instruction::from_str("N").is_err());
        }
    }

    mod ship {
        use super::*;

        #[test]
        fn manhattan_distance() {
            let mut s = Ship::new();

            assert_eq!(s.manhattan_distance(), 0);

            s.x += 5;
            assert_eq!(s.manhattan_distance(), 5);

            s.y -= 10;
            assert_eq!(s.manhattan_distance(), 15);
        }

        #[test]
        fn perform() {
            let mut s = Ship::new();
            let ins = Instruction::from_str("F10").unwrap();

            s.perform(&ins).unwrap();
            assert_eq!(
                s,
                Ship {
                    x: 10,
                    y: 0,
                    dir: Dir::East
                }
            );

            let ins = Instruction::from_str("L90").unwrap();

            s.perform(&ins).unwrap();
            assert_eq!(
                s,
                Ship {
                    x: 10,
                    y: 0,
                    dir: Dir::North
                }
            );

            let ins = Instruction::from_str("F90").unwrap();

            s.perform(&ins).unwrap();
            assert_eq!(
                s,
                Ship {
                    x: 10,
                    y: 90,
                    dir: Dir::North
                }
            );

            let ins = Instruction::from_str("W20").unwrap();

            s.perform(&ins).unwrap();
            assert_eq!(
                s,
                Ship {
                    x: -10,
                    y: 90,
                    dir: Dir::North
                }
            );
        }
    }

    mod waypoint {
        use super::*;

        #[test]
        fn rotation() {
            let mut waypoint = Waypoint(Ship {
                x: 5,
                y: 7,
                dir: Dir::East,
            });

            waypoint.rotate_right(90).unwrap();
            assert_eq!(
                waypoint,
                Waypoint(Ship {
                    x: 7,
                    y: -5,
                    dir: Dir::East
                })
            );

            waypoint.rotate_right(90).unwrap();
            assert_eq!(
                waypoint,
                Waypoint(Ship {
                    x: -5,
                    y: -7,
                    dir: Dir::East
                })
            );

            waypoint.rotate_right(90).unwrap();
            assert_eq!(
                waypoint,
                Waypoint(Ship {
                    x: -7,
                    y: 5,
                    dir: Dir::East
                })
            );

            waypoint.rotate_right(90).unwrap();
            assert_eq!(
                waypoint,
                Waypoint(Ship {
                    x: 5,
                    y: 7,
                    dir: Dir::East
                })
            );

            waypoint.rotate_left(90).unwrap();
            assert_eq!(
                waypoint,
                Waypoint(Ship {
                    x: -7,
                    y: 5,
                    dir: Dir::East
                })
            );

            waypoint.rotate_left(90).unwrap();
            assert_eq!(
                waypoint,
                Waypoint(Ship {
                    x: -5,
                    y: -7,
                    dir: Dir::East
                })
            );

            waypoint.rotate_left(90).unwrap();
            assert_eq!(
                waypoint,
                Waypoint(Ship {
                    x: 7,
                    y: -5,
                    dir: Dir::East
                })
            );

            waypoint.rotate_left(90).unwrap();
            assert_eq!(
                waypoint,
                Waypoint(Ship {
                    x: 5,
                    y: 7,
                    dir: Dir::East
                })
            );

            waypoint.rotate_left(360).unwrap();
            assert_eq!(
                waypoint,
                Waypoint(Ship {
                    x: 5,
                    y: 7,
                    dir: Dir::East
                })
            );

            waypoint.rotate_left(180).unwrap();
            assert_eq!(
                waypoint,
                Waypoint(Ship {
                    x: -5,
                    y: -7,
                    dir: Dir::East
                })
            );
        }
    }

    mod waypoint_ship {
        use super::*;

        #[test]
        fn manhattan_distance() {
            let mut s = WaypointShip::new();

            assert_eq!(s.manhattan_distance(), 0);

            s.x += 5;
            assert_eq!(s.manhattan_distance(), 5);

            s.y -= 10;
            assert_eq!(s.manhattan_distance(), 15);
        }

        #[test]
        fn perform() {
            let mut s = WaypointShip::new();
            let ins = Instruction::from_str("F10").unwrap();

            s.perform(&ins).unwrap();
            assert_eq!(
                s,
                WaypointShip {
                    x: 100,
                    y: 10,
                    waypoint: Waypoint(Ship {
                        x: 10,
                        y: 1,
                        dir: Dir::East
                    })
                }
            );

            let ins = Instruction::from_str("N2").unwrap();

            s.perform(&ins).unwrap();
            assert_eq!(
                s,
                WaypointShip {
                    x: 100,
                    y: 10,
                    waypoint: Waypoint(Ship {
                        x: 10,
                        y: 3,
                        dir: Dir::East
                    })
                }
            );

            let ins = Instruction::from_str("W2").unwrap();

            s.perform(&ins).unwrap();
            assert_eq!(
                s,
                WaypointShip {
                    x: 100,
                    y: 10,
                    waypoint: Waypoint(Ship {
                        x: 8,
                        y: 3,
                        dir: Dir::East
                    })
                }
            );

            let ins = Instruction::from_str("R180").unwrap();

            s.perform(&ins).unwrap();
            assert_eq!(
                s,
                WaypointShip {
                    x: 100,
                    y: 10,
                    waypoint: Waypoint(Ship {
                        x: -8,
                        y: -3,
                        dir: Dir::East
                    })
                }
            );
        }
    }

    mod plan {
        use super::*;

        #[test]
        fn from_input() {
            let input = test_input(
                "
                F10
                N3
                F7
                R90
                F11",
            );

            let plan = Plan::from_input(&input).unwrap();

            assert_eq!(plan.len(), 5);
        }

        #[test]
        fn execute() {
            let input = test_input(
                "
                F10
                N3
                F7
                R90
                F11",
            );

            let mut s = Ship::new();
            let plan = Plan::from_input(&input).unwrap();

            plan.execute(&mut s).unwrap();

            assert_eq!(s.manhattan_distance(), 25);
        }
    }
}
