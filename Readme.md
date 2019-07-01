# typed-array

[![](https://img.shields.io/crates/v/typed-array.svg)][crates-io]
[![](https://docs.rs/typed-array/badge.svg)][api-docs]

A wrapper for all typed array types from `js_sys` that implements common
functionality and useful conversions.

All typed arrays in Javascript have properties like [`buffer`], [`subarray`]
and [`byte_length`] in common. However, they can be tricky to work with if
the variant is not known. This wrapper type makes it easy to use all typed
arrays in the way you'd expect.
  
# Examples
```rust
use typed_array::TypedArray;
use js_sys::Uint8Array;

fn length_of_typed_array<T: Into<TypedArray>>(typed_array: T) -> u32 {
    typed_array.into().length()
}

assert_eq!(
    length_of_typed_array(Uint8Array::new(&10.into())),
    10
);
```

# Features
The following common functionality between typed arrays is implemented:

- [`buffer`]
- [`subarray`]
- [`slice`]
- [`length`]
- [`byte_length`]
- [`byte_offset`]
- [`set`]

Additionally, conversions are easy:

- `From<X> for TypedArray`
- `TryFrom<TypedArray> for X`
- `TryFrom<JsValue> for TypedArray`
- `AsRef<JsValue> for TypedArray`
- `AsRef<Object> for TypedArray`
- [`has_type`], analogous to `JsCast::has_type`
- [`dyn_into`], analogous to `JsCast::dyn_into`

[`buffer`]: enum.TypedArray.html#method.buffer
[`subarray`]: enum.TypedArray.html#method.subarray
[`slice`]: enum.TypedArray.html#method.slice
[`length`]: enum.TypedArray.html#method.length
[`byte_length`]: enum.TypedArray.html#method.byte_length
[`byte_offset`]: enum.TypedArray.html#method.byte_offset
[`set`]: enum.TypedArray.html#method.set
[`has_type`]: enum.TypedArray.html#method.has_type
[`dyn_into`]: enum.TypedArray.html#method.dyn_into
[crates-io]: https://crates.io/crates/typed-array
[api-docs]: https://docs.rs/typed-array/0.1.0/typed_array/