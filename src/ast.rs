use ion_macros::*;
use std::any::Any;
use std::fmt::Debug;
use num_traits::{Num, ToPrimitive, FromPrimitive};

use crate::lexer::{Attr, Flags, TokenType};

#[derive(PartialEq)]
pub enum NodeType{
    //Stmt
    Program,
    VarDecl,
    VarAsg,

    //Expr

    NumericLiteralNode,
    Identifier,
    BinOp,
    Nil,
    Bool,

    Object,
    Property,
    ObjectLiteral,
    PropertyLiteral
}

#[Stmt(NodeType::Program)] 
pub struct Program{
    pub body: Vec<Box<dyn Stmt>>
}

#[Stmt(NodeType::VarDecl)] 
pub struct VarDeclaration{
    pub identifier: String,
    pub flags: Vec<Flags>,
    pub value: Box<dyn Expr>
}

#[Stmt(NodeType::VarAsg)] 
pub struct VarAsg{
    pub lhs: Box<dyn Expr>,
    pub rhs: Box<dyn Expr>
}


#[Expr(NodeType::Bool)]
pub struct Bool{
    pub value: bool
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

#[Expr(NodeType::Property)]
pub struct Property{
    pub key: String,
    pub value: Attr,
}

#[Expr(NodeType::Object)]
pub struct Object{
    pub properties: Vec<Property>,
}

#[Expr(NodeType::PropertyLiteral)]
pub struct PropertyLiteral{
    pub key: String,
    pub value: Box<dyn Expr>,
}

#[Expr(NodeType::ObjectLiteral)]
pub struct ObjectLiteral{
    pub properties: Vec<PropertyLiteral>,
}

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
