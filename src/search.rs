use std::path::Path;

use crate::codepairs::RAW_PAIRS;
use crate::pair::EmojiPair;
use bincode;
use bk_tree::{metrics, BKTree};

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::codepairs::RAW_PAIRS;
    use crate::emoji::Emoji;
    use crate::pair::EmojiPair;
    use bincode;
    use bk_tree::BKTree;

    #[test]
    fn test_find_exaxt() {
        // step #1: open file
        let path: &Path = Path::new("./emojitree.raw");
        let encoded_tree = std::fs::read(path).unwrap();

        // step #2: decode BKTree
        let tree: BKTree<EmojiPair> = bincode::deserialize(&encoded_tree[..]).unwrap();

        for pair in RAW_PAIRS {
            let result = tree.find_exact(&EmojiPair {
                description: pair.0.to_string(),
                emoji: "".to_string(),
            });

            let unwrapped_result = result.expect(&format!("found {} emoji", pair.1));
            assert_eq!(pair.1, unwrapped_result.emoji);
        }
    }
}
