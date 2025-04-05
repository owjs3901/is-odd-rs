//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

use is_odd_rs_node::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_is_odd() {
    assert!(is_odd(JsValue::from_str("1").into()));
    assert!(!is_odd(JsValue::from_str("2").into()));
    assert!(!is_odd(JsValue::from_f64(10.0).into()));
    assert!(is_odd(JsValue::from_f64(11.0).into()));
}

#[wasm_bindgen_test]
fn test_is_even() {
    assert!(!is_even(JsValue::from_str("1").into()));
    assert!(is_even(JsValue::from_str("2").into()));
    assert!(is_even(JsValue::from_f64(10.0).into()));
    assert!(!is_even(JsValue::from_f64(11.0).into()));
}
