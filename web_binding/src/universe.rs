use universe::universe::Universe;
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

use serde::Serialize;


#[derive(Serialize)]

pub struct PublicData {
    result: f64,
}

#[wasm_bindgen]
pub struct WasmUniverse {
    universe: Universe,
}

#[wasm_bindgen]
impl WasmUniverse {

    pub fn new(width: u32, height: u32) -> WasmUniverse {
        WasmUniverse {
            universe: Universe::new(width, height),
        }
    }
    pub fn get_area_object(&self) -> Result<JsValue, serde_wasm_bindgen::Error> {
        let data = PublicData {
            result: self.universe.get_area() as f64,
        };
        serde_wasm_bindgen::to_value(&data)
    }
    pub fn get_area(&self) -> f64 {
        self.universe.get_area() as f64
    }
}
