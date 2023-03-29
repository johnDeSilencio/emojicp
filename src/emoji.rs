use rust_embed::RustEmbed;

// Embed the rust-bktree in the executable
// so that it can be generated at compile time
// and deserialized from disk at runtime.
#[derive(RustEmbed)]
#[folder = "public/"]
pub struct Emoji;
