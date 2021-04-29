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
pub enum Field {
    One(FieldName, DataType),
    List(FieldName, DataType),
}

pub enum DataType {
    Number,
    String,
    Boolean,
    Custom(String),
    NotSpecified,
}

pub enum Error {
    ParseErrors(ParseErrors),
}

#[derive(Debug)]
pub enum RequiredElements {
    StringLiteral,
    Struct,
    LeftBrace,
    Identifier,
    Colon,
}
pub enum ParseErrors {
    UnexpectedEndOfFile,
    Missing(RequiredElements),
}
