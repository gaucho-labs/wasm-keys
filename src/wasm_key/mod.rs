mod traits;
mod macros;

use crate::{Fluid};
use crate::key_map::str_to_key::KEY_MAP;


// https://developer.mozilla.org/en-US/docs/Web/API/UI_Events/Keyboard_event_key_values
#[derive(Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum WasmKey {
    // char values
    CharValue(char),

    // modifier keys
    Alt,
    AltGraph,
    CapsLock,
    Control,
    Fn,
    FnLock,
    Hyper,
    Meta,
    NumLock,
    ScrollLock,
    Shift,
    Super,
    Symbol,
    SymbolLock,

    // whitespace keys
    Enter,
    Tab,
    Space, // " "

    // Navigation keys
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    End,
    Home,
    PageDown,
    PageUp,

    // editing keys
    Backspace,
    Clear,
    Copy,
    CrSel,
    Cut,
    Delete,
    EraseEof,
    ExSel,
    Insert,
    Paste,
    Redo,
    Undo,

    // UI keys
    Accept,
    Again,
    Attn,
    Cancel,
    ContextMenu,
    Escape,
    Execute,
    Find,
    Finish,
    Help,
    Pause,
    Play,
    Props,
    Select,
    ZoomIn,
    ZoomOut,

    // DEVICE KEYS
    BrightnessDown,
    BrightnessUp,
    Eject,
    LogOff,
    Power,
    PowerOff,
    PrintScreen,
    Hibernate,
    Standby,
    WakeUp,

    // COMMON IME Keys
    AllCandidates,
    Alphanumeric,
    CodeInput,
    Compose,
    Convert,
    Dead,
    FinalMode,
    GroupFirst,
    GroupLast,
    GroupNext,
    GroupPrevious,
    ModeChange,
    NextCandidate,
    NonConvert,
    PreviousCandidate,
    Process,
    SingleCandidate,

    // Korean Keyboard only
    HangulMode,
    HanjaMode,
    JunjaMode,

    // Japanese keyboards
    Eisu,
    Hankaku,
    Hiragana,
    HiraganaKatakana,
    KanaMode,
    KanjiMode,
    Katakana,
    Romaji,
    Zenkaku,
    ZenkakuHanaku,

    Unidentified(String),
}


impl Fluid<WasmKey> for WasmKey {
    fn from_str(key_combination: &str) -> WasmKey {
        match KEY_MAP.get(&key_combination.to_lowercase()) {
            Some(key) => key.clone(),
            None => {
                if key_combination.len() == 1 {
                    match key_combination.chars().next() {
                        Some(c) => WasmKey::CharValue(c),
                        None => WasmKey::Unidentified(key_combination.to_string()),
                    }
                } else {
                    WasmKey::Unidentified(key_combination.to_string())
                }
            }
        }
    }
}

