use std::io::Write;
use termion::event::Key;
use termion::input::TermRead;
use termion::color;
use termion::cursor;
use termion::clear;
use termion::style::Reset;
use termion::raw::IntoRawMode;
use std::collections::HashMap;

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

fn display_items<T>(stdout: &mut std::io::Stdout, items: &Vec<SentakuItem<T>>, pos: usize) -> Result<(), std::io::Error> {
    for i in 0..items.len() {
        let item = &items[i];
        if pos == i {
            write!(stdout, "{}{}{}{}\r\n", color::Bg(color::Black), color::Fg(color::White), item.label, Reset)?;
        } else {
            write!(stdout, "{}\r\n", item.label)?;
        }
    }
    stdout.flush()?;
    Ok(())
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
    result.insert(Key::Char('\n'), SentakuAction::Select);
    result.insert(Key::Ctrl('c'), SentakuAction::Cancel);

    result
}

/// Wait for user input and return an item user selects
/// If the user cancels the input or error happens in stdio, it returns `SentakuError`.
/// If `items` is an empty list, it also returns `SentakuError`.
pub fn wait_for_input_with_keymap<'a, T: Clone>(
    stdin: &mut std::io::Stdin,
    items: &'a Vec<SentakuItem<T>>,
    keymap: &HashMap<Key, SentakuAction<'a, T>>,
) -> Result<T, SentakuError> {
    if items.is_empty() {
        return Err(SentakuError::EmptyList);
    }
    let mut stdout = std::io::stdout().into_raw_mode()?;
    let mut pos = 0;
    write!(stdout, "{}", cursor::Hide)?;
    display_items(&mut stdout, items, pos)?;
    let mut canceled = false;
    for c in stdin.keys() {
        let action = keymap.get(&c.unwrap());
        match action {
            Some(SentakuAction::Down) => {
                pos = std::cmp::min(pos + 1, items.len() - 1);
            },
            Some(SentakuAction::Up) => {
                pos = std::cmp::max(1, pos) - 1;
            },
            Some(SentakuAction::Select) => {
                break;
            },
            Some(SentakuAction::Cancel) => {
                canceled = true;
                break;
            },
            Some(SentakuAction::Action(f)) => {
                f(&items[pos].value);
            }
            _ => {}
        }
        write!(stdout, "{}{}", cursor::Up(items.len() as u16), clear::AfterCursor)?;
        display_items(&mut stdout, items, pos)?;
    }

    write!(stdout, "{}", cursor::Show)?;
    stdout.flush().unwrap();
    if canceled {
        Err(SentakuError::Canceled)
    } else {
        Ok(items[pos].value.clone())
    }

}

/// Wait for user input and return an item user selects with default keymap.
/// See also `default_keymap`
pub fn wait_for_input<T: Clone>(stdin: &mut std::io::Stdin, items: &Vec<SentakuItem<T>>) -> Result<T, SentakuError> {
    wait_for_input_with_keymap(stdin, items, &get_default_keymap())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
