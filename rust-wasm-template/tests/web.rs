//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

extern crate rust_wasm_template;
use rust_wasm_template::Universe;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

#[cfg(test)]
pub fn mk_spaceship() -> Universe {
    let mut uni = Universe::new();
    uni.set_width(6);
    uni.set_height(6);
    uni.set_bcells(&[(1, 2), (2,3), (3,1), (3,2), (3,3)]);
    uni
}

#[cfg(test)]
pub fn expect_spaceship() -> Universe {
    let mut uni = Universe::new();
    uni.set_width(6);
    uni.set_height(6);
    uni.set_bcells(&[(2, 1), (2,3), (3,2), (3, 3), (4,2)]);
    uni
}

#[wasm_bindgen_test]
pub fn test_tick() {
    let mut inp = mk_spaceship();
    let expect_out = expect_spaceship();

    inp.tick();

    assert_eq!(&inp.get_cells(), &expect_out.get_cells());
}
