use std::collections::HashSet;

#[derive(Debug, PartialEq, Default)]
pub enum ContractType {
    #[default]
    INTERFACE,
    CONTRACT,
}

pub struct Contract {
    pub name: String,
    pub fields: Vec<ContractField>,
    pub constructor: Function,
    pub events: Vec<Event>,
    pub enums: Vec<Enum>,
    pub structs: Vec<Struct>,
    pub functions: Vec<Function>,
    pub imports: HashSet<String>,
    pub comments: Vec<String>,
}

pub struct Interface {
    pub name: String,
    pub events: Vec<Event>,
    pub enums: Vec<Enum>,
    pub structs: Vec<Struct>,
    pub function_headers: Vec<FunctionHeader>,
    pub imports: HashSet<String>,
    pub comments: Vec<String>,
}

pub struct ContractField {
    pub field_type: String,
    pub name: String,
    pub comments: Vec<String>,
    pub initial_value: Option<String>,
    pub constant: bool,
}

pub struct Modifier {
    pub statements: Vec<Statement>,
    pub comments: Vec<String>,
}

#[derive(Clone)]
pub struct Event {
    pub name: String,
    pub fields: Vec<EventField>,
    pub comments: Vec<String>,
}

#[derive(Clone)]
pub struct EventField {
    pub indexed: bool,
    pub field_type: String,
    pub name: String,
}

pub struct Enum {
    pub name: String,
    pub values: Vec<String>,
    pub comments: Vec<String>,
}

pub struct Struct {
    pub name: String,
    pub fields: Vec<StructField>,
    pub comments: Vec<String>,
}

#[derive(Default, Clone)]
pub struct StructField {
    pub name: String,
    pub field_type: String,
}

#[derive(Default, Clone)]
pub struct Function {
    pub header: FunctionHeader,
    pub body: Vec<Statement>,
}

#[derive(Default, Clone)]
pub struct FunctionHeader {
    pub name: String,
    pub params: Vec<FunctionParam>,
    pub external: bool,
    pub view: bool,
    pub payable: bool,
    pub return_params: Vec<FunctionParam>,
    pub comments: Vec<String>,
}

#[derive(Clone)]
pub struct FunctionParam {
    pub name: String,
    pub param_type: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Statement {
    AddAssign(Expression, Expression),
    Assembly(Vec<Statement>),
    AssemblyEnd,
    Assign(Expression, Expression, Operation),
    Catch(Vec<Statement>),
    CatchEnd,
    Comment(String),
    Declaration(String, String, Option<Expression>),
    Else(Vec<Statement>),
    ElseIf(Condition, Vec<Statement>),
    Emit(String, Vec<Expression>),
    FunctionCall(Expression),
    If(Condition, Vec<Statement>),
    IfEnd,
    Raw(String),
    Require(Condition, String),
    Return(Expression),
    SubAssign(Expression, Expression),
    Try(Vec<Statement>),
    TryEnd,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Condition {
    pub left: Expression,
    pub operation: Operation,
    pub right: Option<Expression>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Operation {
    Add,
    AddAssign,
    AndAssign,
    Assign,
    BitwiseAnd,
    BitwiseOr,
    Div,
    DivAssign,
    Equal,
    GreaterThanEqual,
    GreaterThan,
    LessThanEqual,
    LessThan,
    LogicalAnd,
    LogicalOr,
    Mul,
    MulAssign,
    Not,
    NotEqual,
    OrAssign,
    Pow,
    Subtract,
    SubtractAssign,
    ShiftLeft,
    ShiftRight,
    True,
}

impl ToString for Operation {
    fn to_string(&self) -> String {
        return match self {
            Operation::Add => String::from("+"),
            Operation::AddAssign => String::from("+="),
            Operation::AndAssign => String::from("&="),
            Operation::Assign => String::from("="),
            Operation::BitwiseAnd => String::from("&"),
            Operation::BitwiseOr => String::from("|"),
            Operation::Div => String::from("/"),
            Operation::DivAssign => String::from("/="),
            Operation::Equal => String::from("=="),
            Operation::GreaterThanEqual => String::from(">="),
            Operation::GreaterThan => String::from(">"),
            Operation::LessThanEqual => String::from("<="),
            Operation::LessThan => String::from("<"),
            Operation::LogicalAnd => String::from("&&"),
            Operation::LogicalOr => String::from("||"),
            Operation::Mul => String::from("*"),
            Operation::MulAssign => String::from("*="),
            Operation::Not => String::from("!"),
            Operation::NotEqual => String::from("!="),
            Operation::OrAssign => String::from("|="),
            Operation::Pow => String::from("**"),
            Operation::ShiftLeft => String::from("<<"),
            Operation::ShiftRight => String::from(">>"),
            Operation::Subtract => String::from("-"),
            Operation::SubtractAssign => String::from("-="),
            Operation::True => String::from(""),
        }
    }
}

impl Operation {
    pub fn negate(&self) -> Operation {
        match self {
            Operation::BitwiseAnd => Operation::BitwiseOr,
            Operation::BitwiseOr => Operation::BitwiseAnd,
            Operation::Equal => Operation::NotEqual,
            Operation::GreaterThanEqual => Operation::LessThan,
            Operation::GreaterThan => Operation::LessThanEqual,
            Operation::LessThanEqual => Operation::GreaterThan,
            Operation::LessThan => Operation::GreaterThanEqual,
            // TODO a and b = neg(a) or neg (b)
            Operation::LogicalAnd => Operation::LogicalOr,
            Operation::LogicalOr => Operation::LogicalAnd,
            Operation::Not => Operation::True,
            Operation::NotEqual => Operation::Equal,
            Operation::Add => Operation::Subtract,
            _ => Operation::Not,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expression {
    Arithmetic(Box<Expression>, Box<Expression>, Operation),
    Cast(bool, String, Box<Expression>),
    Condition(Box<Condition>),
    EnvCaller(Option<String>),
    FunctionCall(String, Vec<Expression>, Option<String>, bool),
    IsZero(Box<Expression>),
    Literal(String),
    Logical(Box<Expression>, Operation, Box<Expression>),
    Member(String, Option<String>),
    Mapping(
        String,
        Vec<Expression>,
        Option<String>,
        Option<Box<Expression>>,
    ),
    StructArg(String, Box<Expression>),
    Ternary(Box<Condition>, Box<Expression>, Box<Expression>),
    WithSelector(Box<Expression>, Box<Expression>),
    ZeroAddressInto,
}

pub enum Block {
    Assembly,
    Catch,
    Else,
    ElseIf,
    If,
    Try,
    Unchecked,
}
