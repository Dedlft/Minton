mod lexer;
mod ast;
mod parser;
mod evaluator;
mod typechecker;

use lexer::Lexer;
use parser::Parser;
use evaluator::Evaluator;
use typechecker::TypeChecker;

fn main() {
    let source = r#" (2+3)*4 | print;"#;
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    let checker = TypeChecker::new();
    let mut evaluator = Evaluator::new();

    for stmt in program.statements {
        match stmt {
			ast::Stmt::Expression(expr) => {
				match checker.check_expr(&expr) {
					Ok(_) => {
						evaluator.eval_expression(expr);
					}
					Err(e) => println!("{}", e),
				}
			}
		}
    }
}
