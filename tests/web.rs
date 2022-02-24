//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use wordsearch_wasm::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_rand_char() {
    let c_1: char = rand_char();
    let c_2: char = rand_char();
    assert_ne!(c_1, c_2);
}
