// sin cos tan
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Basics {}

// These functions have magic names, and we need them on the 
// javascript side to make the flowchart work and operate smoothly
#[wasm_bindgen] 
impl Basics {
    pub fn add(a: f64, b: f64) -> f64 { a + b }
    pub fn sub(a: f64, b: f64) -> f64 { a - b }
    pub fn mul(a: f64, b: f64) -> f64 { a * b }
    pub fn div(a: f64, b: f64) -> f64 { a / b }
    
    pub fn sin(a: f64) -> f64 { a.sin() }
    pub fn cos(a: f64) -> f64 { a.cos() }
    pub fn tan(a: f64) -> f64 { a.tan() }
}
