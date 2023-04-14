mod carousel;
mod clipboard;
mod codepairs;
mod emoji;
mod pair;

use bk_tree::BKTree;
use carousel::EmojiCarousel;
use emoji::Emoji;
use pair::EmojiPair;

fn main() {
    // step #1: get the raw bytes from the embedded file
    let emoji_file = Emoji::get("static/emojitree.raw").unwrap();
    let encoded_tree = emoji_file.data.as_ref();

    // step #2: decode BKTree
    let tree: BKTree<EmojiPair> = bincode::deserialize(encoded_tree).unwrap();

    let mut viewer = EmojiCarousel::new(tree);
    viewer.run();
}
