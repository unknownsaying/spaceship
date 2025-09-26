// src/parser.rs
use crate::lexer::{Lexer, Token};
use crate::ast::*;
use crate::error::{MathError, Result};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Result<Self> {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.next_token()?;
        
        Ok(Self {
            lexer,
            current_token,
        })
    }
    
    pub fn parse_math_expression(&mut self) -> Result<MathExpr> {
        self.parse_expression()
    }
    
    fn parse_expression(&mut self) -> Result<MathExpr> {
        self.parse_binary_expression(0)
    }
    
    fn parse_binary_expression(&mut self, min_precedence: u8) -> Result<MathExpr> {
        let mut left = self.parse_unary_expression()?;
        
        while let Some(op) = self.get_binary_operator() {
            let precedence = Self::get_operator_precedence(&op);
            if precedence < min_precedence {
                break;
            }
            
            self.advance()?; // Consume the operator
            
            let right = self.parse_binary_expression(precedence + 1)?;
            
            left = MathExpr {
                root: MathNode::BinaryOp {
                    op,
                    left: Box::new(left.root),
                    right: Box::new(right.root),
                },
                style: MathStyle::default(),
            };
        }
        
        Ok(left)
    }
    
    fn parse_unary_expression(&mut self) -> Result<MathExpr> {
        match &self.current_token {
            Token::Plus => {
                self.advance()?;
                let operand = self.parse_unary_expression()?;
                Ok(MathExpr {
                    root: MathNode::UnaryOp {
                        op: UnaryOperator::Plus,
                        operand: Box::new(operand.root),
                    },
                    style: MathStyle::default(),
                })
            }
            Token::Minus => {
                self.advance()?;
                let operand = self.parse_unary_expression()?;
                Ok(MathExpr {
                    root: MathNode::UnaryOp {
                        op: UnaryOperator::Minus,
                        operand: Box::new(operand.root),
                    },
                    style: MathStyle::default(),
                })
            }
            _ => self.parse_primary_expression(),
        }
    }
    
    fn parse_primary_expression(&mut self) -> Result<MathExpr> {
        let mut expr = self.parse_atom()?;
        
        // Handle scripts (superscript/subscript)
        while matches!(&self.current_token, Token::Caret | Token::Underscore) {
            if self.current_token == Token::Caret {
                self.advance()?;
                let superscript = self.parse_primary_expression()?;
                expr = MathExpr {
                    root: MathNode::Superscript {
                        base: Box::new(expr.root),
                        superscript: Box::new(superscript.root),
                    },
                    style: expr.style,
                };
            } else if self.current_token == Token::Underscore {
                self.advance()?;
                let subscript = self.parse_primary_expression()?;
                expr = MathExpr {
                    root: MathNode::Subscript {
                        base: Box::new(expr.root),
                        subscript: Box::new(subscript.root),
                    },
                    style: expr.style,
                };
            }
        }
        
        Ok(expr)
    }
    
    fn parse_atom(&mut self) -> Result<MathExpr> {
        match &self.current_token {
            Token::Identifier(name) => {
                let name = name.clone();
                self.advance()?;
                Ok(MathExpr {
                    root: MathNode::Identifier(name),
                    style: MathStyle::default(),
                })
            }
            Token::Number(value) => {
                let value: f64 = value.parse().map_err(|_| MathError::Parser {
                    message: format!("Invalid number: {}", value),
                })?;
                self.advance()?;
                Ok(MathExpr {
                    root: MathNode::Number(value),
                    style: MathStyle::default(),
                })
            }
            Token::LeftParen => {
                self.advance()?;
                let expr = self.parse_expression()?;
                self.expect(Token::RightParen)?;
                Ok(MathExpr {
                    root: MathNode::Group(Box::new(expr.root)),
                    style: expr.style,
                })
            }
            Token::Frac => {
                self.advance()?;
                self.expect(Token::LeftBrace)?;
                let numerator = self.parse_expression()?;
                self.expect(Token::RightBrace)?;
                self.expect(Token::LeftBrace)?;
                let denominator = self.parse_expression()?;
                self.expect(Token::RightBrace)?;
                
                Ok(MathExpr {
                    root: MathNode::Fraction {
                        numerator: Box::new(numerator.root),
                        denominator: Box::new(denominator.root),
                    },
                    style: MathStyle::default(),
                })
            }
            Token::Sqrt => {
                self.advance()?;
                let degree = if self.current_token == Token::LeftBracket {
                    self.advance()?;
                    let degree_expr = self.parse_expression()?;
                    self.expect(Token::RightBracket)?;
                    Some(Box::new(degree_expr.root))
                } else {
                    None
                };
                
                self.expect(Token::LeftBrace)?;
                let radicand = self.parse_expression()?;
                self.expect(Token::RightBrace)?;
                
                Ok(MathExpr {
                    root: MathNode::Root {
                        degree,
                        radicand: Box::new(radicand.root),
                    },
                    style: MathStyle::default(),
                })
            }
            Token::Sum => {
                self.advance()?;
                let (lower, upper) = self.parse_limits()?;
                self.expect(Token::LeftBrace)?;
                let body = self.parse_expression()?;
                self.expect(Token::RightBrace)?;
                
                Ok(MathExpr {
                    root: MathNode::Sum {
                        lower_limit: lower.map(Box::new),
                        upper_limit: upper.map(Box::new),
                        body: Box::new(body.root),
                    },
                    style: MathStyle::default(),
                })
            }
            _ => Err(MathError::Parser {
                message: format!("Unexpected token: {:?}", self.current_token),
            }),
        }
    }
    
    fn parse_limits(&mut self) -> Result<(Option<MathExpr>, Option<MathExpr>)> {
        let mut lower = None;
        let mut upper = None;
        
        if self.current_token == Token::Underscore {
            self.advance()?;
            if self.current_token == Token::LeftBrace {
                self.advance()?;
                lower = Some(self.parse_expression()?);
                self.expect(Token::RightBrace)?;
            } else {
                lower = Some(self.parse_primary_expression()?);
            }
        }
        
        if self.current_token == Token::Caret {
            self.advance()?;
            if self.current_token == Token::LeftBrace {
                self.advance()?;
                upper = Some(self.parse_expression()?);
                self.expect(Token::RightBrace)?;
            } else {
                upper = Some(self.parse_primary_expression()?);
            }
        }
        
        Ok((lower, upper))
    }
    
    fn get_binary_operator(&self) -> Option<BinaryOperator> {
        match &self.current_token {
            Token::Plus => Some(BinaryOperator::Plus),
            Token::Minus => Some(BinaryOperator::Minus),
            Token::Asterisk => Some(BinaryOperator::Multiply),
            Token::Slash => Some(BinaryOperator::Divide),
            Token::Equal => Some(BinaryOperator::Equal),
            Token::NotEqual => Some(BinaryOperator::NotEqual),
            Token::Less => Some(BinaryOperator::Less),
            Token::Greater => Some(BinaryOperator::Greater),
            Token::LessEqual => Some(BinaryOperator::LessEqual),
            Token::GreaterEqual => Some(BinaryOperator::GreaterEqual),
            _ => None,
        }
    }
    
    fn get_operator_precedence(op: &BinaryOperator) -> u8 {
        match op {
            BinaryOperator::Plus | BinaryOperator::Minus => 1,
            BinaryOperator::Multiply | BinaryOperator::Divide => 2,
            BinaryOperator::Equal | BinaryOperator::NotEqual => 0,
            BinaryOperator::Less | BinaryOperator::Greater 
            | BinaryOperator::LessEqual | BinaryOperator::GreaterEqual => 0,
            _ => 0,
        }
    }
    
    fn advance(&mut self) -> Result<()> {
        self.current_token = self.lexer.next_token()?;
        Ok(())
    }
    
    fn expect(&mut self, expected: Token) -> Result<()> {
        if self.current_token == expected {
            self.advance()?;
            Ok(())
        } else {
            Err(MathError::Parser {
                message: format!("Expected {:?}, found {:?}", expected, self.current_token),
            })
        }
    }
}