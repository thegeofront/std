use core::panic;

use serde::{Serialize, Deserialize};
use serde_json::json;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::{GeoShaderType, PointBuffer, GeoType};

#[wasm_bindgen(inspectable)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[wasm_bindgen]
impl Point {
    
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point { x , y, z }
    }

    pub fn add_num(&mut self, x: f64, y: f64, z: f64) {
        self.x += x;
        self.y += y;
        self.z += z;
    }

    pub fn add(&self, other: &Point) -> Point {
        Point::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z
        )
    }

    pub fn subtract(&self, other: &Point) -> Point {
        Point::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z
        )
    }
}
 
///////////////////////////////////////////////////////////////////////////////////

// impl Typed for Point
#[wasm_bindgen]
impl Point {

    pub fn gf_has_trait_typed() -> bool {
        true
    }
    pub fn gf_is_convertable_to(gt: GeoType) -> bool {
        match gt {
            GeoType::Json | GeoType::Point => true,
            _ => false,
        }
    }
    pub fn gf_default() -> Self {
        Point { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn gf_to_json(&self) -> JsValue {
        JsValue::from_serde(&self).unwrap()
    }

    pub fn gf_from_json(val: &JsValue) -> Self {
        val.into_serde().unwrap()
    }
}

// impl Renderable for Point 
#[wasm_bindgen]
impl Point {

    pub fn gf_has_trait_renderable() -> bool {
        true
    }

    pub fn gf_get_shader_type() -> GeoShaderType {
        GeoShaderType::PointShader
    }

    pub fn gf_get_bounding_box(&self) -> JsValue {
        let value = json!({
            "xa": self.x,
            "ya": self.y,
            "za": self.z,
            "xb": self.x,
            "yb": self.y,
            "zb": self.z,
        });
        JsValue::from_serde(&value).unwrap()
    }

    pub fn gf_get_buffers(&self) -> JsValue {
        let buffer = PointBuffer {
            x: self.x,
            y: self.y,
            z: self.z,
        };
        JsValue::from_serde(&buffer).unwrap()
    }
}

// impl descriptive for Point 
#[wasm_bindgen]
impl Point {

    pub fn gf_has_trait_descriptive() -> bool {
        true
    }

    pub fn gf_get_description() -> JsValue {
        let value = json!({
            "gf_type": "This is a normal, cartesian point in 3D space. It uses f64 internally.",
            "new": "Create a new point.",
            "distance": "The distance between two points",
        });
        JsValue::from_serde(&value).unwrap()
    }
}

// impl Iterable for Point 
#[wasm_bindgen]
impl Point {

    pub fn gf_has_trait_iterable() -> bool {
        true
    }

    pub fn gf_get_base_type() -> GeoType {
        GeoType::Float
    }

    pub fn gf_get_length(&self) -> usize {
        3
    }

    pub fn gf_get_item(&self, index: usize) -> Option<f64> {
        match index {
            0 => Some(self.x),
            1 => Some(self.y),
            2 => Some(self.z),
            _ => None,
        }
    }
}