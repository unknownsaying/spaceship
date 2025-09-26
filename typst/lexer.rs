// src/lexer.rs
use logos::{Logos, Lexer as LogosLexer};
use crate::error::{MathError, Result};

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\n\f]+")] // Skip whitespace
pub enum Token {
    // Identifiers and numbers
    #[regex(r"[a-zA-Z][a-zA-Z0-9]*")]
    Identifier(String),
    
    #[regex(r"[0-9]+(\.[0-9]+)?([eE][+-]?[0-9]+)?")]
    Number(String),
    
    // Operators
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Asterisk,
    #[token("/")]
    Slash,
    #[token("=")]
    Equal,
    #[token("!=")]
    NotEqual,
    #[token("<")]
    Less,
    #[token(">")]
    Greater,
    #[token("<=")]
    LessEqual,
    #[token(">=")]
    GreaterEqual,
    
    // Grouping
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token("|")]
    VerticalBar,
    
    // Scripts
    #[token("^")]
    Caret,
    #[token("_")]
    Underscore,
    
    // Special commands
    #[token("\\frac")]
    Frac,
    #[token("\\sqrt")]
    Sqrt,
    #[token("\\sum")]
    Sum,
    #[token("\\int")]
    Integral,
    #[token("\\lim")]
    Limit,
    
    // Greek letters and common symbols
    #[token("\\alpha")] Alpha,
    #[token("\\beta")] Beta,
    #[token("\\gamma")] Gamma,
    #[token("\\Gamma")] CapitalGamma,
    // ... more Greek letters
    
    // Punctuation
    #[token(",")]
    Comma,
    #[token(";")]
    Semicolon,
    #[token(":")]
    Colon,
    
    // End of input
    Eof,
}

pub struct Lexer<'a> {
    inner: LogosLexer<'a, Token>,
    peeked: Option<Option<Token>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            inner: Token::lexer(input),
            peeked: None,
        }
    }
    
    pub fn next_token(&mut self) -> Result<Token> {
        if let Some(peeked) = self.peeked.take() {
            return peeked.ok_or_else(|| MathError::Lexer {
                position: self.inner.span().start,
                message: "Unexpected end of input".to_string(),
            });
        }
        
        match self.inner.next() {
            Some(Ok(token)) => Ok(token),
            Some(Err(_)) => Err(MathError::Lexer {
                position: self.inner.span().start,
                message: "Invalid token".to_string(),
            }),
            None => Ok(Token::Eof),
        }
    }
    
    pub fn peek_token(&mut self) -> Result<&Token> {
        if self.peeked.is_none() {
            self.peeked = Some(Some(self.next_token()?));
        }
        
        match self.peeked.as_ref().unwrap() {
            Some(token) => Ok(token),
            None => Err(MathError::Lexer {
                position: self.inner.span().start,
                message: "Unexpected end of input".to_string(),
            }),
        }
    }
    
    pub fn span(&self) -> std::ops::Range<usize> {
        self.inner.span()
    }
}