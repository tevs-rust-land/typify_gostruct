use crate::scanner::Position;
pub struct StructDeclaration {
    pub name: String,
    pub body: Vec<AST>,
}
pub enum AST {
    Error(Error),
    Declaration(Box<StructDeclaration>),
    Field(Field),
}

pub struct FieldName(String);

pub enum FieldType {
    One(DataType),
    List(DataType),
}
pub struct Field(FieldName, FieldType);

pub enum DataType {
    Number,
    String,
    Boolean,
    Custom(String),
    NotSpecified,
}

pub enum Error {
    ParseError(ParseError),
}

#[derive(Debug)]
pub enum RequiredElements {
    StringLiteral,
    Struct,
    LeftBrace,
    Identifier,
    Colon,
}
pub enum ParseError {
    UnknownElement(String),
    UnexpectedEndOfFile,
    Missing(RequiredElements, String, Position),
}
