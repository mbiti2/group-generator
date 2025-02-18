#[derive(Debug, Clone)]
pub enum Labelling {
    Numeric,
    Alphabetic,
    Alphanumeric
}

impl From<&str> for Labelling {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "numeric" => Self::Numeric,
            "alphabetic" => Self::Alphabetic,
            "alphanumeric" => Self::Alphanumeric,
            _ => Self::Numeric,
        }
    }
}

impl Into<&str> for Labelling {
    fn into(self) -> &'static str {
        match self {
            Self::Numeric => "numeric",
            Self::Alphabetic => "alphabetic",
            Self::Alphanumeric => "alphanumeric",
        }
    }
}