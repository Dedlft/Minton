use crate::ast::{Expr, MintonType};

pub struct TypeChecker;

impl TypeChecker {
	pub fn new() -> Self {
		Self
	}
	pub fn check_expr(&self, expr: &Expr) -> Result<MintonType, String> {
		match expr {
			Expr::Integer(_) => Ok(MintonType::Int),
			Expr::Literal(_) => Ok(MintonType::Str),
			Expr::BinaryOP {left, op, right} => {
				let left_type = self.check_expr(left)?;
				let right_type = self.check_expr(right)?;
				match op.as_str() {
					"+" | "-" | "*" | "/" => {
						if left_type == MintonType::Int && right_type == MintonType::Int {
							Ok(MintonType::Int)
						} else {
							Err(format!("Type Error: Cannot perform '{}' on {:?} and {:?}", op, left_type, right_type))
						  }
					}
					_ => Err(format!("Unknown operator: {}", op)),
				}
			}
			Expr::Pipeline {left, right} => {
				let left_type = self.check_expr(left)?;
				match &**right {
					Expr::Identifier(name) if name == "print" => Ok(MintonType::Void),
					_ => Err("Type Error: Right side of the pipeline must be a callable function".to_string()),
				}
			}
			Expr::Identifier(name) => {
				if name == "print" {
					Ok(MintonType::Void)
				} else {
					Err(format!("Undefined identifier: {}", name))
				  }
			}
		}
	}
}
