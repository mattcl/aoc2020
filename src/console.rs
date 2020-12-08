use crate::error::{AocError, Result};
use std::collections::HashSet;

#[derive(Eq, PartialEq, Debug)]
pub enum Op {
    acc,
    jmp,
    nop,
}

impl Op {
    pub fn from_str(input: &str) -> Result<Op> {
        match input {
            "acc" => Ok(Op::acc),
            "jmp" => Ok(Op::jmp),
            "nop" => Ok(Op::nop),
            _ => Err(AocError::UnknownOperation(input.to_string())),
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct Instruction {
    op: Op,
    val: i64,
}

impl Instruction {
    pub fn new(input: &str) -> Result<Self> {
        let mut parts = input.split(' ');

        if let Some(op_str) = parts.next() {
            if let Some(val_str) = parts.next() {
                if let Some(_) = parts.next() {
                    return Err(AocError::InvalidInstruction(input.to_string()));
                }

                return Ok(Instruction {
                    op: Op::from_str(op_str)?,
                    val: val_str.parse::<i64>()?,
                });
            }
        }

        Err(AocError::InvalidInstruction(input.to_string()))
    }
}

pub struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    pub fn new(input: &[String]) -> Result<Self> {
        Ok(Program {
            instructions: input
                .iter()
                .map(|line| Instruction::new(line))
                .collect::<Result<Vec<Instruction>>>()?,
        })
    }

    pub fn execute(&self) -> Result<(i64, bool)> {
        let mut seen: HashSet<i64> = HashSet::new();

        if self.instructions.is_empty() {
            return Err(AocError::InvalidProgram("No instructions".to_string()));
        }

        let mut ptr: i64 = 0;
        let mut accumulator = 0;
        let eof = self.instructions.len() as i64;
        loop {
            if let Some(cur) = self.instructions.get(ptr as usize) {
                seen.insert(ptr);
                ptr = match cur.op {
                    Op::acc => {
                        accumulator += cur.val;
                        ptr + 1
                    }
                    Op::jmp => ptr + cur.val,
                    Op::nop => ptr + 1,
                };

                if ptr == eof {
                    break;
                }

                if ptr < 0 || ptr > eof {
                    return Err(AocError::InvalidProgram(format!(
                        "Attempted to access instruction location out of bounds {} of {}",
                        ptr,
                        self.instructions.len()
                    )));
                }

                if seen.contains(&ptr) {
                    return Ok((accumulator, false));
                }
            } else {
                unreachable!("Should not be possible");
            }
        }

        Ok((accumulator, true))
    }

    pub fn correct(&mut self) -> Result<i64> {
        for i in 0..self.instructions.len() {
            if let Some(ins) = self.instructions.get_mut(i) {
                match ins.op {
                    Op::jmp => ins.op = Op::nop,
                    Op::nop => ins.op = Op::jmp,
                    _ => {}
                }
            }

            if let Ok((val, normal)) = self.execute() {
                if normal {
                    return Ok(val);
                }
            }

            if let Some(ins) = self.instructions.get_mut(i) {
                match ins.op {
                    Op::jmp => ins.op = Op::nop,
                    Op::nop => ins.op = Op::jmp,
                    _ => {}
                }
            }
        }

        Err(AocError::InvalidProgram("Could not be fixed".to_string()))
    }
}

#[cfg(test)]
mod tests {
    mod op {
        use super::super::*;

        #[test]
        fn from_str() {
            assert_eq!(Op::from_str("acc").unwrap(), Op::acc);
            assert_eq!(Op::from_str("jmp").unwrap(), Op::jmp);
            assert_eq!(Op::from_str("nop").unwrap(), Op::nop);
            assert!(Op::from_str("foo").is_err());
            assert!(Op::from_str("bar").is_err());
        }
    }

    mod instruction {
        use super::super::*;

        #[test]
        fn new() {
            assert!(Instruction::new("").is_err());
            assert!(Instruction::new("acc").is_err());
            assert!(Instruction::new("acc +1 2").is_err());
            assert!(Instruction::new("acc +a").is_err());
            assert!(Instruction::new("bar +1").is_err());

            let i = Instruction::new("acc +1").unwrap();
            assert_eq!(i.op, Op::acc);
            assert_eq!(i.val, 1);

            let i = Instruction::new("nop -1333").unwrap();
            assert_eq!(i.op, Op::nop);
            assert_eq!(i.val, -1333);
        }
    }

    mod program {
        use super::super::*;

        fn input() -> Vec<String> {
            vec![
                "nop +0".to_string(),
                "acc +1".to_string(),
                "jmp +4".to_string(),
                "acc +3".to_string(),
                "jmp -3".to_string(),
                "acc -99".to_string(),
                "acc +1".to_string(),
                "jmp -4".to_string(),
                "acc +6".to_string(),
            ]
        }

        fn correct_input() -> Vec<String> {
            vec![
                "nop +0".to_string(),
                "acc +1".to_string(),
                "jmp +4".to_string(),
                "acc +3".to_string(),
                "jmp -3".to_string(),
                "acc -99".to_string(),
                "acc +1".to_string(),
                "nop -4".to_string(),
                "acc +6".to_string(),
            ]
        }

        #[test]
        fn new() {
            let p = Program::new(&input()).unwrap();
            assert_eq!(p.instructions.len(), 9);
        }

        #[test]
        fn execute() {
            let p = Program::new(&input()).unwrap();
            assert_eq!(p.execute().unwrap(), (5, false));

            let p = Program::new(&correct_input()).unwrap();
            assert_eq!(p.execute().unwrap(), (8, true));
        }

        #[test]
        fn correct() {
            let mut p = Program::new(&input()).unwrap();
            assert_eq!(p.correct().unwrap(), 8);
        }
    }
}
