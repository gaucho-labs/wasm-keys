pub mod hotkey;
pub mod keyboard_event;
pub mod translate;
pub use keyboard_event::*;
pub use std::fmt;
pub use translate::*;

use phf::{phf_map, Map};

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ParseError {
    UnidentifiedKey(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::UnidentifiedKey(s) => write!(f, "Unidentified key: {}", s),
        }
    }
}

impl std::error::Error for ParseError {}

// https://developer.mozilla.org/en-US/docs/Web/API/UI_Events/Keyboard_event_key_values
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Key {
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

pub static KEY_MAP: Map<&'static str, Key> = phf_map! {
    "alt" => Key::Alt,
    "altgraph" => Key::AltGraph,
    "capslock" => Key::CapsLock,
    "control" => Key::Control,
    "fn" => Key::Fn,
    "fnlock" => Key::FnLock,
    "hyper" => Key::Hyper,
    "meta" => Key::Meta,

    // In Firefox, the Windows key is reported as "OS" instead of as "Meta". This will be changed in Firefox per Firefox bug 1232918
    "os" => Key::Meta, // Note: Adjusted as per note for Firefox
    "numlock" => Key::NumLock,
    "scrolllock" => Key::ScrollLock,
    "shift" => Key::Shift,
    "super" => Key::Super,
    "symbol" => Key::Symbol,
    "symbollock" => Key::SymbolLock,

    "enter" => Key::Enter,
    "tab" => Key::Tab,
    " " => Key::Space,

    "arrowdown" => Key::ArrowDown,
    "arrowleft" => Key::ArrowLeft,
    "arrowright" => Key::ArrowRight,
    "arrowup" => Key::ArrowUp,
    "end" => Key::End,
    "home" => Key::Home,
    "pagedown" => Key::PageDown,
    "pageup" => Key::PageUp,

    "backspace" => Key::Backspace,
    "clear" => Key::Clear,
    "copy" => Key::Copy,
    "crsel" => Key::CrSel,
    "cut" => Key::Cut,
    "delete" => Key::Delete,
    "eraseeof" => Key::EraseEof,
    "exsel" => Key::ExSel,
    "insert" => Key::Insert,
    "paste" => Key::Paste,
    "redo" => Key::Redo,
    "undo" => Key::Undo,

    "accept" => Key::Accept,
    "again" => Key::Again,
    "attn" => Key::Attn,
    "cancel" => Key::Cancel,
    "contextmenu" => Key::ContextMenu,
    "escape" => Key::Escape,
    "execute" => Key::Execute,
    "find" => Key::Find,
    "finish" => Key::Finish,
    "help" => Key::Help,
    "pause" => Key::Pause,
    "play" => Key::Play,
    "props" => Key::Props,
    "select" => Key::Select,
    "zoomin" => Key::ZoomIn,
    "zoomout" => Key::ZoomOut,

    "brightnessdown" => Key::BrightnessDown,
    "brightnessup" => Key::BrightnessUp,
    "eject" => Key::Eject,
    "logoff" => Key::LogOff,
    "power" => Key::Power,
    "poweroff" => Key::PowerOff,
    "printscreen" => Key::PrintScreen,
    "hibernate" => Key::Hibernate,
    "standby" => Key::Standby,
    "wakeup" => Key::WakeUp,

    "allcandidates" => Key::AllCandidates,
    "alphanumeric" => Key::Alphanumeric,
    "codeinput" => Key::CodeInput,
    "compose" => Key::Compose,
    "convert" => Key::Convert,
    "dead" => Key::Dead,
    "finalmode" => Key::FinalMode,
    "groupfirst" => Key::GroupFirst,
    "grouplast" => Key::GroupLast,
    "groupnext" => Key::GroupNext,
    "groupprevious" => Key::GroupPrevious,
    "modechange" => Key::ModeChange,
    "nextcandidate" => Key::NextCandidate,
    "nonconvert" => Key::NonConvert,
    "previouscandidate" => Key::PreviousCandidate,
    "process" => Key::Process,
    "singlecandidate" => Key::SingleCandidate,

    "hangulmode" => Key::HangulMode,
    "hanjamode" => Key::HanjaMode,
    "junjamode" => Key::JunjaMode,

    "eisu" => Key::Eisu,
    "hankaku" => Key::Hankaku,
    "hiragana" => Key::Hiragana,
    "hiraganakatakana" => Key::HiraganaKatakana,
    "kanamode" => Key::KanaMode,
    "kanjimode" => Key::KanjiMode,
    "katakana" => Key::Katakana,
    "romaji" => Key::Romaji,
    "zenkaku" => Key::Zenkaku,
    "zenkakuhanaku" => Key::ZenkakuHanaku,
};
