use web_sys::KeyboardEvent;
use crate::hotkey::Hotkey;
use crate::Fluid;

impl From<KeyboardEvent> for Hotkey {
    fn from(event: KeyboardEvent) -> Self {
        let key_string = event.key();
        Hotkey::from_str(&key_string)
    }
}

impl From<String> for Hotkey {
    fn from(key_combination: String) -> Self {
        Hotkey::from_str(&key_combination)
    }
}

impl From<&str> for Hotkey {
    fn from(key_combination: &str) -> Self {
        Hotkey::from_str(key_combination)
    }
}

/*
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
 */