use crate::errors::location::Location;

#[derive(Debug, Clone)]
pub enum Ast<'a> {
    Expr(Expr<'a>),
    Statement(Statement<'a>),
}

#[derive(Debug, Clone)]
pub enum Expr<'a> {
    Literal {
        value: Lit,
        location: Location<'a>,
    },
    Binary {
        op: Op,
        lhs: Box<Expr<'a>>,
        rhs: Box<Expr<'a>>,
        location: Location<'a>,
    },
    Unary {
        op: Op,
        expr: Box<Expr<'a>>,
    },
    Ident {
        value: String,
        location: Location<'a>,
    },
    FuncCall {
        name: String,
        args: Vec<Expr<'a>>,
        location: Location<'a>,
    },
    Let {
        name: String,
        value: Box<Expr<'a>>,
        location: Location<'a>,
    },
    Assignment {
        name: String,
        value: Box<Expr<'a>>,
        location: Location<'a>,
    },
    Conditional {
        condition: Box<Expr<'a>>,
        code: Vec<Ast<'a>>,
        else_stmt: Option<Box<Ast<'a>>>,
        location: Location<'a>,
    },
}

#[derive(Debug, Clone)]
pub enum Statement<'a> {
    ForLoop {
        expr: Expr<'a>,
        code: Vec<Ast<'a>>,
        location: Location<'a>,
    },
    WhileLoop {
        conditoin: Box<Expr<'a>>,
        code: Vec<Ast<'a>>,
        location: Location<'a>,
    },
}

#[derive(Debug, Clone)]
pub enum Item<'a> {
    FunctionDef {
        name: String,
        parameters: Vec<(String, Type)>,
        body: Vec<Ast<'a>>,
        return_type: Type,
        location: Location<'a>,
    },
}

#[derive(Debug, Clone)]
pub struct Type {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Lit {
    Int(isize),
    UInt(usize),
    Float(f64),
    String(String),
    True,
    False,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    PLUS,
    MINUS,
    SLASH,
    STAR,
}
