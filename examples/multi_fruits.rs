use sentaku::cli::{MultiSentakuAction, MultiSentakuCli};
use sentaku::{SentakuError, SentakuItem};
use std::io::stdin;
use termion::event::Key;
use webbrowser;

fn main() {
    let mut stdin = stdin();
    let items = vec!["apple", "banana", "berry"]
        .iter()
        .map(|s| SentakuItem::from_str(s))
        .collect();
    let open_browser = |values: &Vec<String>| {
        for value in values.iter() {
            webbrowser::open(&format!("https://crates.io/search?q={}", value)).unwrap();
        }
    };
    let mut cli = MultiSentakuCli::new(&items);
    cli.add_key_assign(
        Key::Char('o'),
        MultiSentakuAction::Action(Box::new(open_browser)),
    );
    let result = cli.wait_for_input(&mut stdin);
    match result {
        Ok(values) => println!("{}", values.join(", ")),
        Err(SentakuError::Canceled) => println!("Canceled"),
        _ => println!("Unexpected io error"),
    }
}
