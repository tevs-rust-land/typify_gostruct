use std::fmt::{self, Display};

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

pub struct FieldName(pub String);

pub enum FieldType {
    One(DataType),
    List(DataType),
}

pub struct Tag {
    name: String,
    value: String,
}
pub enum Field {
    Plain(FieldName, FieldType),
    WithWithTags(FieldName, FieldType, Vec<AST>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum DataType {
    Number,
    String,
    Boolean,
    Custom(String),
    NotSpecified,
    Embedded,
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

impl Display for RequiredElements {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RequiredElements::StringLiteral => write!(f, "StringLiteral"),
            RequiredElements::Struct => write!(f, "Struct"),
            RequiredElements::LeftBrace => write!(f, "LeftBrace"),
            RequiredElements::Identifier => write!(f, "Identifier"),
            RequiredElements::Colon => write!(f, "Colon"),
        }
    }
}

pub enum ParseError {
    UnexpectedElement(String),
    UnknownElement(String),
    UnexpectedEndOfFile,
    Missing(RequiredElements, String, Position),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::UnexpectedEndOfFile => write!(f, "Unexpected End Of file"),
            ParseError::UnknownElement(element) => write!(f, "Unknown element {}.", element),
            ParseError::UnexpectedElement(element) => write!(f, "Unexpected element {}.", element),
            ParseError::Missing(token, lexeme, Position { line, column, .. }) => {
                write!(
                    f,
                    "Expected {} but found `{}` at line {} column {}",
                    token, lexeme, line, column
                )
            }
        }
    }
}
