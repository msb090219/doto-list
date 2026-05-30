use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Doto {
    pub id: usize,
    pub text: String,
    pub completed: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Doto {
    pub fn new(id: usize, text: String) -> Self {
        Self {
            id,
            text,
            completed: false,
            created_at: chrono::Utc::now(),
        }
    }

}

impl fmt::Display for Doto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if self.completed { "[✓]" } else { "[ ]" };
        write!(f, "{} {}", status, self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doto_creation() {
        let doto = Doto::new(1, "Test task".to_string());
        assert_eq!(doto.id, 1);
        assert_eq!(doto.text, "Test task");
        assert!(!doto.completed);
    }

}
