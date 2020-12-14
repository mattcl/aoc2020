use std::collections::HashMap;
use crate::error::{AocError, Result};
use std::str::FromStr;


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Program {
    memory: HashMap<usize, u64>
}

impl Program {
    pub fn new() -> Self {
        Program {
            memory: HashMap::new(),
        }
    }

    pub fn set(&mut self, addr: usize, val: u64) {
        self.memory.insert(addr, val);
    }

    pub fn get(&self, addr: usize) -> Option<&u64> {
        self.memory.get(&addr)
    }

    pub fn memory_sum(&self) -> u64 {
        self.memory.values().sum()
    }
}


#[derive(Debug, Clone, Eq, PartialEq)]
struct Mask {
    raw: Vec<char>,
    and: u64,
    or: u64,
}

impl Mask {
    pub fn new(s: &str) -> Result<Self> {
        Ok(
            Mask {
                raw: s.chars().collect(),
                and: u64::from_str_radix(&s.replace("X", "1"), 2)?,
                or: u64::from_str_radix(&s.replace("X", "0"), 2)?,
            }
        )
    }

    pub fn apply(&self, val: u64) -> u64 {
        (val & self.and) | self.or
    }

    pub fn apply_address(&self, addr: usize) -> Result<Vec<usize>> {
        let mut addr = format!("{:036b}", addr).chars().collect::<Vec<char>>();
        let mut addresses = Vec::new();
        self.recur(0, &mut addr, &mut addresses)?;
        Ok(addresses)
    }

    fn recur(&self, index: usize, addr: &mut [char], acc: &mut Vec<usize>) -> Result<()>{
        if index >= self.raw.len() {
            acc.push(usize::from_str_radix(&addr.iter().collect::<String>(), 2)?);
            return Ok(());
        }

        match self.raw[index] {
            '1' => {
                if let Some(ch) = addr.get_mut(index) {
                    *ch = '1';
                }
                self.recur(index + 1, addr, acc)?;
            },
            'X' => {
                if let Some(ch) = addr.get_mut(index) {
                    *ch = '1';
                }
                self.recur(index + 1, addr, acc)?;

                if let Some(ch) = addr.get_mut(index) {
                    *ch = '0';
                }
                self.recur(index + 1, addr, acc)?;
            },
            '0' => self.recur(index + 1, addr, acc)?,
            _ => unreachable!(),
        }

        Ok(())
    }

    pub fn apply_address_memoized(&self, addr: usize) -> std::result::Result<Vec<usize>, std::num::ParseIntError> {
        let mut addr = format!("{:036b}", addr).chars().collect::<Vec<char>>();
        let mut cache = HashMap::new();
        self.recur_memoized(0, &mut addr, &mut cache)
            .iter()
            .map(|a| usize::from_str_radix(a, 2))
            .collect::<std::result::Result<Vec<usize>, std::num::ParseIntError>>()
    }

    fn recur_memoized(
        &self,
        index: usize,
        addr: &mut [char],
        cache: &mut HashMap<usize, Vec<String>>
    ) -> Vec<String> {
        if index >= self.raw.len() {
            return vec![String::new()];
        }

        // cur string + append char
        let mut strings = Vec::new();

        if let Some(vals) = cache.get(&index) {
            return vals.clone();
        } else {
            match self.raw[index] {
                '1' => {
                    if let Some(ch) = addr.get_mut(index) {
                        *ch = '1';
                    }
                    self.recur_memoized(index + 1, addr, cache)
                        .iter()
                        .map(|s| format!("1{}", s))
                        .for_each(|s| strings.push(s));
                },
                'X' => {
                    if let Some(ch) = addr.get_mut(index) {
                        *ch = '1';
                    }
                    self.recur_memoized(index + 1, addr, cache)
                        .iter()
                        .map(|s| format!("1{}", s))
                        .for_each(|s| strings.push(s));

                    if let Some(ch) = addr.get_mut(index) {
                        *ch = '0';
                    }
                    self.recur_memoized(index + 1, addr, cache)
                        .iter()
                        .map(|s| format!("0{}", s))
                        .for_each(|s| strings.push(s));
                },
                '0' => {
                    self.recur_memoized(index + 1, addr, cache)
                        .iter()
                        .map(|s| format!("{}{}", addr[index], s))
                        .for_each(|s| strings.push(s));
                }
                _ => unreachable!(),
            }
        }

        cache.insert(index, strings.clone());

        strings
    }
}


#[derive(Debug, Clone, Eq, PartialEq)]
enum Instruction {
    Mask(String),
    Assign(usize, u64),
}

impl FromStr for Instruction {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split(" = ");
        if let Some(left) = parts.next() {
            if let Some(right) = parts.next() {
                if let Some(_extra) = parts.next() {
                    return Err(AocError::InvalidInput(format!("Instruction has too many components: '{}'", s)));
                }

                match left {
                    "mask" => { return Ok(Instruction::Mask(right.to_string())) }
                    _ => {
                        if left.starts_with("mem[") {
                            let addr = left
                                .strip_prefix("mem[")
                                .unwrap_or("Unknown instruction")
                                .strip_suffix("]")
                                .unwrap_or("Unknown instruction")
                                .parse::<usize>()?;
                            return Ok(Instruction::Assign(addr, right.parse::<u64>()?));
                        }
                    }
                };
            }
        }
        Err(AocError::InvalidInput(format!("Cannot parse instruction: '{}'", s)))
    }
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Initializer;

