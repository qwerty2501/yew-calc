use crate::parser::*;
use crate::result::Result;

struct Calculator {
    status: Status,
}

enum Status {
    Empty,
}

impl Calculator {
    fn calc(&mut self, input: &str) -> Result<String> {
        match self.status {
            _ => Ok(input.to_string()),
        }
    }
}

pub fn calculate(input: &str) -> Result<String> {
    let _ = parse(input);
    let mut calculator = Calculator {
        status: Status::Empty,
    };
    calculator.calc(input)
}
