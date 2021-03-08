use crate::scanner::*;
use std::rc::Rc;

#[derive(Debug)]
pub struct StructDefinition {
    pub name: String,
    pub body: GoStruct,
}

#[derive(Debug)]
pub enum GoStruct {
    StructNameOnly(String),
    StructNameWithTypeOnly(String, DataTypeEnum),
    StructWithJSONTags(String, DataTypeEnum, Vec<GoStruct>),
    StructWithList(String),
    StructWithListAndType(String, DataTypeEnum),
    StructWithIdentifierTypeOnly(String, String),
    StructWithListTypeAndJSONTags(String, DataTypeEnum, Vec<GoStruct>),
    StructWithCustomListIdentifier(String, String),
    StructWithCustomListIdentifierAndJSONTags(String, String, Vec<GoStruct>),
    StructWithIdentifierAndJSONTags(String, String, Vec<GoStruct>),
    Block(Box<Block>),
    StructDefinition(Rc<StructDefinition>),
    JSONName(String),
    Binding,
    Unknown,
}

#[derive(Debug)]
pub struct Block {
    pub statements: Vec<GoStruct>,
}
