use sentaku::cli::SingleSentakuCli;
use sentaku::{SentakuAction, SentakuError, SentakuItem};
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
    let mut cli = SingleSentakuCli::new(&items);
    cli.add_key_assign(
        Key::Char('o'),
        SentakuAction::Action(Box::new(open_browser)),
    );
    let result = cli.wait_for_input(&mut stdin);
    match result {
        Ok(value) => println!("{}", value),
        Err(SentakuError::Canceled) => println!("Canceled"),
        _ => println!("Unexpected io error"),
    }
}
