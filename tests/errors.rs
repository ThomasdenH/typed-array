use typed_array::{TryFromJsValueError, TryFromTypedArrayError};
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn test_try_from_js_value_error_send() {
    fn assert_send<T: Send>() {}
    assert_send::<TryFromJsValueError>();
}

#[wasm_bindgen_test]
fn test_try_from_js_value_error_sync() {
    fn assert_sync<T: Sync>() {}
    assert_sync::<TryFromJsValueError>();
}

#[wasm_bindgen_test]
fn test_try_from_typed_array_error_send() {
    fn assert_send<T: Send>() {}
    assert_send::<TryFromTypedArrayError>();
}

#[wasm_bindgen_test]
fn test_try_from_typed_array_error_sync() {
    fn assert_sync<T: Sync>() {}
    assert_sync::<TryFromTypedArrayError>();
}
