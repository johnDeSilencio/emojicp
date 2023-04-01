#[path = "src/pair.rs"]
mod pair;

#[path = "src/codepairs.rs"]
mod codepairs;

use std::path::Path;

use bincode;
use bk_tree::{metrics, BKTree};
use codepairs::RAW_PAIRS;
use pair::EmojiPair;

pub fn fill_bk_tree() {
    // step #1: initialize BK-tree
    let mut tree: BKTree<EmojiPair> = BKTree::new(metrics::Levenshtein);

    // step #2: insert each pair into BK-tree
    for pair in RAW_PAIRS {
        println!("{:?}", pair);
        let suggestion = EmojiPair {
            description: pair.0.to_string(),
            emoji: pair.1.to_string(),
        };
        tree.add(suggestion);
    }

    // step #3: serialize tree into binary format using `bincode`
    let encoded_tree: Vec<u8> = bincode::serialize(&tree).unwrap();

    // step #4: save bytes to file
    let path: &Path = Path::new("./public/static/emojitree.raw");
    std::fs::write(path, encoded_tree).unwrap();
}

fn main() {
    fill_bk_tree();
}
