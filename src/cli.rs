use std::collections::{HashMap, HashSet};
use std::io::Write;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::style::Reset;
use termion::{clear, color, cursor};

use crate::{
    get_default_keymap, get_default_multi_sentaku_keymap, SentakuAction, SentakuError, SentakuItem,
};

pub struct SingleSentakuCli<'a, T> {
    items: &'a Vec<SentakuItem<T>>,
    keymap: HashMap<Key, SentakuAction<'a, T>>,
}

pub enum MultiSentakuAction<'a, T> {
    Up,
    Down,
    Cancel,
    Select,
    Finish,
    Action(Box<dyn Fn(&Vec<T>) + 'a>),
}

pub struct MultiSentakuCli<'a, T> {
    items: &'a Vec<SentakuItem<T>>,
    keymap: HashMap<Key, MultiSentakuAction<'a, T>>,
}

impl<'a, T: Clone> SingleSentakuCli<'a, T> {
    pub fn new(items: &'a Vec<SentakuItem<T>>) -> Self {
        Self::with_keymap(items, get_default_keymap())
    }

    pub fn with_keymap(
        items: &'a Vec<SentakuItem<T>>,
        keymap: HashMap<Key, SentakuAction<'a, T>>,
    ) -> Self {
        SingleSentakuCli { items, keymap }
    }

    fn display_items(
        &self,
        stdout: &mut std::io::Stdout,
        pos: usize,
    ) -> Result<(), std::io::Error> {
        for i in 0..self.items.len() {
            let item = &self.items[i];
            if pos == i {
                write!(
                    stdout,
                    "{}{}{}{}\r\n",
                    color::Bg(color::Black),
                    color::Fg(color::White),
                    item.label,
                    Reset
                )?;
            } else {
                write!(stdout, "{}\r\n", item.label)?;
            }
        }
        stdout.flush()?;
        Ok(())
    }

    pub fn add_key_assign(&mut self, key: Key, action: SentakuAction<'a, T>) {
        self.keymap.insert(key, action);
    }

    pub fn remove_key_assign(&mut self, key: Key) {
        self.keymap.remove(&key);
    }

    pub fn wait_for_input(&self, stdin: &mut std::io::Stdin) -> Result<T, SentakuError> {
        if self.items.is_empty() {
            return Err(SentakuError::EmptyList);
        }
        if !termion::is_tty(stdin) {
            return Err(SentakuError::NotTTY);
        }
        let mut stdout = std::io::stdout().into_raw_mode()?;
        let mut pos = 0;
        write!(stdout, "{}", cursor::Hide)?;
        self.display_items(&mut stdout, pos)?;
        let mut canceled = false;
        for c in stdin.keys() {
            let action = self.keymap.get(&c.unwrap());
            match action {
                Some(SentakuAction::Down) => {
                    pos = std::cmp::min(pos + 1, self.items.len() - 1);
                }
                Some(SentakuAction::Up) => {
                    pos = std::cmp::max(1, pos) - 1;
                }
                Some(SentakuAction::Select) => {
                    break;
                }
                Some(SentakuAction::Cancel) => {
                    canceled = true;
                    break;
                }
                Some(SentakuAction::Action(f)) => {
                    f(&self.items[pos].value);
                }
                _ => {}
            }
            write!(
                stdout,
                "{}{}",
                cursor::Up(self.items.len() as u16),
                clear::AfterCursor
            )?;
            self.display_items(&mut stdout, pos)?;
        }

        write!(stdout, "{}", cursor::Show)?;
        stdout.flush().unwrap();
        if canceled {
            Err(SentakuError::Canceled)
        } else {
            Ok(self.items[pos].value.clone())
        }
    }
}

impl<'a, T: Clone> MultiSentakuCli<'a, T> {
    pub fn new(items: &'a Vec<SentakuItem<T>>) -> Self {
        Self::with_keymap(items, get_default_multi_sentaku_keymap())
    }

    pub fn with_keymap(
        items: &'a Vec<SentakuItem<T>>,
        keymap: HashMap<Key, MultiSentakuAction<'a, T>>,
    ) -> Self {
        MultiSentakuCli { items, keymap }
    }

    pub fn add_key_assign(&mut self, key: Key, action: MultiSentakuAction<'a, T>) {
        self.keymap.insert(key, action);
    }

    pub fn remove_key_assign(&mut self, key: Key) {
        self.keymap.remove(&key);
    }

    fn display_items(
        &self,
        stdout: &mut std::io::Stdout,
        pos: usize,
        selected: &HashSet<usize>,
    ) -> Result<(), std::io::Error> {
        for i in 0..self.items.len() {
            let item = &self.items[i];
            if pos == i {
                write!(
                    stdout,
                    "{}{}{}{}\r\n",
                    color::Bg(color::Black),
                    color::Fg(color::White),
                    item.label,
                    Reset
                )?;
            } else if selected.contains(&i) {
                write!(
                    stdout,
                    "{}{}{}{}\r\n",
                    color::Bg(color::Blue),
                    color::Fg(color::White),
                    item.label,
                    Reset
                )?;
            } else {
                write!(stdout, "{}\r\n", item.label)?;
            }
        }
        stdout.flush()?;
        Ok(())
    }

    pub fn wait_for_input(&self, stdin: &mut std::io::Stdin) -> Result<Vec<T>, SentakuError> {
        if self.items.is_empty() {
            return Err(SentakuError::EmptyList);
        }
        let mut stdout = std::io::stdout().into_raw_mode()?;
        let mut pos = 0;
        let mut selected = HashSet::new();
        write!(stdout, "{}", cursor::Hide)?;
        self.display_items(&mut stdout, pos, &selected)?;
        let mut canceled = false;
        for c in stdin.keys() {
            let action = self.keymap.get(&c.unwrap());
            match action {
                Some(MultiSentakuAction::Down) => {
                    pos = std::cmp::min(pos + 1, self.items.len() - 1);
                }
                Some(MultiSentakuAction::Up) => {
                    pos = std::cmp::max(1, pos) - 1;
                }
                Some(MultiSentakuAction::Select) => {
                    if selected.contains(&pos) {
                        selected.remove(&pos);
                    } else {
                        selected.insert(pos);
                    }
                }
                Some(MultiSentakuAction::Cancel) => {
                    canceled = true;
                    break;
                }
                Some(MultiSentakuAction::Action(f)) => {
                    let mut values = Vec::new();
                    for i in 0..self.items.len() {
                        if selected.contains(&i) {
                            values.push(self.items[i].value.clone());
                        }
                    }
                    f(&values);
                }
                Some(MultiSentakuAction::Finish) => {
                    break;
                }
                _ => {}
            }
            write!(
                stdout,
                "{}{}",
                cursor::Up(self.items.len() as u16),
                clear::AfterCursor
            )?;
            self.display_items(&mut stdout, pos, &selected)?;
        }

        write!(stdout, "{}", cursor::Show)?;
        stdout.flush().unwrap();
        if canceled {
            Err(SentakuError::Canceled)
        } else {
            let mut values = Vec::new();
            for i in 0..self.items.len() {
                if selected.contains(&i) {
                    values.push(self.items[i].value.clone());
                }
            }
            Ok(values)
        }
    }
}
