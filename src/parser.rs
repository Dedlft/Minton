use crate::lexer::{Lexer, Token};
use crate::ast::{Expr, Stmt, Program};

#[derive(Debug, PartialEq, PartialOrd)]
enum Precedence {
	Lowest,
	Pipe,
	Sum,
	Product,
	Prefix,
	Call
}

pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let cur_token = lexer.next_token();
        let peek_token = lexer.next_token();
        Self {
            lexer,
            cur_token,
            peek_token,
        }
    }

    fn next_token(&mut self) {
        self.cur_token = std::mem::replace(&mut self.peek_token, self.lexer.next_token());
    }
    
    fn get_precedence(token: &Token) -> Precedence {
		match token {
			Token::Pipe => Precedence::Pipe,
			Token::Plus | Token::Minus => Precedence::Sum,
			Token::Asterisk | Token::Slash => Precedence::Product,
			_ => Precedence::Lowest,
		}
	}

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program { statements: Vec::new() };
        while self.cur_token != Token::EOF {
            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
            }
            self.next_token();
        }
        program
    }

    fn parse_statement(&mut self) -> Option<Stmt> {
        let expr = self.parse_expression(Precedence::Lowest)?;
        if self.peek_token == Token::Semicolon {
            self.next_token();
        }
        Some(Stmt::Expression(expr))
    }
    
    fn parse_grouped_expression(&mut self) -> Option<Expr> {
		self.next_token();
		let expr = self.parse_expression(Precedence::Lowest)?;
		if self.peek_token == Token::Rparen {
			self.next_token();
			self.next_token();
			Some(expr)
		} else {
			None
		}
	}
	
	fn parse_binary_expression(&mut self, left: Expr) -> Option<Expr> {
		let op = match &self.cur_token {
			Token::Plus => "+".to_string(),
			Token::Minus => "-".to_string(),
			Token::Asterisk => "*".to_string(),
			Token::Slash => "/".to_string(),
			_ => return None,
		};
		
		let precedence = Self::get_precedence(&self.cur_token);
		self.next_token();
		let right = self.parse_expression(precedence)?;
		Some(Expr::BinaryOP {
			left: Box::new(left),
			op,
			right: Box::new(right),
		})
	}

    fn parse_expression(&mut self, precedence:Precedence) -> Option<Expr> {
        let mut left = match &self.cur_token {
			Token::Int(val) => Some(Expr::Integer(*val)),
            Token::String(s) => Some(Expr::Literal(s.clone())),
            Token::Identifier(i) => Some(Expr::Identifier(i.clone())),
            Token::Lparen => self.parse_grouped_expression(),
            _ => return None,
        }?;
        
        self.next_token();
        
        while self.cur_token != Token::Semicolon && self.cur_token != Token::EOF && precedence < Self::get_precedence(&self.peek_token) {
			match self.peek_token {
				Token::Plus | Token::Minus | Token::Asterisk | Token::Slash => {
					self.next_token();
					left = self.parse_binary_expression(left)?;
				}
				Token::Pipe => {
					self.next_token();
					self.next_token();
					let right = self.parse_expression(Precedence::Pipe)?;
					left = Expr::Pipeline {
						left: Box::new(left),
						right: Box::new(right),
					};
				}
				_ => break,
			}
		}
        Some(left)
    }
}
