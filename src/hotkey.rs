use crate::{translate_to_hotkey, Key};

#[derive(Debug, PartialEq, Hash, Eq)]
pub struct KeyboardModifiers {
    pub(crate) alt: bool,
    pub(crate) ctrl: bool,
    pub(crate) meta: bool,
    pub(crate) shift: bool,
}

impl Default for KeyboardModifiers {
    fn default() -> Self {
        KeyboardModifiers {
            alt: false,
            ctrl: false,
            meta: false,
            shift: false,
        }
    }
}

#[derive(Debug)]
pub struct Hotkey {
    pub modifiers: KeyboardModifiers,
    pub keys: Vec<Key>,
}

impl Hotkey {
    pub fn new(key_combination: &str) -> Self {
        translate_to_hotkey(key_combination)
    }
}

impl From<&str> for Hotkey {
    fn from(key_combination: &str) -> Self {
        Hotkey::new(key_combination)
    }
}

impl From<String> for Hotkey {
    fn from(key_combination: String) -> Self {
        Hotkey::new(&key_combination)
    }
}
