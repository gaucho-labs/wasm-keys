use crate::{
    hotkey::{Hotkey, KeyboardModifiers},
    Key, KEY_MAP,
};

pub fn translate_to_key(key_string: &str) -> Key {
    match KEY_MAP.get(&key_string) {
        Some(key) => *key,
        None => {
            if key_string.len() == 1 {
                key_string
                    .chars()
                    .next()
                    .map(Key::CharValue)
                    .unwrap_or(Key::Unidentified)
            } else {
                Key::Unidentified
            }
        }
    }
}

pub fn translate_to_hotkey(key_combination: &str) -> Hotkey {
    let parts = key_combination.split('+').map(str::trim);

    let mut modifiers = KeyboardModifiers::default();
    let mut keys = Vec::new();

    for part in parts {
        match part {
            p if p.eq_ignore_ascii_case("control") || p.eq_ignore_ascii_case("ctrl") => {
                modifiers.ctrl = true
            }
            p if p.eq_ignore_ascii_case("alt") || p.eq_ignore_ascii_case("option") => {
                modifiers.alt = true
            }
            p if p.eq_ignore_ascii_case("meta")
                || p.eq_ignore_ascii_case("command")
                || p.eq_ignore_ascii_case("cmd")
                || p.eq_ignore_ascii_case("super")
                || p.eq_ignore_ascii_case("win") =>
            {
                modifiers.meta = true
            }
            p if p.eq_ignore_ascii_case("shift") => modifiers.shift = true,
            key => {
                keys.push(translate_to_key(key));
            }
        }
    }

    Hotkey { keys, modifiers }
}