impl Initializer {
    pub fn initialize(input: &[String]) -> Result<Program> {
        let mut program = Program::new();
        let mut mask = None;

        let instructions = input
            .iter()
            .map(|line| Instruction::from_str(line))
            .collect::<Result<Vec<Instruction>>>()?;

        for instruction in &instructions {
            match instruction {
                Instruction::Mask(m) => {mask = Some(Mask::new(m)?); },
                Instruction::Assign(addr, value) => {
                    // apply mask to value
                    if let Some(ref mask) = mask {
                        program.set(*addr, mask.apply(*value));
                    } else {
                        program.set(*addr, *value);
                    }
                }
            }
        }


        Ok(program)
    }

    pub fn initialize_v2(input: &[String]) -> Result<Program> {
        let mut program = Program::new();
        let mut mask = None;

        let instructions = input
            .iter()
            .map(|line| Instruction::from_str(line))
            .collect::<Result<Vec<Instruction>>>()?;

        for instruction in &instructions {
            match instruction {
                Instruction::Mask(m) => {mask = Some(Mask::new(m)?); },
                Instruction::Assign(addr, value) => {
                    // apply mask to value
                    if let Some(ref mask) = mask {
                        mask
                            .apply_address(*addr)?
                            .iter()
                            .for_each(|a| program.set(*a, *value));
                    } else {
                        program.set(*addr, *value);
                    }
                }
            }
        }


        Ok(program)
    }

    pub fn initialize_v2_memoized(input: &[String]) -> Result<Program> {
        let mut program = Program::new();
        let mut mask = None;

        let instructions = input
            .iter()
            .map(|line| Instruction::from_str(line))
            .collect::<Result<Vec<Instruction>>>()?;

        for instruction in &instructions {
            match instruction {
                Instruction::Mask(m) => {mask = Some(Mask::new(m)?); },
                Instruction::Assign(addr, value) => {
                    // apply mask to value
                    if let Some(ref mask) = mask {
                        mask
                            .apply_address_memoized(*addr)?
                            .iter()
                            .for_each(|a| program.set(*a, *value));
                    } else {
                        program.set(*addr, *value);
                    }
                }
            }
        }


        Ok(program)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod instruction {
        use super::*;

        #[test]
        fn from_str() {
            let input = "mask = 0XX1XXX1101X101100101001010X1X110000";
            assert_eq!(
                Instruction::from_str(input).unwrap(),
                Instruction::Mask("0XX1XXX1101X101100101001010X1X110000".to_string())
            );

            let input = "mem[41476] = 14032";
            assert_eq!(
                Instruction::from_str(input).unwrap(),
                Instruction::Assign(41476, 14032)
            );

            let input = "mem[20538] = 23975525";
            assert_eq!(
                Instruction::from_str(input).unwrap(),
                Instruction::Assign(20538, 23975525)
            );

            let input = "mem[ajifjaioe] = 23975525";
            assert!(
                Instruction::from_str(input).is_err()
            );

            let input = "foo = 23975525";
            assert!(
                Instruction::from_str(input).is_err()
            );
        }
    }

    mod mask {
        use super::*;
        #[test]
        fn new() {
            let input = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
            let m = Mask::new(input);
            assert!(Mask::new(input).is_ok());

            let m = Mask::new(input).unwrap();
            assert_eq!(m.or, 64);
        }

        #[test]
        fn apply() {
            let input = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
            let m = Mask::new(input).unwrap();

            assert_eq!(m.apply(11), 73);
        }

        #[test]
        fn apply_address() {
            /*address: 000000000000000000000000000000101010  (decimal 42)
             * mask:    000000000000000000000000000000X1001X
             * result:  000000000000000000000000000000X1101X
             * After applying the mask, four bits are overwritten, three of which are different,
             * and two of which are floating. Floating bits take on every possible combination of
             * values; with two floating bits, four actual memory addresses are written:
             *
             * 000000000000000000000000000000011010  (decimal 26)
             * 000000000000000000000000000000011011  (decimal 27)
             * 000000000000000000000000000000111010  (decimal 58)
             * 000000000000000000000000000000111011  (decimal 59)
             */
            let input = "000000000000000000000000000000X1001X";
            let m = Mask::new(input).unwrap();

            let expected = vec![26, 27, 58, 59];
            for i in m.apply_address(42).unwrap() {
                assert!(expected.contains(&i));
            }

        }
    }

    mod initializer {
        use super::*;
        use crate::util::test_input;

        #[test]
        fn initialize() {
            let input = test_input("
                mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
                mem[8] = 11
                mem[7] = 101
                mem[8] = 0
            ");

            let program = Initializer::initialize(&input).unwrap();

            assert_eq!(program.get(7), Some(&101));
            assert_eq!(program.get(8), Some(&64));
            assert_eq!(program.memory_sum(), 165);
        }

        #[test]
        fn initialize_v2() {
            let input = test_input("
                mask = 000000000000000000000000000000X1001X
                mem[42] = 100
                mask = 00000000000000000000000000000000X0XX
                mem[26] = 1
            ");

            let program = Initializer::initialize_v2(&input).unwrap();

            assert_eq!(program.memory_sum(), 208);

            let program = Initializer::initialize_v2_memoized(&input).unwrap();

            assert_eq!(program.memory_sum(), 208);
        }
    }
}
