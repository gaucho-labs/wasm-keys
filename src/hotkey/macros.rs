#[macro_export]
macro_rules! hotkey {
    ($event:expr) => {{
        $crate::Hotkey::from($event)
    }};
}
