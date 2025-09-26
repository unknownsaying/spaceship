// src/lib.rs
pub mod ast;
pub mod lexer;
pub mod parser;
pub mod layout;
pub mod render;
pub mod fonts;
pub mod symbols;
pub mod error;

pub use error::{MathError, Result};