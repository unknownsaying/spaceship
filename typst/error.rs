// src/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MathError {
    #[error("Lexer error at position {position}: {message}")]
    Lexer { position: usize, message: String },
    
    #[error("Parser error: {message}")]
    Parser { message: String },
    
    #[error("Layout error: {message}")]
    Layout { message: String },
    
    #[error("Font error: {message}")]
    Font { message: String },
    
    #[error("Unknown symbol: {symbol}")]
    UnknownSymbol { symbol: String },
    
    #[error("Invalid math expression: {reason}")]
    InvalidExpression { reason: String },
}

pub type Result<T> = std::result::Result<T, MathError>;