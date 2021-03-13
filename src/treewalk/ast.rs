use crate::data_types::Type;
use std::rc::Rc;

#[derive(Debug)]
pub struct StructDefinition {
    pub name: String,
    pub body: GoStruct,
}

#[derive(Debug)]
pub enum GoStruct {
    FieldNameOnly(String),
    FieldNameWithTypeOnly(String, Type),
    FieldWithJSONTags(String, Type, Vec<GoStruct>),
    FieldWithList(String),
    FieldWithListAndType(String, Type),
    FieldWithIdentifierTypeOnly(String, String),
    FieldWithListTypeAndJSONTags(String, Type, Vec<GoStruct>),
    FieldWithCustomListIdentifier(String, String),
    FieldWithCustomListIdentifierAndJSONTags(String, String, Vec<GoStruct>),
    FieldWithIdentifierAndJSONTags(String, String, Vec<GoStruct>),
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
