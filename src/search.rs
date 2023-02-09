use std::path::Path;

use crate::codepairs::RAW_PAIRS;
use crate::suggestion::Suggestion;
use bincode;
use bk_tree::{metrics, BKTree};

pub fn fill_bk_tree() {
    // step #1: initialize BK-tree
    let mut tree: BKTree<Suggestion> = BKTree::new(metrics::Levenshtein);

    // step #2: insert each pair into BK-tree
    for pair in RAW_PAIRS {
        println!("{:?}", pair);
        let suggestion = Suggestion {
            description: pair.0.to_string(),
            emoji: pair.1.to_string(),
        };
        tree.add(suggestion);
    }

    // step #3: serialize tree into binary format using `bincode`
    let encoded_tree: Vec<u8> = bincode::serialize(&tree).unwrap();

    // step #4: save bytes to file
    let path: &Path = Path::new("./emojitree.raw");
    std::fs::write(path, encoded_tree).unwrap();
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::codepairs::RAW_PAIRS;
    use crate::suggestion::Suggestion;
    use bincode;
    use bk_tree::BKTree;

    #[test]
    fn test_find_exaxt() {
        // step #1: open file
        let path: &Path = Path::new("./emojitree.raw");
        let encoded_tree = std::fs::read(path).unwrap();

        // step #2: decode BKTree
        let tree: BKTree<Suggestion> = bincode::deserialize(&encoded_tree[..]).unwrap();

        for pair in RAW_PAIRS {
            let result = tree.find_exact(&Suggestion {
                description: pair.0.to_string(),
                emoji: "".to_string(),
            });

            let unwrapped_result = result.expect(&format!("found {} emoji", pair.1));
            assert_eq!(pair.1, unwrapped_result.emoji);
        }
    }
}
