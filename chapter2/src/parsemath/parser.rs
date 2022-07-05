use std::fmt;

use super::ast::Node;
use super::token::{OperPrec, Token};
use super::tokenizer::Tokenizer;

#[derive(Debug)]
pub enum ParseError {
    InvalidOperator(String),
    UnableToParse(String),
}

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    // parse the first character to create a filled Parser struct
    pub fn new(expr: &'a str) -> Result<Self, ParseError> {
        let mut tokenizer = Tokenizer::new(expr);
        let current_token = match tokenizer.next() {
            Some(token) => token,
            None => return Err(ParseError::InvalidOperator("Invalid character".into())),
        };
        Ok(Parser {
            tokenizer,
            current_token,
        })
    }

    pub fn parse(&mut self) -> Result<Node, ParseError> {
        self.generate_ast(OperPrec::DefaultZero)
    }
}

impl<'a> Parser<'a> {
    fn get_next_token(&mut self) -> Result<(), ParseError> {
        let next_token = match self.tokenizer.next() {
            Some(token) => token,
            None => return Err(ParseError::InvalidOperator("Invalid character".into())),
        };
        self.current_token = next_token;
        Ok(())
    }

    fn generate_ast(&mut self, oper_prec: OperPrec) -> Result<Node, ParseError> {
        let mut left_expr = self.parse_number()?;

        while oper_prec < self.current_token.get_oper_prec() {
            if self.current_token == Token::EOF {
                break;
            } else {
            }
            let right_expr = self.convert_token_to_node(left.expr.clone())?;
            left_expr = right_expr;
        }
        Ok(left_expr)
    }

    fn parse_number(&mut self) -> Result<Node, ParseError> {
        let token = self.current_token.clone();
        match token {
            Token::Subtract => {
                self.get_next_token()?;
                let expr = self.generate_ast(OperPrec::Negative)?;
                Ok(Node::Negative(Box::new(expr)))
            }
            Token::Num(i) => {
                self.get_next_token()?;
                Ok(Node::Number(i))
            }
            Token::LeftParen => {
                self.get_next_token()?;
                let expr = self.generate_ast(OperPrec::DefaultZero)?;
                self.check_paren(Token::RightParen)?;
                if self.current_token == Token::LeftParen {
                    let right = self.generate_ast(OperPrec::MulDiv)?;
                    return Ok(Node::Multiply(Box::new(expr), Box::new(right)));
                } else {
                }
                Ok(expr)
            }

            _ => Err(ParseError::UnableToParse("Unable to parse".to_string())),
        }
    }
}
