use std::collections::HashMap;

use termion::event::Key;

use crate::cli::{MultiSentakuAction, SingleSentakuCli};

pub mod cli;

/// Error during `wait_for_input`
#[derive(Debug)]
pub enum SentakuError {
    EmptyList,
    Canceled,
    IoError(std::io::Error),
}

impl std::convert::From<std::io::Error> for SentakuError {
    fn from(e: std::io::Error) -> SentakuError {
        SentakuError::IoError(e)
    }
}

/// Structure for each item to be chosen by the user
/// `label` will be displayed and `value` will be returned when the user select the item
pub struct SentakuItem<T> {
    label: String,
    value: T,
}

impl SentakuItem<String> {
    /// construct `SentakuItem`. `value` will be the same as `label`.
    pub fn from_str(label: &str) -> Self {
        SentakuItem {
            label: String::from(label),
            value: String::from(label),
        }
    }
}

pub enum SentakuAction<'a, T> {
    Up,
    Down,
    Cancel,
    Select,
    Action(Box<dyn Fn(&'a T) + 'a>),
}

impl<T> SentakuItem<T> {
    /// construct `SentakuItem`.
    pub fn new(label: &str, value: T) -> Self {
        SentakuItem {
            label: String::from(label),
            value,
        }
    }
}

/// Get default keymap
/// `Key::Up`: move cursor up
/// `Key::Down`: move cursor down
/// `Key::Char('\n')`: select current item
/// `Key::Ctrl('c')`: cancel current selection
pub fn get_default_keymap<'a, T>() -> HashMap<Key, SentakuAction<'a, T>> {
    let mut result = HashMap::new();
    result.insert(Key::Up, SentakuAction::Up);
    result.insert(Key::Down, SentakuAction::Down);
    result.insert(Key::Char('k'), SentakuAction::Up);
    result.insert(Key::Char('j'), SentakuAction::Down);
    result.insert(Key::Char('\n'), SentakuAction::Select);
    result.insert(Key::Ctrl('c'), SentakuAction::Cancel);

    result
}

/// Get default keymap for multi select
/// `Key::Up`: move cursor up
/// `Key::Down`: move cursor down
/// `Key::Char('\n')`: select current item
/// `Key::Ctrl('c')`: cancel current selection
pub fn get_default_multi_sentaku_keymap<'a, T>() -> HashMap<Key, MultiSentakuAction<'a, T>> {
    let mut result = HashMap::new();
    result.insert(Key::Up, MultiSentakuAction::Up);
    result.insert(Key::Down, MultiSentakuAction::Down);
    result.insert(Key::Char('k'), MultiSentakuAction::Up);
    result.insert(Key::Char('j'), MultiSentakuAction::Down);
    result.insert(Key::Char(' '), MultiSentakuAction::Select);
    result.insert(Key::Char('\n'), MultiSentakuAction::Finish);
    result.insert(Key::Ctrl('c'), MultiSentakuAction::Cancel);

    result
}

/// Wait for user input and return an item user selects
/// If the user cancels the input or error happens in stdio, it returns `SentakuError`.
/// If `items` is an empty list, it also returns `SentakuError`.
pub fn wait_for_input_with_keymap<'a, T: Clone>(
    stdin: &mut std::io::Stdin,
    items: &'a Vec<SentakuItem<T>>,
    keymap: HashMap<Key, SentakuAction<'a, T>>,
) -> Result<T, SentakuError> {
    let cli = SingleSentakuCli::with_keymap(items, keymap);
    cli.wait_for_input(stdin)
}

/// Wait for user input and return an item user selects with default keymap.
/// See also `default_keymap`
pub fn wait_for_input<T: Clone>(
    stdin: &mut std::io::Stdin,
    items: &Vec<SentakuItem<T>>,
) -> Result<T, SentakuError> {
    wait_for_input_with_keymap(stdin, items, get_default_keymap())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
