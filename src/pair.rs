use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmojiPair {
    pub description: String,
    pub emoji: String,
}

impl fmt::Display for EmojiPair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.description.len() >= 8 {
            write!(f, "{}\t{}", self.description, self.emoji)
        } else {
            write!(f, "{}\t\t{}", self.description, self.emoji)
        }
    }
}

impl AsRef<str> for EmojiPair {
    fn as_ref(&self) -> &str {
        &self.description
    }
}
