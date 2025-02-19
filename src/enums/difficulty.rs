use std::fmt::{write, Display};

#[derive(Debug, Clone)]
pub enum Difficulty {
    Hard,
    Medium,
    EAsy
}

impl From<&str> for Difficulty {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "hard" => Difficulty::Hard,
            "medium" => Difficulty::Medium,
            "easy" => Difficulty::EAsy,
            _ => Difficulty::EAsy,
        }
    }
}

impl Into<&str> for Difficulty {
    fn into(self) -> &'static str {
        match self {
            Difficulty::Hard => "hard",
            Difficulty::Medium => "medium",
            Difficulty::EAsy => "easy",
        }
    }
}

impl Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}