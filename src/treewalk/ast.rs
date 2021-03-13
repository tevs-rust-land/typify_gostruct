use crate::data_types::Type;
use std::rc::Rc;

#[derive(Debug)]
pub struct StructDefinition {
    pub name: String,
    pub body: GoStruct,
}

#[derive(Debug)]
pub enum GoStruct {
    StructNameOnly(String),
    StructNameWithTypeOnly(String, Type),
    StructWithJSONTags(String, Type, Vec<GoStruct>),
    StructWithList(String),
    StructWithListAndType(String, Type),
    StructWithIdentifierTypeOnly(String, String),
    StructWithListTypeAndJSONTags(String, Type, Vec<GoStruct>),
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
