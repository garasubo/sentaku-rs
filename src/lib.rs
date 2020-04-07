use std::io::Write;
use termion::event::Key;
use termion::input::TermRead;
use termion::color;
use termion::cursor;
use termion::clear;
use termion::style::Reset;
use termion::raw::IntoRawMode;

/// Error during `wait_for_input`
#[derive(Debug)]
pub enum SentakuError {
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

/// Wait for user input and return an item user selects
/// If the user cancels the input or error happens in stdio, it returns `SentakuError`
pub fn wait_for_input<T: Clone>(stdin: &mut std::io::Stdin, items: &Vec<SentakuItem<T>>) -> Result<T, SentakuError> {
    let mut stdout = std::io::stdout().into_raw_mode()?;
    let mut pos = 0;
    write!(stdout, "{}", cursor::Hide)?;
    display_items(&mut stdout, items, pos)?;
    let mut canceled = false;
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Down => {
                pos = std::cmp::min(pos + 1, items.len() - 1);
            },
            Key::Up => {
                pos = std::cmp::max(1, pos) - 1;
            },
            Key::Char('\n') => {
                break;
            },
            Key::Ctrl('c') => {
                canceled = true;
                break;
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
