mod clipboard;
mod codepairs;
mod search;
mod suggestion;

use bk_tree::BKTree;
use search::fill_bk_tree;
use std::path::Path;
use suggestion::{EmojiCarousel, Suggestion};

fn main() {
    // step #1: open file
    let path: &Path = Path::new("./emojitree.raw");
    let encoded_tree = std::fs::read(path).unwrap();

    // step #2: decode BKTree
    let tree: BKTree<Suggestion> = bincode::deserialize(&encoded_tree[..]).unwrap();

    let mut viewer = EmojiCarousel::new(tree);
    viewer.run();
}
