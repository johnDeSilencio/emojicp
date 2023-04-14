use clipboard_anywhere::set_clipboard;

pub fn set(selection: String) {
    // If we fail to set to the clipboard, ignore
    // the error and exit the program gracefully
    _ = set_clipboard(selection.as_str());
}
