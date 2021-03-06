use crate::error::AppError;
use crate::result::Result;
use bigdecimal::*;
use std::str::FromStr;

#[derive(PartialEq, Debug, Clone)]
pub enum Symbol {
    Decimal(BigDecimal),
    Operator(Operator),
}
#[derive(PartialEq, Debug, Clone)]
pub enum Operator {
    Plus,
    Minus,
    Division,
    Multiplication,
}

struct Parser {
    expression: Vec<Symbol>,
    cache: String,
}

pub fn parse(input: &str) -> Result<Vec<Symbol>> {
    let parser = Parser {
        expression: vec![],
        cache: String::new(),
    };
    parser.parse(input)
}

impl Parser {
    fn parse(mut self, input: &str) -> Result<Vec<Symbol>> {
        for c in input.chars() {
            if c >= '0' && c <= '9' || c == '.' {
                self.cache.push(c);
            } else {
                match c {
                    '+' => self.on_sigin_operator(c, Operator::Plus)?,
                    '-' => self.on_sigin_operator(c, Operator::Minus)?,
                    '/' => {
                        self.flush()?;
                        self.expression.push(Symbol::Operator(Operator::Division));
                    }
                    '*' | '×' => {
                        self.flush()?;
                        self.expression
                            .push(Symbol::Operator(Operator::Multiplication));
                    }
                    '%' => {
                        self.flush()?;
                        let first_symbol =
                            self.expression.last_mut().ok_or(AppError::InvalidPercent)?;
                        if let Symbol::Decimal(decimal) = first_symbol {
                            *decimal = decimal.clone() / 100;
                        } else {
                            Err(AppError::InvalidPercent)?;
                        }
                    }
                    ' ' => (),
                    v => Err(AppError::InvalidChar(v))?,
                };
            }
        }
        self.flush()?;
        Ok(self.expression)
    }
    fn on_sigin_operator(&mut self, c: char, op: Operator) -> Result<()> {
        self.flush()?;
        if self.expression.is_empty() {
            self.cache.push(c);
        } else {
            self.expression.push(Symbol::Operator(op));
        }
        Ok(())
    }
    fn flush(&mut self) -> Result<()> {
        if !self.cache.is_empty() {
            let bd = BigDecimal::from_str(&self.cache)
                .or_else(|_| Err(AppError::InvalidString(self.cache.clone())))?;
            if let Some(Symbol::Decimal(_)) = self.expression.last() {
                Err(AppError::InvalidExpression)?;
            }
            self.expression.push(Symbol::Decimal(bd));
        }
        self.cache.clear();
        Ok(())
    }
}

impl Operator {
    pub fn operate(&self, v1: &BigDecimal, v2: &BigDecimal) -> BigDecimal {
        match self {
            Operator::Plus => v1 + v2,
            Operator::Minus => v1 - v2,
            Operator::Division => v1 / v2,
            Operator::Multiplication => v1 * v2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("+100",vec![Symbol::Decimal(BigDecimal::from(100))] => Ok(()))]
    #[test_case("101",vec![Symbol::Decimal(BigDecimal::from(101))]=> Ok(()))]
    #[test_case("-102",vec![Symbol::Decimal(BigDecimal::from(-102))]=> Ok(()))]
    #[test_case("102-2000",vec![Symbol::Decimal(BigDecimal::from(102)),Symbol::Operator(Operator::Minus),Symbol::Decimal(BigDecimal::from(2000))]=> Ok(()))]
    #[test_case("2000*1000",vec![Symbol::Decimal(BigDecimal::from(2000)),Symbol::Operator(Operator::Multiplication),Symbol::Decimal(BigDecimal::from(1000))] => Ok(()))]
    #[test_case("4000×2000",vec![Symbol::Decimal(BigDecimal::from(4000)),Symbol::Operator(Operator::Multiplication),Symbol::Decimal(BigDecimal::from(2000))] => Ok(()))]
    #[test_case("4000%",vec![Symbol::Decimal(BigDecimal::from(40))] => Ok(()))]
    #[test_case("4000%%",vec![Symbol::Decimal(BigDecimal::from(0.4))] => Ok(()))]
    #[test_case("2.143",vec![Symbol::Decimal(BigDecimal::from(2.143))]=> Ok(()))]
    #[test_case("-3.343",vec![Symbol::Decimal(BigDecimal::from(-3.343))]=> Ok(()))]
    #[test_case("-a3.343",vec![Symbol::Decimal(BigDecimal::from(-3.343))]=> Err(AppError::InvalidChar('a')))]
    #[test_case("4000%20",vec![] => Err(AppError::InvalidExpression))]
    #[test_case("\u{0028}400+20\u{0029}*20",vec![] => Err(AppError::InvalidChar('\u{0028}')))]
    #[test_case("400 + 23",vec![Symbol::Decimal(BigDecimal::from(400)),Symbol::Operator(Operator::Plus),Symbol::Decimal(BigDecimal::from(23))] => Ok(()))]
    fn parse_works(input: &str, expected: Vec<Symbol>) -> Result<()> {
        assert_eq!(expected, parse(input)?);
        Ok(())
    }

    #[test_case(Operator::Plus,BigDecimal::from(100),BigDecimal::from(300) => BigDecimal::from(400))]
    #[test_case(Operator::Minus,BigDecimal::from(300),BigDecimal::from(500) => BigDecimal::from(-200))]
    #[test_case(Operator::Multiplication,BigDecimal::from(300),BigDecimal::from(500) => BigDecimal::from(150000))]
    #[test_case(Operator::Division,BigDecimal::from(500),BigDecimal::from(100) => BigDecimal::from(5))]
    fn operate_works(op: Operator, v1: BigDecimal, v2: BigDecimal) -> BigDecimal {
        op.operate(&v1, &v2)
    }
}
