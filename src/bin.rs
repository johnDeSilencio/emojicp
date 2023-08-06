mod carousel;
mod cli;
mod clipboard;
mod constants;
mod emoji;
mod pair;
mod types;
mod ui;

use crate::cli::*;
use crate::types::{Args, Carousel, EmojiCarousel, EmojiError};
use crate::ui::*;
use bk_tree::BKTree;
use clap::Parser;
use emoji::Emoji;
use pair::EmojiPair;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Parse the command-line input and run the program
    let args = Args::parse();

    Ok(entry(&args)?)
}

/*
fn main() {
    // Default to return successful exit code
    let mut exit_code: i32 = 0;

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
*/