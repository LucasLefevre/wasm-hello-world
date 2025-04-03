use js_sys::Uint8Array;
use tokenizer::tokenizer::{encode_tokens_to_bytes, parallel_tokenize, tokenize};
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

extern crate console_error_panic_hook;


#[wasm_bindgen]
pub fn wasm_parallel_tokenize(inputs: Vec<String>) -> Result<JsValue, serde_wasm_bindgen::Error> {
    set_panic_hook();
    let tokens = parallel_tokenize(inputs);
    serde_wasm_bindgen::to_value(&tokens)
}

#[wasm_bindgen]
pub fn wasm_batch_tokenize(inputs: Vec<String>) -> Result<JsValue, serde_wasm_bindgen::Error> {
    set_panic_hook();
    let tokens: Vec<_> = inputs.into_iter().map(|f| tokenize(&f)).collect();
    serde_wasm_bindgen::to_value(&tokens)
}

#[wasm_bindgen]
pub fn wasm_buffer_batch_tokenize(inputs: Vec<String>) -> Uint8Array {
    set_panic_hook();
    let tokens_vec: Vec<_> = inputs.into_iter().map(|f| tokenize(&f)).collect();
    let mut buffer = encode_tokens_to_bytes(tokens_vec);
    unsafe { Uint8Array::view_mut_raw(buffer.as_mut_ptr(), buffer.len()) }
}

#[wasm_bindgen]
pub fn wasm_tokenize(input: String) -> Result<JsValue, serde_wasm_bindgen::Error> {
    set_panic_hook();
    let tokens = tokenize(&input);
    serde_wasm_bindgen::to_value(&tokens)
}

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
