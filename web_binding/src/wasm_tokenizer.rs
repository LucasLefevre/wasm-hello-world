use tokenizer::tokenizer::parallel_tokenize as tok;
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};


#[wasm_bindgen]
pub fn parallel_tokenize(inputs: Vec<String>) -> Result<JsValue, serde_wasm_bindgen::Error> {
    let tokens = tok(inputs);
    serde_wasm_bindgen::to_value(&tokens)
}
