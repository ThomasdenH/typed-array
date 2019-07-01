use typed_array::TypedArray;
use js_sys::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn test_length() {

    fn length_of_typed_array<T: Into<TypedArray>>(typed_array: T) -> u32 {
        typed_array.into().length()
    }

    assert_eq!(
        length_of_typed_array(Uint8Array::new(&10.into())),
        10
    );
    assert_eq!(
        length_of_typed_array(Int8Array::new(&10.into())),
        10
    );
    assert_eq!(
        length_of_typed_array(Uint8ClampedArray::new(&10.into())),
        10
    );
    assert_eq!(
        length_of_typed_array(Int16Array::new(&10.into())),
        10
    );
    assert_eq!(
        length_of_typed_array(Uint16Array::new(&10.into())),
        10
    );
    assert_eq!(
        length_of_typed_array(Int32Array::new(&10.into())),
        10
    );
    assert_eq!(
        length_of_typed_array(Uint32Array::new(&10.into())),
        10
    );
    assert_eq!(
        length_of_typed_array(Float32Array::new(&10.into())),
        10
    );
    assert_eq!(
        length_of_typed_array(Float64Array::new(&10.into())),
        10
    );
}