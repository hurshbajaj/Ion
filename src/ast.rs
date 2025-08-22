use ion_macros::*;
use std::any::Any;
use std::fmt::Debug;
use num_traits::Num;

use crate::lexer::{Attr, Flags};

#[derive(PartialEq)]
pub enum NodeType{

    //Stmt

    Program,
    VarDecl,
    VarAsg,

    //Expr

    NumericLiteralNode,
    String,
    Identifier,
    BinOp,
    Nil,
    Bool,

    Object,
    MemberExpr,
    Array,

    Property,
    ObjectLiteral,
    PropertyLiteral,
    ArrayLiteral,

    FnStruct,
    Param,
    CallExpr
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

#[Expr(NodeType::String)]
pub struct Str{
    pub content: String,
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

#[Expr(NodeType::Array)]
pub struct Array{
    pub attr: Attr,
    pub complex_attr: Option<String>,
    pub length: usize,
}

#[Expr(NodeType::ArrayLiteral)]
pub struct ArrayLiteral{
    pub entries: Vec<Box<dyn Expr>>,
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

#[Expr(NodeType::MemberExpr)]
pub struct MemberExpr{
    pub obj: Box<dyn Expr>,
    pub prop: Box<dyn Expr>,
}

#[Expr(NodeType::Param)]
pub struct Param{
    pub param: String,
    pub param_type: Attr,
}

#[Expr(NodeType::FnStruct)]
pub struct FnStruct{
    pub params: Vec<Param>,
    pub ret_type: Attr,
}

#[Expr(NodeType::CallExpr)]
pub struct CallExpr{
    pub args: Vec<Box<dyn Expr>>,
    pub call_to: Box<dyn Expr>,
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
