pub enum NodeType{
    Program,
    NumericLiteral,
    Identifier,
    BinOp,
    CallExpr,
    UnaryExpr,
    FnDecl,
}

pub struct Program{
    kind: NodeType,
    body: Vec<NodeType>
}

