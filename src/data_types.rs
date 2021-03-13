#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Type {
    TypeNumber,
    TypeString,
    TypeNullString,
    TypeNullNumber,
    TypeDate,
    TypeBoolean,
    TypeAny,
}
