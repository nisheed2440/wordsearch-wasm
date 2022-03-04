//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;
use wordsearch_wasm::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen_test]
fn test_word_search() {
    let words: Vec<String> = vec![
        String::from("hello"),
        String::from("world"),
        String::from("dart"),
        String::from("rust"),
        String::from("wasm"),
    ];
    const width: i32 = 7;
    const height: i32 = 7;
    let mut word_search = WordSearch::new(&words, width, height);
    let output = word_search.create();

    let mut i = 0;
    while i < width {
        let mut row = String::from(" ");
        let mut j = 0;
        while j < height {
            row += " ";
            row += output.puzzle.puzzle[i as usize][j as usize].to_string().as_str();
            j += 1;
        }
        console_log!("{}", row);
        i += 1;
    }
}
