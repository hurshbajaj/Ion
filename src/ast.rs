use ion_macros::*;
use std::any::Any;
use std::fmt::Debug;
use num_traits::{Num, ToPrimitive, FromPrimitive};

pub enum NodeType{
    Program,
    NumericLiteralNode,
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

#[Expr(NodeType::NumericLiteralNode)]
pub struct NumericLiteral<T: Num + Debug = f64>{
    pub value: T,
}
#[Expr(NodeType::Nil)]
pub struct Nil{}

impl Clone for Box<dyn Stmt> {
    fn clone(&self) -> Box<dyn Stmt> {
        self.clone_box()
    }
}
impl Clone for Box<dyn Expr> {
    fn clone(&self) -> Box<dyn Expr> {
        self.clone_box_expr()
    }
}

pub trait Stmt: Debug + Any{
    fn kind(&self) -> NodeType;
    fn as_any(&self) -> &dyn Any;
    fn clone_box(&self) -> Box<dyn Stmt>;
}
pub trait Expr: Stmt{
    fn clone_box_expr(&self) -> Box<dyn Expr>;
}
