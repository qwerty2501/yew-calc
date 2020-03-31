use crate::err::*;

struct Calculator {
    status: Status,
}

enum Status {
    Empty,
}

impl Calculator {
    fn calc(&mut self, display: &str) -> Result<String, Error> {
        match self.status {
            _ => Ok(display.to_string()),
        }
    }
}

pub fn calculate(display: &str) -> Result<String, Error> {
    let mut calculator = Calculator {
        status: Status::Empty,
    };
    calculator.calc(display)
}
