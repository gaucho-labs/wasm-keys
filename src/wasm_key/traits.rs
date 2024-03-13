use web_sys::KeyboardEvent;
use crate::Fluid;
use crate::wasm_key::WasmKey;

impl From<KeyboardEvent> for WasmKey{
    fn from(event: KeyboardEvent) -> Self {
        let key_string = event.key();
        WasmKey::from_str(&key_string)
    }
}

impl From<String> for WasmKey {
    fn from(value: String) -> Self {
        WasmKey::from_str(&value)
    }
}
