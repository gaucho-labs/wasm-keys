use crate::{translate_to_hotkey, Key, ParseError};
use std::collections::HashMap;

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
    pub keys: HashMap<Key, usize>,
}

impl Hotkey {
    pub fn new(key_combination: &str) -> Self {
        translate_to_hotkey(key_combination)
    }

    pub fn try_new(key_combination: &str) -> Result<Self, ParseError> {
        let hotkey = translate_to_hotkey(key_combination);

        for (key, _) in hotkey.keys.iter() {
            if let Key::Unidentified(unidentified_str) = key {
                return Err(ParseError::UnidentifiedKey(unidentified_str.clone()));
            }
        }

        Ok(hotkey)
    }

    pub fn from_keys(keys: Vec<Key>) -> Self {
        let mut hotkey = Hotkey::default();

        for key in keys {
            match key {
                Key::Alt => hotkey.modifiers.alt += 1,
                Key::Meta => hotkey.modifiers.meta += 1,
                Key::Control => hotkey.modifiers.control += 1,
                Key::Shift => hotkey.modifiers.shift += 1,
                _ => {
                    let mult = hotkey.keys.entry(key).or_insert(0);
                    *mult += 1;
                }
            }
        }

        hotkey
    }

    pub fn try_from_keys(keys: Vec<Key>) -> Result<Self, ParseError> {
        let mut hotkey = Hotkey::default();

        for key in keys {
            match key {
                Key::Unidentified(key) => {
                    return Err(ParseError::UnidentifiedKey(key));
                }
                Key::Alt => hotkey.modifiers.alt += 1,
                Key::Meta => hotkey.modifiers.meta += 1,
                Key::Control => hotkey.modifiers.control += 1,
                Key::Shift => hotkey.modifiers.shift += 1,
                _ => {
                    let mult = hotkey.keys.entry(key).or_insert(0);
                    *mult += 1;
                }
            }
        }

        Ok(hotkey)
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

impl TryFrom<&str> for Hotkey {
    type Error = ParseError;

    fn try_from(key_combination: &str) -> Result<Self, Self::Error> {
        Hotkey::try_new(key_combination)
    }
}

impl TryFrom<String> for Hotkey {
    type Error = ParseError;

    fn try_from(key_combination: String) -> Result<Self, Self::Error> {
        Hotkey::try_new(&key_combination)
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
                    Key::Alt,
                    Key::Meta,
                    Key::Shift,
                    Key::Shift,
                    Key::Shift,
                    Key::Shift,
                ]),
            ),
            ((0, 0, 2, 0), Hotkey::from_keys(vec![Key::Meta, Key::Meta])),
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
                Hotkey::from_keys(vec![Key::Control, Key::CharValue('k'), Key::ArrowUp]),
            ),
            (
                "option+cmd+k",
                Hotkey::from_keys(vec![Key::Alt, Key::Meta, Key::CharValue('k')]),
            ),
            (
                "shift+shift+shift",
                Hotkey::from_keys(vec![Key::Shift, Key::Shift, Key::Shift]),
            ),
            (
                "wef",
                Hotkey::from_keys(vec![Key::Unidentified("wef".to_string())]),
            ),
        ];

        for (input, expected) in test_cases {
            let input_hotkey = Hotkey::new(input);
            assert_eq!(input_hotkey, expected)
        }
    }

    #[test]
    fn test_result_hotkeys() {
        let test_cases = vec![
            (
                "shift+shift+shift",
                Hotkey::try_from_keys(vec![Key::Shift, Key::Shift, Key::Shift]),
                false,
            ),
            (
                "wef",
                Hotkey::try_from_keys(vec![Key::Unidentified("wef".to_string())]),
                true,
            ),
        ];

        for (input, expected, should_err) in test_cases {
            let input_hotkey = Hotkey::new(input);

            if should_err {
                assert!(expected.is_err())
            } else {
                let expected = expected.unwrap();
                assert_eq!(input_hotkey, expected)
            }
        }
    }
}
