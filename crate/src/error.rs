use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum AppError {
    #[error("{0}は無効な文字です")]
    InvalidChar(char),

    #[error("{0}は無効な文字列です")]
    InvalidString(String),

    #[error("数式が間違っています")]
    InvalidExpression,

    #[error("%の前は数値でなくてはなりません")]
    InvalidPercent,
}
