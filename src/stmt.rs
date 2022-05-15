// This is an autogenerated file. Do not edit manually. Use gen-ast package.
// Use gen-ast package to generate this file.

use crate::error::*;
use crate::expr::Expr;
use crate::token::Token;
use std::hash::Hash;
use std::hash::Hasher;
use std::rc::Rc;

pub enum Stmt {
    Block(Rc<BlockStmt>),
    Class(Rc<ClassStmt>),
    Expression(Rc<ExpressionStmt>),
    Function(Rc<FunctionStmt>),
    If(Rc<IfStmt>),
    Print(Rc<PrintStmt>),
    Return(Rc<ReturnStmt>),
    Var(Rc<VarStmt>),
    While(Rc<WhileStmt>),
    Break(Rc<BreakStmt>),
}

impl PartialEq for Stmt {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Stmt::Block(a), Stmt::Block(b)) => Rc::ptr_eq(a, b),
            (Stmt::Class(a), Stmt::Class(b)) => Rc::ptr_eq(a, b),
            (Stmt::Expression(a), Stmt::Expression(b)) => Rc::ptr_eq(a, b),
            (Stmt::Function(a), Stmt::Function(b)) => Rc::ptr_eq(a, b),
            (Stmt::If(a), Stmt::If(b)) => Rc::ptr_eq(a, b),
            (Stmt::Print(a), Stmt::Print(b)) => Rc::ptr_eq(a, b),
            (Stmt::Return(a), Stmt::Return(b)) => Rc::ptr_eq(a, b),
            (Stmt::Var(a), Stmt::Var(b)) => Rc::ptr_eq(a, b),
            (Stmt::While(a), Stmt::While(b)) => Rc::ptr_eq(a, b),
            (Stmt::Break(a), Stmt::Break(b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }
}

impl Eq for Stmt {}

impl Hash for Stmt {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        match self {
            Stmt::Block(a) => {
                hasher.write_usize(Rc::as_ptr(a) as usize);
            }
            Stmt::Class(a) => {
                hasher.write_usize(Rc::as_ptr(a) as usize);
            }
            Stmt::Expression(a) => {
                hasher.write_usize(Rc::as_ptr(a) as usize);
            }
            Stmt::Function(a) => {
                hasher.write_usize(Rc::as_ptr(a) as usize);
            }
            Stmt::If(a) => {
                hasher.write_usize(Rc::as_ptr(a) as usize);
            }
            Stmt::Print(a) => {
                hasher.write_usize(Rc::as_ptr(a) as usize);
            }
            Stmt::Return(a) => {
                hasher.write_usize(Rc::as_ptr(a) as usize);
            }
            Stmt::Var(a) => {
                hasher.write_usize(Rc::as_ptr(a) as usize);
            }
            Stmt::While(a) => {
                hasher.write_usize(Rc::as_ptr(a) as usize);
            }
            Stmt::Break(a) => {
                hasher.write_usize(Rc::as_ptr(a) as usize);
            }
        }
    }
}

impl Stmt {
    pub fn accept<T>(&self, base: Rc<Stmt>, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxResult> {
        match self {
            Stmt::Block(v) => visitor.visit_block_stmt(base, v),
            Stmt::Class(v) => visitor.visit_class_stmt(base, v),
            Stmt::Expression(v) => visitor.visit_expression_stmt(base, v),
            Stmt::Function(v) => visitor.visit_function_stmt(base, v),
            Stmt::If(v) => visitor.visit_if_stmt(base, v),
            Stmt::Print(v) => visitor.visit_print_stmt(base, v),
            Stmt::Return(v) => visitor.visit_return_stmt(base, v),
            Stmt::Var(v) => visitor.visit_var_stmt(base, v),
            Stmt::While(v) => visitor.visit_while_stmt(base, v),
            Stmt::Break(v) => visitor.visit_break_stmt(base, v),
        }
    }
}

#[derive(Clone)]
pub struct BlockStmt {
    pub statements: Rc<Vec<Rc<Stmt>>>,
}

#[derive(Clone)]
pub struct ClassStmt {
    pub name: Token,
    pub superclass: Option<Rc<Expr>>,
    pub methods: Rc<Vec<Rc<Stmt>>>,
}

#[derive(Clone)]
pub struct ExpressionStmt {
    pub expression: Rc<Expr>,
}

#[derive(Clone)]
pub struct FunctionStmt {
    pub name: Token,
    pub params: Rc<Vec<Token>>,
    pub body: Rc<Vec<Rc<Stmt>>>,
}

#[derive(Clone)]
pub struct IfStmt {
    pub condition: Rc<Expr>,
    pub then_branch: Rc<Stmt>,
    pub else_branch: Option<Rc<Stmt>>,
}

#[derive(Clone)]
pub struct PrintStmt {
    pub expression: Rc<Expr>,
}

#[derive(Clone)]
pub struct ReturnStmt {
    pub keyword: Token,
    pub value: Option<Rc<Expr>>,
}

#[derive(Clone)]
pub struct VarStmt {
    pub name: Token,
    pub initializer: Option<Rc<Expr>>,
}

#[derive(Clone)]
pub struct WhileStmt {
    pub condition: Rc<Expr>,
    pub body: Rc<Stmt>,
}

#[derive(Clone)]
pub struct BreakStmt {
    pub token: Token,
}

pub trait StmtVisitor<T> {
    fn visit_block_stmt(&self, base: Rc<Stmt>, stmt: &BlockStmt) -> Result<T, LoxResult>;
    fn visit_class_stmt(&self, base: Rc<Stmt>, stmt: &ClassStmt) -> Result<T, LoxResult>;
    fn visit_expression_stmt(&self, base: Rc<Stmt>, stmt: &ExpressionStmt) -> Result<T, LoxResult>;
    fn visit_function_stmt(&self, base: Rc<Stmt>, stmt: &FunctionStmt) -> Result<T, LoxResult>;
    fn visit_if_stmt(&self, base: Rc<Stmt>, stmt: &IfStmt) -> Result<T, LoxResult>;
    fn visit_print_stmt(&self, base: Rc<Stmt>, stmt: &PrintStmt) -> Result<T, LoxResult>;
    fn visit_return_stmt(&self, base: Rc<Stmt>, stmt: &ReturnStmt) -> Result<T, LoxResult>;
    fn visit_var_stmt(&self, base: Rc<Stmt>, stmt: &VarStmt) -> Result<T, LoxResult>;
    fn visit_while_stmt(&self, base: Rc<Stmt>, stmt: &WhileStmt) -> Result<T, LoxResult>;
    fn visit_break_stmt(&self, base: Rc<Stmt>, stmt: &BreakStmt) -> Result<T, LoxResult>;
}
