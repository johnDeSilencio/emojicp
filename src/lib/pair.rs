use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmojiPair {
    pub description: String,
    pub emoji: String,
}

impl fmt::Display for EmojiPair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Make sure that all the emojis are column aligned
        let spaces = 20usize.saturating_sub(self.description.len());
        let spaces = " ".repeat(spaces);
        write!(f, "{}{}{}", self.description, spaces, self.emoji)
    }
}

impl AsRef<str> for EmojiPair {
    fn as_ref(&self) -> &str {
        &self.description
    }
}

#[cfg(test)]
mod tests {
    use crate::pair::EmojiPair;

    #[test]
    fn test_emoji_pair_display() {
        let mut pair = EmojiPair {
            description: String::from(""),
            emoji: String::from("🐵"),
        };

        // emojis with short descriptions are displayed properly
        pair.description = String::from("monkey");
        assert_eq!("monkey\t\t\t🐵", format!("{}", pair));

        // emojis with medium descriptions are displayed properly
        pair.description = String::from("cool monkey");
        assert_eq!("cool monkey\t\t🐵", format!("{}", pair));

        // emojis with long descriptions are displayed properly
        pair.description = String::from("very cool monkey");
        assert_eq!("very cool monkey\t🐵", format!("{}", pair));
    }
}
