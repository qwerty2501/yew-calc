use crate::error::*;
use crate::parser::*;
use crate::result::Result;
use bigdecimal::*;

pub fn evaluate(input: &str) -> Result<Option<String>> {
    Ok(evaluate_expression(parse(input)?)?.map(|r| r.to_string()))
}

fn evaluate_expression(expression: Vec<Symbol>) -> Result<Option<BigDecimal>> {
    let expression = operate_expression(expression, Operator::Division)?;
    let expression = operate_expression(expression, Operator::Multiplication)?;
    let expression = operate_expression(expression, Operator::Plus)?;
    let mut expression = operate_expression(expression, Operator::Minus)?;
    if expression.len() == 1 {
        let symbol = expression.pop().unwrap();
        if let Symbol::Decimal(decimal) = symbol {
            Ok(Some(decimal))
        } else {
            Err(AppError::InvalidExpression)
        }
    } else if expression.is_empty() {
        Ok(None)
    } else {
        Err(AppError::InvalidExpression)
    }
}

fn operate_expression(expression: Vec<Symbol>, op: Operator) -> Result<Vec<Symbol>> {
    let mut result_expression = Vec::with_capacity(expression.len());
    let mut index = 0;
    while index < expression.len() {
        let symbol = expression.get(index).ok_or(AppError::InvalidExpression)?;
        index += 1;
        match symbol {
            Symbol::Decimal(_) => result_expression.push(symbol.clone()),
            Symbol::Operator(iop) => {
                if iop == &op {
                    let v1 = if let Some(Symbol::Decimal(decimal)) = result_expression.pop() {
                        Ok(decimal)
                    } else {
                        Err(AppError::InvalidExpression)
                    }?;

                    let v2 = if let Some(Symbol::Decimal(decimal)) = expression.get(index) {
                        index += 1;
                        Ok(decimal)
                    } else {
                        Err(AppError::InvalidExpression)
                    }?;
                    result_expression.push(Symbol::Decimal(op.operate(&v1, v2)));
                } else {
                    result_expression.push(symbol.clone());
                }
            }
        }
    }
    Ok(result_expression)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![Symbol::Decimal(BigDecimal::from(100))],Operator::Multiplication => Ok(vec![Symbol::Decimal(BigDecimal::from(100))]))]
    #[test_case(vec![
        Symbol::Decimal(BigDecimal::from(200)),
        Symbol::Operator(Operator::Plus),
        Symbol::Decimal(BigDecimal::from(100)),
    ],
    Operator::Multiplication => Ok(vec![
        Symbol::Decimal(BigDecimal::from(200)),
        Symbol::Operator(Operator::Plus),
        Symbol::Decimal(BigDecimal::from(100)),
    ]))]
    #[test_case(vec![
        Symbol::Decimal(BigDecimal::from(200)),
        Symbol::Operator(Operator::Multiplication),
        Symbol::Decimal(BigDecimal::from(100)),
    ],
    Operator::Multiplication => Ok(vec![
        Symbol::Decimal(BigDecimal::from(20000)),
    ]))]
    #[test_case(vec![Symbol::Decimal(BigDecimal::from(100))],Operator::Plus => Ok(vec![Symbol::Decimal(BigDecimal::from(100))]))]
    #[test_case(vec![
        Symbol::Decimal(BigDecimal::from(200)),
        Symbol::Operator(Operator::Minus),
        Symbol::Decimal(BigDecimal::from(100)),
    ],
    Operator::Plus => Ok(vec![
        Symbol::Decimal(BigDecimal::from(200)),
        Symbol::Operator(Operator::Minus),
        Symbol::Decimal(BigDecimal::from(100)),
    ]))]
    #[test_case(vec![
        Symbol::Decimal(BigDecimal::from(200)),
        Symbol::Operator(Operator::Plus),
        Symbol::Decimal(BigDecimal::from(100)),
    ],
    Operator::Plus => Ok(vec![
        Symbol::Decimal(BigDecimal::from(300)),
    ]))]
    #[test_case(vec![Symbol::Decimal(BigDecimal::from(100))],Operator::Minus => Ok(vec![Symbol::Decimal(BigDecimal::from(100))]))]
    #[test_case(vec![
        Symbol::Decimal(BigDecimal::from(200)),
        Symbol::Operator(Operator::Plus),
        Symbol::Decimal(BigDecimal::from(100)),
    ],
    Operator::Minus => Ok(vec![
        Symbol::Decimal(BigDecimal::from(200)),
        Symbol::Operator(Operator::Plus),
        Symbol::Decimal(BigDecimal::from(100)),
    ]))]
    #[test_case(vec![
        Symbol::Decimal(BigDecimal::from(200)),
        Symbol::Operator(Operator::Minus),
        Symbol::Decimal(BigDecimal::from(100)),
    ],
    Operator::Minus => Ok(vec![
        Symbol::Decimal(BigDecimal::from(100)),
    ]))]
    #[test_case(vec![Symbol::Decimal(BigDecimal::from(100))],Operator::Division => Ok(vec![Symbol::Decimal(BigDecimal::from(100))]))]
    #[test_case(vec![
        Symbol::Decimal(BigDecimal::from(200)),
        Symbol::Operator(Operator::Plus),
        Symbol::Decimal(BigDecimal::from(100)),
    ],
    Operator::Division => Ok(vec![
        Symbol::Decimal(BigDecimal::from(200)),
        Symbol::Operator(Operator::Plus),
        Symbol::Decimal(BigDecimal::from(100)),
    ]))]
    #[test_case(vec![
        Symbol::Decimal(BigDecimal::from(200)),
        Symbol::Operator(Operator::Division),
        Symbol::Decimal(BigDecimal::from(100)),
    ],
    Operator::Division => Ok(vec![
        Symbol::Decimal(BigDecimal::from(2)),
    ]))]
    fn operate_expression_works(expression: Vec<Symbol>, op: Operator) -> Result<Vec<Symbol>> {
        operate_expression(expression, op)
    }

    #[test_case("" => Ok(None))]
    #[test_case("100" => Ok(Some("100".to_string())))]
    #[test_case("100+ 200" => Ok(Some("300".to_string())))]
    #[test_case("100+ 200 * 500" => Ok(Some("100100".to_string())))]
    #[test_case("100+ 3 / 100" => Ok(Some("100.03".to_string())))]
    #[test_case("abc" => Err(AppError::InvalidChar('a')))]
    #[test_case("100+++" => Err(AppError::InvalidExpression))]
    #[test_case("*100" => Err(AppError::InvalidExpression); "first_multipliton_operator_case")]
    #[test_case("/100" => Err(AppError::InvalidExpression); "first_division_operator_case")]
    #[test_case("+100" => Ok(Some("100".to_string())); "first_plus_operator_case")]
    fn evaluate_works(input: &str) -> Result<Option<String>> {
        evaluate(input)
    }
}
