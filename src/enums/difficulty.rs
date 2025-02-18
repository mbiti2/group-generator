
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