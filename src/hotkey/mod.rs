mod macros;
mod traits;

use crate::Fluid;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use crate::wasm_key::WasmKey;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct KeyboardModifiers {
    pub alt: usize,
    pub control: usize,
    pub meta: usize,
    pub shift: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Hotkey {
    pub modifiers: KeyboardModifiers,
    pub keys: HashMap<WasmKey, usize>,
}

impl Hotkey {
    pub fn from_keys(keys: Vec<WasmKey>) -> Self {
        let mut hotkey = Hotkey::default();

        for key in keys {
            match key {
                WasmKey::Alt => hotkey.modifiers.alt += 1,
                WasmKey::Meta => hotkey.modifiers.meta += 1,
                WasmKey::Control => hotkey.modifiers.control += 1,
                WasmKey::Shift => hotkey.modifiers.shift += 1,
                _ => {
                    let mult = hotkey.keys.entry(key).or_insert(0);
                    *mult += 1;
                }
            }
        }

        hotkey
    }

}

impl Fluid<Hotkey> for Hotkey {
    fn from_str(key_combination: &str) -> Hotkey {
        let parts = key_combination.split('+').map(str::trim);

        let mut modifiers = KeyboardModifiers::default();
        let mut keys: HashMap<WasmKey, usize> = HashMap::new();

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
                    let count = keys.entry(WasmKey::from_str(key)).or_insert(0);
                    *count += 1;
                }
            }
        }

        Hotkey { keys, modifiers }
    }
}

impl Hash for Hotkey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut keys: Vec<_> = self.keys.keys().collect();
        keys.sort();
        keys.hash(state);
    }
}


impl Default for KeyboardModifiers {
    fn default() -> Self {
        KeyboardModifiers {
            alt: 0,
            control: 0,
            meta: 0,
            shift: 0,
        }
    }
}

impl Default for Hotkey {
    fn default() -> Self {
        Hotkey {
            modifiers: KeyboardModifiers::default(),
            keys: HashMap::new(),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_modifiers() {
        let test_cases = vec![
            ((0, 0, 0, 0), Hotkey::from_keys(vec![])),
            (
                (1, 0, 1, 4),
                Hotkey::from_keys(vec![
                    WasmKey::Alt,
                    WasmKey::Meta,
                    WasmKey::Shift,
                    WasmKey::Shift,
                    WasmKey::Shift,
                    WasmKey::Shift,
                ]),
            ),
            ((0, 0, 2, 0), Hotkey::from_keys(vec![WasmKey::Meta, WasmKey::Meta])),
        ];

        for (expected, hotkey) in test_cases {
            let modifiers = hotkey.modifiers;

            match expected {
                (alt, control, meta, shift) => {
                    assert_eq!(
                        KeyboardModifiers {
                            alt,
                            control,
                            meta,
                            shift
                        },
                        modifiers
                    )
                }
            }
        }
    }

    #[test]
    fn test_basic_hotkeys() {
        let test_cases = vec![
            (
                "ctrl+k+arrowup",
                Hotkey::from_keys(vec![WasmKey::Control, WasmKey::CharValue('k'), WasmKey::ArrowUp]),
            ),
            (
                "option+cmd+k",
                Hotkey::from_keys(vec![WasmKey::Alt, WasmKey::Meta, WasmKey::CharValue('k')]),
            ),
            (
                "shift+shift+shift",
                Hotkey::from_keys(vec![WasmKey::Shift, WasmKey::Shift, WasmKey::Shift]),
            ),
            (
                "wef",
                Hotkey::from_keys(vec![WasmKey::Unidentified("wef".to_string())]),
            ),
        ];

        for (input, expected) in test_cases {
            let input_hotkey = Hotkey::from(input);
            assert_eq!(input_hotkey, expected)
        }
    }

    #[test]
    fn test_hotkey_match() {
        let test_cases = vec![
            (
                Hotkey::from("shift+k"),
                Hotkey::from("k + Shift"),
                true
            ),
            (
                Hotkey::from("shift + shift + 7 + b"),
                Hotkey::from("shift + 7 + b"),
                false
            ),
            (
                Hotkey::from("Meta + alt + b"),
                Hotkey::from("alt +meta+b"),
                true
            ),
        ];

        for (input, expected, cond) in test_cases {
            assert_eq!((input == expected), cond)
        }
    }
}
