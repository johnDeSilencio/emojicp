# emojicp

---

A Rust command line tool to search for emojis by name and copy them to
your clipboard.

### Installing

1. [Download Rust](https://doc.rust-lang.org/cargo/getting-started/installation.html)
2. Clone this repository
3. Navigate to the new folder
4. Run the following command:

```bash
cargo install --path .
```

### Usage

To search for an emoji, simply run `emojicp` like you would for any other
command-line utility. After selecting the emoji you want, pressing `Enter`
will copy the emoji to your clipboard.

To copy the emoji to your clipboard without having to search for it, you can
run the command with the name of the emoji as an argument to the utility, e.g.

```bash
$> emojicp 100 # the emoji 💯 will be copied to your clipboard
```

### Acknowledgements

> "If I have seen further it is by standing on the shoulders of Giants"
> -- Sir Isaac Newton

I'd like to give a big thanks to [Daniel Prilik](https://prilik.com/) whose [compressed emoji shortcodes](https://github.com/daniel5151/compressed-emoji-shortcodes) was a pleasure to read through and whose source code I shamelessly stole to create this CLI tool.