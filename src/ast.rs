use ion_macros::*;
use std::fmt::Debug;
use num_traits::{Num, ToPrimitive, FromPrimitive};

pub enum NodeType{
    Program,
    NumericLiteral,
    Identifier,
    BinOp,
    Nil,
}

#[Stmt(NodeType::Program)] 
pub struct Program{
    pub body: Vec<Box<dyn Stmt>>
}

#[Expr(NodeType::BinOp)]
pub struct BinExpr {
    pub left: Box<dyn Expr>,
    pub right: Box<dyn Expr>,
    pub operator: String,
}

#[Expr(NodeType::Identifier)]
pub struct Identifier{
    pub symbol: String,
}

#[Expr(NodeType::NumericLiteral)]
pub struct NumericLiteral<T: Num + Debug = f64>{
    pub value: T,
}

#[Expr(NodeType::Nil)]
pub struct Nil{}

pub trait Stmt: Debug{
    fn kind(&self) -> NodeType;
}
pub trait Expr: Stmt{}
