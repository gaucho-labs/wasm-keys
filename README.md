## *wasm-keys*

This crate delivers compile-time static type bindings for key values in `web-sys::KeyboardEvent`.

```rust
use wasm_keys::WasmKey;

fn SomeComponent() {
    let pressed_keys: Vec<WasmKey> = vec![];
    
    let keydown_listener = wasm_bindgen::closure::Closure::wrap(
        Box::new(move |event: web_sys::KeyboardEvent| { 
            pressed_keys.push(WasmKey::from(event.key()));
        }) as Box<dyn Fn(_)>);
}
```

### Typing
At its core, this crate introduces the `WasmKey` enum, written to reflect every key value listed in the [MDN `KeyboardEvent` documentation](ttps://developer.mozilla.org/en-US/docs/Web/API/UI_Events/Keyboard_event_key_values). In addition, this crate provides a direct mapping from `KeyboardEvent.key()` to `WasmKey` upon **compile-time**.

### Hotkeys
This crate offers a feature to build a `Hotkey`, a type that consists of one or many `WasmKeys`. Specific modifier keys are noted as such.

`Hotkey` implements the `Eq` trait, allowing nice deep equality checks:
```rust
use wasm_keys::{WasmKey, Hotkey};

fn compare(hotkey: Hotkey, keys: Vec<WasmKey>) -> bool {
    hotkey == Hotkey::from_keys(keys);
}
```

### References
- [web-sys documentation](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.KeyboardEvent.html#method.key)
- [MDN `Key` documentation](https://developer.mozilla.org/en-US/docs/Web/API/UI_Events/Keyboard_event_key_values)

### Usages
[leptos-hotkeys](https://leptos-hotkeys.vercel.app/) - a hotkey library for Leptos, a web framework written in Rust.
