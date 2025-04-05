use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "string | number")]
    pub type StringOrNumber;
}

#[wasm_bindgen(js_name = isOdd)]
pub fn is_odd(n: StringOrNumber) -> bool {
    // check type
    if let Some(num) = n.as_f64() {
        is_odd_rs_core::is_odd(num.to_string())
    } else if let Some(str_value) = n.as_string() {
        is_odd_rs_core::is_odd(&str_value)
    } else {
        false
    }
}

#[wasm_bindgen(js_name = isEven)]
pub fn is_even(n: StringOrNumber) -> bool {
    // check type
    if let Some(num) = n.as_f64() {
        is_odd_rs_core::is_even(num.to_string())
    } else if let Some(str_value) = n.as_string() {
        is_odd_rs_core::is_even(&str_value)
    } else {
        false
    }
}
