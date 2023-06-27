use bk_tree::BKTree;
use carousel::EmojiCarousel;
use emoji::Emoji;
use pair::EmojiPair;
use std::env;

fn main() {
    // Default to return successful exit code
    let mut exit_code: i32 = 0;

    // step #1: get the raw bytes from the embedded file
    let emoji_file = Emoji::get("static/emojitree.raw").unwrap();
    let encoded_tree = emoji_file.data.as_ref();

    // step #2: decode BKTree
    let tree: BKTree<EmojiPair> = bincode::deserialize(encoded_tree).unwrap();

    let args: Vec<String> = env::args().collect();

    if let Some(search_term) = args.get(1) {
        let key = EmojiPair {
            description: search_term.to_owned(),
            emoji: "".to_owned(), // doesn't matter for search
        };

        if let Some(result) = tree.find_exact(&key) {
            clipboard::set(result.emoji.to_string());
        } else {
            exit_code = 1;
        }
    } else {
        let mut viewer = EmojiCarousel::new(tree);
        viewer.run();
    }

    std::process::exit(exit_code);
}
