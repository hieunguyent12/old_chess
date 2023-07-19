pub mod chess;
pub mod chess2;
pub mod errors;
mod utils;
use std::collections::HashMap;
use std::rc::{self, Rc};

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub struct New {
    // lmao: HashMap<String, u8>,
    test: Vec<u8>,
}
#[wasm_bindgen]
impl New {
    pub fn new() -> New {
        New {
            test: vec![1, 2, 3, 128], // lmao: HashMap::new(),
        }
    }

    pub fn get(&self) -> *const u8 {
        self.test.as_ptr()
    }

    pub fn get_test(&self) -> Vec<u8> {
        self.test.clone()
    }
}

// #[wasm_bindgen]
// pub fn greet() -> [u8; 128] {
//     let board: [u8; 128] = [0; 128];

//     board
// }
