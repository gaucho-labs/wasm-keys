#[macro_export]
macro_rules! key {
    ($event:expr) => {{
        $crate::Key::from($event)
    }};
}