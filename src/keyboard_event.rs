use crate::{hotkey::Hotkey, translate_to_key, Key};
use web_sys::KeyboardEvent;

impl From<KeyboardEvent> for Key {
    fn from(event: KeyboardEvent) -> Self {
        let key_string = event.key();
        translate_to_key(&key_string)
    }
}

#[macro_export]
macro_rules! key {
    ($event:expr) => {{
        $crate::Key::from($event)
    }};
}

impl From<KeyboardEvent> for Hotkey {
    fn from(event: KeyboardEvent) -> Self {
        let key_string = event.key();
        Hotkey::new(&key_string)
    }
}

#[macro_export]
macro_rules! hotkey {
    ($event:expr) => {{
        $crate::Hotkey::from($event)
    }};
}
