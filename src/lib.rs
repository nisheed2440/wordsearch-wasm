mod utils;

use wasm_bindgen::prelude::*;
use js_sys::Math::{random, floor};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn rand_char() -> char {
    const LETTERS: &str = "abcdefghijklmnopqrstuvwxyz";
    let random_number: f64 = random();
    let random_index: usize = floor(random_number * LETTERS.len() as f64) as usize;
    LETTERS.chars().nth(random_index).unwrap()
}
