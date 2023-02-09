use arboard::Clipboard;

pub fn set_clipboard(selection: String) {
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(selection).unwrap();
}
