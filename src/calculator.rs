#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    Op(Op),
    Val(i64)
}


#[derive(Debug, Eq, PartialEq)]
pub enum Op {
    Add,
    Multiply,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Parser;

impl Parser {
    pub fn extract_number(index: &mut usize, chars: &[char]) -> i64 {
        let mut val = 0;
        if let Some(ch) = chars.get(*index) {
            val = ch.to_digit(10).unwrap_or(0);
        }

        loop {
            if let Some(ch) = chars.get(*index + 1) {
                match ch {
                    '0'..='9' => {
                        val = val * 10 + ch.to_digit(10).unwrap();
                        *index += 1;
                    },
                    _ => break,
                }
            } else {
                break;
            }
        }

        val as i64
    }

    pub fn eval(&self, raw: &str) -> i64 {
        let mut index = 0;
        let chars = raw.chars().collect::<Vec<char>>();
        self.recur(&mut index, &chars)

    }

    pub fn recur(&self, index: &mut usize, chars: &[char]) -> i64 {
        let mut op: Option<Op> = None;
        let mut result: i64 = 0;

        loop {
            if let Some(ch) = chars.get(*index) {
                match ch {
                    '0'..='9' => {
                        // extract the whole number
                        let val = Parser::extract_number(index, chars);
                        match op {
                            Some(Op::Add) => result += val,
                            Some(Op::Multiply) => result *= val,
                            None => result = val,
                        }
                    },
                    '+' => op = Some(Op::Add),
                    '*' => op = Some(Op::Multiply),
                    '(' => {
                        *index += 1;
                        let val = self.recur(index, chars);
                        match op {
                            Some(Op::Add) => result += val,
                            Some(Op::Multiply) => result *= val,
                            None => result = val,
                        }
                    },
                    ')' => return result,
                    ' ' => {},
                    _ => unreachable!()
                }
            } else {
                break;
            }

            *index += 1;
        }

        result
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct AdvancedParser;

impl AdvancedParser {
    pub fn extract_number(index: &mut usize, chars: &[char]) -> i64 {
        let mut val = 0;
        if let Some(ch) = chars.get(*index) {
            val = ch.to_digit(10).unwrap_or(0);
        }

        loop {
            if let Some(ch) = chars.get(*index + 1) {
                match ch {
                    '0'..='9' => {
                        val = val * 10 + ch.to_digit(10).unwrap();
                        *index += 1;
                    },
                    _ => break,
                }
            } else {
                break;
            }
        }

        val as i64
    }

    pub fn eval(&self, raw: &str) -> i64 {
        let mut index = 0;
        let chars = raw.chars().collect::<Vec<char>>();
        self.recur(&mut index, &chars)

    }

    pub fn recur(&self, index: &mut usize, chars: &[char]) -> i64 {
        let mut tokens: Vec<Token> = Vec::new();

        loop {
            if let Some(ch) = chars.get(*index) {
                match ch {
                    '0'..='9' => {
                        let val = AdvancedParser::extract_number(index, chars);
                        tokens.push(Token::Val(val));
                    },
                    '+' => tokens.push(Token::Op(Op::Add)),
                    '*' => tokens.push(Token::Op(Op::Multiply)),
                    '(' => {
                        *index += 1;
                        let val = self.recur(index, chars);
                        tokens.push(Token::Val(val));
                    },
                    ')' => break,
                    ' ' => {},
                    _ => unreachable!()
                }
            } else {
                break;
            }

            *index += 1;
        }

        let mut index = 0;
        self.calculate(&mut index, &tokens)
    }

    pub fn sum(&self, initial: i64, index: &mut usize, tokens: &[Token]) -> i64 {
        let mut sum = initial;

        loop {
            if let Some(token) = tokens.get(*index + 1) {
                match token {
                    Token::Op(Op::Add) => {
                        *index += 1;
                        match tokens.get(*index + 1) {
                            Some(Token::Val(val)) => {
                                *index += 1;
                                sum += val;
                            },
                            _ => {},
                        }
                    },
                    _ => break
                }
            } else {
                break;
            }
        }

        sum
    }

    pub fn calculate(&self, index: &mut usize, tokens: &[Token]) -> i64 {
        let mut result: i64 = 0;
        let mut op: Option<Op> = None;

        loop {
            if let Some(token) = tokens.get(*index) {
                match token {
                    Token::Val(val) => {
                        match op {
                            Some(Op::Add) => result += val,
                            Some(Op::Multiply) => result *= self.sum(*val, index, tokens),
                            None => result = *val,
                        }
                    }
                    Token::Op(o) => {
                        match o {
                            Op::Add => op = Some(Op::Add),
                            Op::Multiply => op = Some(Op::Multiply),
                        }
                    }
                }
            } else {
                break;
            }

            *index += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod parser {
        use super::*;

        #[test]
        fn extract_number() {
            let mut index = 0;
            let chars = "572".chars().collect::<Vec<char>>();
            assert_eq!(Parser::extract_number(&mut index, &chars), 572);
            assert_eq!(index, 2);

            let mut index = 2;
            let chars = "ab5s".chars().collect::<Vec<char>>();
            assert_eq!(Parser::extract_number(&mut index, &chars), 5);
            assert_eq!(index, 2);

            let mut index = 2;
            let chars = "abzs".chars().collect::<Vec<char>>();
            assert_eq!(Parser::extract_number(&mut index, &chars), 0);
            assert_eq!(index, 2);
        }

        #[test]
        fn eval() {
            let p = Parser {};

            assert_eq!(p.eval("2 * 3 + (4 * 5)"), 26);
            assert_eq!(p.eval("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), 13632);
        }
    }

    mod advanced_parser {
        use super::*;

        #[test]
        fn eval() {
            let p = AdvancedParser {};

            assert_eq!(p.eval("1 + (2 * 3) + (4 * (5 + 6))"), 51);
            assert_eq!(p.eval("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
        }
    }
}
