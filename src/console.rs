use crate::error::{AocError, Result};
use std::collections::HashSet;

#[derive(Eq, PartialEq, Debug)]
pub enum Op {
    Acc,
    Jmp,
    Nop,
}

impl Op {
    pub fn from_str(input: &str) -> Result<Op> {
        match input {
            "acc" => Ok(Op::Acc),
            "jmp" => Ok(Op::Jmp),
            "nop" => Ok(Op::Nop),
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

    pub fn step(&self, ptr: i64, accumulator: i64) -> Result<(i64, i64)> {
        if let Some(cur) = self.instructions.get(ptr as usize) {
            let mut new_acc = accumulator;
            let new_ptr = match cur.op {
                Op::Acc => {
                    new_acc += cur.val;
                    ptr + 1
                }
                Op::Jmp => ptr + cur.val,
                Op::Nop => ptr + 1,
            };
            Ok((new_ptr, new_acc))
        } else {
            Err(AocError::InvalidProgram(format!(
                "Could not find instruction at {}",
                ptr
            )))
        }
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
            seen.insert(ptr);

            let res = self.step(ptr, accumulator)?;
            ptr = res.0;
            accumulator = res.1;

            if ptr == eof {
                break;
            }

            if ptr < 0 || ptr > eof {
                return Err(AocError::InvalidProgram(format!(
                    "Attempted to access instruction location out of bounds {} of {}",
                    ptr, eof
                )));
            }

            if seen.contains(&ptr) {
                return Ok((accumulator, false));
            }
        }

        Ok((accumulator, true))
    }

    pub fn correct(&mut self) -> Result<i64> {
        for i in 0..self.instructions.len() {
            if let Some(ins) = self.instructions.get_mut(i) {
                match ins.op {
                    Op::Jmp => ins.op = Op::Nop,
                    Op::Nop => ins.op = Op::Jmp,
                    _ => {}
                }
            }

            let res = self.execute();

            if let Some(ins) = self.instructions.get_mut(i) {
                match ins.op {
                    Op::Jmp => ins.op = Op::Nop,
                    Op::Nop => ins.op = Op::Jmp,
                    _ => {}
                }
            }

            // we could have done this before we put it back, but making
            // benchmarks more possible
            if let Ok((val, normal)) = res {
                if normal {
                    return Ok(val);
                }
            }
        }

        Err(AocError::InvalidProgram("Could not be fixed".to_string()))
    }

    pub fn correct_recursive(&mut self) -> Result<i64> {
        let mut seen = HashSet::new();
        let mut final_accumulator = 0;
        if self.execute_r(0, 0, false, &mut seen, &mut final_accumulator)? {
            return Ok(final_accumulator);
        }

        Err(AocError::InvalidProgram(
            "Could not fix program".to_string(),
        ))
    }

    pub fn execute_r(
        &mut self,
        ptr: i64,
        accumulator: i64,
        changed: bool,
        seen: &mut HashSet<i64>,
        final_accumulator: &mut i64,
    ) -> Result<bool> {
        let eof = self.instructions.len() as i64;

        if ptr < 0 || ptr > eof {
            return Err(AocError::InvalidProgram(format!(
                "Attempted to access instruction location out of bounds {} of {}",
                ptr, eof,
            )));
        }

        if ptr == eof {
            *final_accumulator = accumulator;
            return Ok(true);
        }

        if seen.contains(&ptr) {
            return Ok(false);
        }

        seen.insert(ptr);

        if !changed {
            let mut did_swap = true;
            if let Some(ins) = self.instructions.get_mut(ptr as usize) {
                match ins.op {
                    Op::Jmp => ins.op = Op::Nop,
                    Op::Nop => ins.op = Op::Jmp,
                    _ => did_swap = false,
                }
            }

            if did_swap {
                let res = self.step(ptr, accumulator)?;
                let execution_res = self.execute_r(res.0, res.1, true, seen, final_accumulator)?;

                if let Some(ins) = self.instructions.get_mut(ptr as usize) {
                    match ins.op {
                        Op::Jmp => ins.op = Op::Nop,
                        Op::Nop => ins.op = Op::Jmp,
                        _ => {}
                    }
                }

                if execution_res {
                    return Ok(true);
                }
            }
        }

        let res = self.step(ptr, accumulator)?;
        if self.execute_r(res.0, res.1, changed, seen, final_accumulator)? {
            return Ok(true);
        }

        seen.remove(&ptr);
        return Ok(false);
    }
}

#[cfg(test)]
mod tests {
    mod op {
        use super::super::*;

        #[test]
        fn from_str() {
            assert_eq!(Op::from_str("acc").unwrap(), Op::Acc);
            assert_eq!(Op::from_str("jmp").unwrap(), Op::Jmp);
            assert_eq!(Op::from_str("nop").unwrap(), Op::Nop);
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
            assert_eq!(i.op, Op::Acc);
            assert_eq!(i.val, 1);

            let i = Instruction::new("nop -1333").unwrap();
            assert_eq!(i.op, Op::Nop);
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

            let mut p = Program::new(&input()).unwrap();
            assert_eq!(p.correct_recursive().unwrap(), 8);
        }
    }
}
