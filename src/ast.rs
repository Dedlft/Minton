#[derive(Debug, Clone, PartialEq)]
pub enum MintonType {
	Int,
	Str,
	Void,
	Bool
}

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(String),
    Integer(i64),
    Identifier(String),
    BinaryOP {
		left: Box<Expr>,
		op: String,
		right: Box<Expr>,
	},
    Pipeline {
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

pub enum Stmt {
    Expression(Expr),
}

pub struct Program {
    pub statements: Vec<Stmt>,
}
