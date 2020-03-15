use sentaku::{wait_for_input, SentakuError, SentakuItem};
use std::io::stdin;

fn main() {
    let mut stdin = stdin();
    let items = vec!["apple", "banana", "berry"]
        .iter()
        .map(|s| SentakuItem::from_str(s))
        .collect();
    let result = wait_for_input(&mut stdin, &items);
    match result {
        Ok(value) => { println!("{}", value) },
        Err(SentakuError::Canceled) => { println!("Canceled") },
        _ => { println!("Unexpected io error") },
    }
}