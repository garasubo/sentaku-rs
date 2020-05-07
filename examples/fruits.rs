use sentaku::{SentakuError, SentakuItem, wait_for_input_with_keymap, get_default_keymap, SentakuAction};
use std::io::stdin;
use termion::event::Key;
use webbrowser;

fn main() {
    let mut stdin = stdin();
    let items = vec!["apple", "banana", "berry"]
        .iter()
        .map(|s| SentakuItem::from_str(s))
        .collect();
    let open_browser = |value| {
        webbrowser::open(&format!("https://crates.io/search?q={}", value)).unwrap();
    };
    let mut keymap = get_default_keymap();
    keymap.insert(Key::Char('o'), SentakuAction::Action(Box::new(open_browser)));
    let result = wait_for_input_with_keymap(&mut stdin, &items, &keymap);
    match result {
        Ok(value) => { println!("{}", value) },
        Err(SentakuError::Canceled) => { println!("Canceled") },
        _ => { println!("Unexpected io error") },
    }
}