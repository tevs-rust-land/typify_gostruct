use std::collections::HashMap;
use std::fmt::{self, Display};

use crate::scanner::Position;
#[derive(Debug)]
pub struct StructDeclaration {
    pub name: String,
    pub body: Vec<AST>,
}

#[derive(Debug)]
pub enum AST {
    Error(Error),
    Declaration(Box<StructDeclaration>),
    Field(Field),
}
#[derive(Debug)]
pub struct FieldName(pub String);
#[derive(Debug)]

pub enum FieldType {
    One(DataType),
    List(DataType),
}
#[derive(Debug)]

pub struct Tag {
    name: String,
    value: String,
}
#[derive(Debug)]
pub enum Field {
    Blank,
    Plain(FieldName, FieldType),
    WithTags(FieldName, FieldType, HashMap<TagKey, TagValue>),
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct TagKey(pub String);

#[derive(Debug)]
pub struct TagValue(pub String);

#[derive(Debug, PartialEq, Clone)]
pub enum DataType {
    Number,
    String,
    Boolean,
    Custom(String),
    NotSpecified,
    Embedded,
}

#[derive(Debug)]
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

#[derive(Debug)]
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
            ParseError::UnknownElement(element) => write!(f, "Unknown element `{}`", element),
            ParseError::UnexpectedElement(element) => {
                write!(f, "Unexpected element `{}`", element)
            }
            ParseError::Missing(token, lexeme, Position { line, column, .. }) => {
                write!(
                    f,
                    "Expected `{}` but found `{}` at line {} column {} ",
                    token, lexeme, line, column
                )
            }
        }
    }
}
