#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Type {
    Number,
    String,
    NullString,
    NullNumber,
    Date,
    Boolean,
    Any,
}
