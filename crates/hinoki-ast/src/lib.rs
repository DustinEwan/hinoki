use std::rc::Rc;

#[derive(Debug)]
pub struct Program {
    pub instructions: Vec<TopLevelInstruction>,
}

#[derive(Debug)]
pub enum TopLevelInstruction {
    FunctionDefinition(FunctionDefinition),
    StructDefinition,
    EnumDefinition,
    Import,
    ImplDefinition,
    TraitDefinition,
    Statement(Statement),
    Expression,
    EOI,
}

#[derive(Debug)]
pub enum Locality {
    Local,
    Global,
}

#[derive(Debug)]
pub enum Exclusivity {
    Shared,
    Exclusive,
    Unique,
}

#[derive(Debug)]
pub enum Visibility {
    Public,
    Private,
}

#[derive(Debug)]
pub enum Mutability {
    Mutable,
    Immutable,
}

#[derive(Debug)]
pub struct FunctionParameter {
    pub locality: Locality,
    pub exclusivity: Exclusivity,
    pub mutability: Mutability,
    pub name: Box<str>,
    pub r#type: Type,
}

#[derive(Debug)]
pub enum Type {
    Integer { signed: bool, size: usize },
    Float { size: usize },
    Boolean,
    String(String),
    UserDefined(UserDefinedType),
    Inferred,
}

#[derive(Debug)]
pub struct FunctionDefinition {
    pub visibility: Visibility,
    pub locality: Locality,
    pub name: Box<str>,
    pub parameters: Box<[FunctionParameter]>,
    pub generic_parameters: Option<Box<[String]>>,
    pub return_type: Type,
    pub body: BlockExpr,
}

#[derive(Debug)]
pub struct UserDefinedType {
    pub name: Box<str>,
    pub generic_parameters: Option<Box<[String]>>,
}

#[derive(Debug)]
pub enum Command {
    Statement(Statement),
    Expression(Expression),
}

#[derive(Debug)]
pub enum Statement {
    Declaration(DeclarationStmt),
    Assignment(AssignmentStmt),
    Return(ReturnStmt),
    Expression(Expression),
}

#[derive(Debug)]
pub struct DeclarationStmt {}
#[derive(Debug)]
pub struct AssignmentStmt {}
#[derive(Debug)]
pub struct ReturnStmt {}

#[derive(Debug)]
pub enum Expression {
    Integer(i64),
    Float(f64),
    Binary(BinaryExpr),
    Block(Box<[Command]>),
}

#[derive(Debug)]
pub struct BlockExpr {
    pub commands: Box<[Command]>,
}

#[derive(Debug)]
pub struct BinaryExpr {
    pub lhs: Box<Expression>,
    pub op: Op,
    pub rhs: Box<Expression>,
}

#[derive(Debug)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
    Mod,
    Power,

    ShiftLeft,
    ShiftRight,
    BitwiseAnd,
    BitwiseOr,
    Xor,
    Not,

    LessThan,
    GreaterThan,
    Equal,
    LessThanOrEqual,
    GreaterThanOrEqual,
    NotEqual,
}
