use crate::ast::{Expr, Stmt};

#[derive(Debug, Clone)]
pub enum Value {
	Int(i64),
	Str(String),
	Void,
}

pub struct Evaluator;

impl Evaluator {
    pub fn new() -> Self {
        Self
    }

    pub fn eval_statement(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Expression(expr) => {
                self.eval_expression(expr);
            }
        }
    }

    fn eval_expression(&mut self, expr: Expr) -> Value {
        match expr {
			Expr::Integer(v) => Value::Int(v),
            Expr::Literal(s) => Value::Str(s),
            Expr::BinaryOP { left, op, right } => {
				let left_val = self.eval_expression(*left);
				let right_val = self.eval_expression(*right);
				match (left_val, right_val) {
					(Value::Int(l), Value::Int(r)) => {
						match op.as_str() {
							"+" => Value::Int(l+r),
							"-" => Value::Int(l-r),
							"*" => Value::Int(l*r),
							"/" => Value::Int(l/r),
							_ => Value::Int(0),
						}
					}
					_ => Value::Void,
				}
			}
			Expr::Pipeline {left, right} => {
				let input = self.eval_expression(*left);
				self.execute_pipeline(input, *right)
			}
			Expr::Identifier(n) => Value::Str(n),
        }
    }

    fn execute_pipeline(&mut self, input: Value, action: Expr) -> Value {
        if let Expr::Identifier(name) = action {
			if name == "print" {
				match input {
					Value::Int(v) => println!("{}", v),
					Value::Str(v) => println!("{}", v),
					Value::Void => println!("void"),
				}
			}
		}
    Value::Void
	}
}
