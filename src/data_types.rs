use std::fmt;

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

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Number => write!(f, "number"),
            Self::String => write!(f, "string"),
            Self::NullNumber => write!(f, "null | number"),
            Self::NullString => write!(f, "null | string"),
            Self::Date => write!(f, "string"),
            Self::Boolean => write!(f, "boolean"),
            Self::Any => write!(f, "any"),
        }
    }
}
