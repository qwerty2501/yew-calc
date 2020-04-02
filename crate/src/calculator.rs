use crate::error::*;
use crate::parser::*;
use crate::result::Result;
use bigdecimal::*;

pub fn calculate(input: &str) -> Result<String> {
    Ok(calculate_tokens(parse(input)?)?.to_string())
}

fn calculate_tokens(tokens: Vec<Token>) -> Result<BigDecimal> {
    let tokens = operate_tokens(tokens, Operator::Division)?;
    let tokens = operate_tokens(tokens, Operator::Multiplication)?;
    let tokens = operate_tokens(tokens, Operator::Plus)?;
    let mut tokens = operate_tokens(tokens, Operator::Minus)?;
    if tokens.len() == 1 {
        let token = tokens.pop().unwrap();
        if let Token::Decimal(decimal) = token {
            Ok(decimal)
        } else {
            Err(AppError::InvalidExpression)
        }
    } else {
        Err(AppError::InvalidExpression)
    }
}

fn operate_tokens(tokens: Vec<Token>, op: Operator) -> Result<Vec<Token>> {
    let mut result_tokens = Vec::with_capacity(tokens.len());
    for (index, token) in tokens.iter().enumerate() {
        match token {
            Token::Decimal(_) => result_tokens.push(token.clone()),
            Token::Operator(iop) => {
                if iop == &op {
                    let v1 = if let Some(Token::Decimal(decimal)) = result_tokens.pop() {
                        Ok(decimal)
                    } else {
                        Err(AppError::InvalidExpression)
                    }?;
                    let v2 = if let Some(Token::Decimal(decimal)) = tokens.get(index + 1) {
                        Ok(decimal)
                    } else {
                        Err(AppError::InvalidExpression)
                    }?;
                    result_tokens.push(Token::Decimal(op.operate(&v1, v2)));
                } else {
                    result_tokens.push(token.clone());
                }
            }
        }
    }
    Ok(result_tokens)
}
