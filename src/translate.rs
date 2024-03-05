use crate::{
    hotkey::{Hotkey, KeyboardModifiers},
    Key, KEY_MAP,
};
use std::collections::HashMap;

pub fn translate_to_key(key_str: &str) -> Key {
    match KEY_MAP.get(&key_str) {
        Some(key) => key.clone(),
        None => {
            if key_str.len() == 1 {
                match key_str.chars().next() {
                    Some(c) => Key::CharValue(c),
                    None => Key::Unidentified(key_str.to_string()),
                }
            } else {
                Key::Unidentified(key_str.to_string())
            }
        }
    }
}

pub fn translate_to_hotkey(key_combination: &str) -> Hotkey {
    let parts = key_combination.split('+').map(str::trim);

    let mut modifiers = KeyboardModifiers::default();
    let mut keys: HashMap<Key, usize> = HashMap::new();

    for part in parts {
        match part {
            p if p.eq_ignore_ascii_case("control") || p.eq_ignore_ascii_case("ctrl") => {
                modifiers.control += 1
            }
            p if p.eq_ignore_ascii_case("alt") || p.eq_ignore_ascii_case("option") => {
                modifiers.alt += 1
            }
            p if p.eq_ignore_ascii_case("meta")
                || p.eq_ignore_ascii_case("command")
                || p.eq_ignore_ascii_case("cmd")
                || p.eq_ignore_ascii_case("super")
                || p.eq_ignore_ascii_case("win") =>
            {
                modifiers.meta += 1
            }
            p if p.eq_ignore_ascii_case("shift") => modifiers.shift += 1,
            key => {
                let count = keys.entry(translate_to_key(key)).or_insert(0);
                *count += 1;
            }
        }
    }

    Hotkey { keys, modifiers }
}
