//! A wrapper for all typed array types from `js_sys` that implements common
//! functionality and useful conversions.
//!
//! All typed arrays in Javascript have properties like `buffer`, `subarray`
//! and `byte_length` in common. However, they can be tricky to work with if
//! the variant is not known. This wrapper type makes it easy to use all typed
//! arrays in the way you'd expect.
//!  
//! # Examples
//! ```rust
//! use typed_array::TypedArray;
//! use js_sys::Uint8Array;
//!
//! fn length_of_typed_array<T: Into<TypedArray>>(typed_array: T) -> u32 {
//!     typed_array.into().length()
//! }
//!
//! assert_eq!(
//!     length_of_typed_array(Uint8Array::new(&10.into())),
//!     10
//! );
//! ```
//!
//! # Features
//! The following common functionality between typed arrays is implemented:
//!
//! - `buffer`
//! - `subarray`
//! - `slice`
//! - `length`
//! - `byte_length`
//! - `byte_offset`
//! - `set`
//!
//! Additionally, conversions are easy:
//!
//! - `From<X> for TypedArray`
//! - `TryFrom<TypedArray> for X`
//! - `TryFrom<JsValue> for TypedArray`
//! - `Deref<Target = Object> for TypedArray`
//! - `AsRef<JsValue> for TypedArray`
//! - `AsRef<Object> for TypedArray`
//! - `has_type`, analogous to `JsCast::has_type`
//! - `dyn_into`, analogous to `JsCast::dyn_into`
//!

use core::convert::TryFrom;
use err_derive::*;
use js_sys::{
    ArrayBuffer, Float32Array, Float64Array, Int16Array, Int32Array, Int8Array, Uint16Array,
    Uint32Array, Uint8Array, Uint8ClampedArray,
};
use wasm_bindgen::{JsCast, JsValue};

/// Returned when attempting to convert a `TypedArray` to a specific typed
/// array instance.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default, Error)]
#[error(display = "could not convert TypedArray to typed array instance")]
pub struct TryFromTypedArrayError;

macro_rules! impl_from {
    ($arr:ident) => {
        impl From<$arr> for TypedArray {
            /// Convert the primitive array to a `TypedArray` instance.
            fn from(i: $arr) -> Self {
                TypedArray::$arr(i)
            }
        }

        impl TryFrom<TypedArray> for $arr {
            type Error = TryFromTypedArrayError;
            fn try_from(i: TypedArray) -> Result<Self, Self::Error> {
                if let TypedArray::$arr(inner) = i {
                    Ok(inner)
                } else {
                    Err(TryFromTypedArrayError)
                }
            }
        }
    };
}

#[derive(Clone, Debug)]
pub enum TypedArray {
    Int8Array(Int8Array),
    Uint8Array(Uint8Array),
    Uint8ClampedArray(Uint8ClampedArray),
    Int16Array(Int16Array),
    Uint16Array(Uint16Array),
    Int32Array(Int32Array),
    Uint32Array(Uint32Array),
    Float32Array(Float32Array),
    Float64Array(Float64Array),
}

impl_from!(Int8Array);
impl_from!(Uint8Array);
impl_from!(Uint8ClampedArray);
impl_from!(Int16Array);
impl_from!(Uint16Array);
impl_from!(Int32Array);
impl_from!(Uint32Array);
impl_from!(Float32Array);
impl_from!(Float64Array);

macro_rules! match_every {
    ($to_match:expr, $name:ident, $x:expr) => {
        match $to_match {
            TypedArray::Int8Array($name) => $x,
            TypedArray::Uint8Array($name) => $x,
            TypedArray::Uint8ClampedArray($name) => $x,
            TypedArray::Int16Array($name) => $x,
            TypedArray::Uint16Array($name) => $x,
            TypedArray::Int32Array($name) => $x,
            TypedArray::Uint32Array($name) => $x,
            TypedArray::Float32Array($name) => $x,
            TypedArray::Float64Array($name) => $x,
        }
    };
}

impl TypedArray {
    /// The `buffer` accessor property represents the `ArrayBuffer` referenced
    /// by a `TypedArray` at construction time.
    pub fn buffer(&self) -> ArrayBuffer {
        match_every!(self, i, i.buffer())
    }

    /// The `subarray()` method stores multiple values in the typed array,
    /// reading input values from a specified array.
    pub fn subarray(&self, begin: u32, end: u32) -> Self {
        match_every!(self, i, i.subarray(begin, end).into())
    }

    /// The `slice()` method returns a shallow copy of a portion of a typed
    /// array into a new typed array object. This method has the same algorithm
    /// as `Array.prototype.slice()`.
    pub fn slice(&self, begin: u32, end: u32) -> Self {
        match_every!(self, i, i.slice(begin, end).into())
    }

    /// The `length` accessor property represents the length (in elements) of a
    /// typed array.
    pub fn length(&self) -> u32 {
        match_every!(self, i, i.length())
    }

    /// The `byteLength` accessor property represents the length (in bytes) of a
    /// typed array.
    pub fn byte_length(&self) -> u32 {
        match_every!(self, i, i.byte_length())
    }

    /// The `byteOffset` accessor property represents the offset (in bytes) of a
    /// typed array from the start of its `ArrayBuffer`.
    pub fn byte_offset(&self) -> u32 {
        match_every!(self, i, i.byte_offset())
    }

    /// The `set()` method stores multiple values in the typed array, reading
    /// input values from a specified array.
    pub fn set(&self, src: &JsValue, offset: u32) {
        match_every!(self, i, i.set(src, offset))
    }

    /// Tests whether the provided value is a typed array.
    pub fn has_type(i: JsValue) -> bool {
        i.has_type::<Int8Array>()
            || i.has_type::<Uint8Array>()
            || i.has_type::<Uint8ClampedArray>()
            || i.has_type::<Int16Array>()
            || i.has_type::<Uint16Array>()
            || i.has_type::<Int32Array>()
            || i.has_type::<Uint32Array>()
            || i.has_type::<Float32Array>()
            || i.has_type::<Float64Array>()
    }

    /// If the provided value is a typed array, returns `TypedArray`. If not,
    /// returns the provided value.
    pub fn dyn_into(i: JsValue) -> Result<Self, JsValue> {
        JsCast::dyn_into::<Int8Array>(i)
            .map(TypedArray::from)
            .or_else(|e| JsCast::dyn_into::<Uint8Array>(e).map(TypedArray::from))
            .or_else(|e| JsCast::dyn_into::<Uint8ClampedArray>(e).map(TypedArray::from))
            .or_else(|e| JsCast::dyn_into::<Int16Array>(e).map(TypedArray::from))
            .or_else(|e| JsCast::dyn_into::<Uint16Array>(e).map(TypedArray::from))
            .or_else(|e| JsCast::dyn_into::<Int32Array>(e).map(TypedArray::from))
            .or_else(|e| JsCast::dyn_into::<Uint32Array>(e).map(TypedArray::from))
            .or_else(|e| JsCast::dyn_into::<Float32Array>(e).map(TypedArray::from))
            .or_else(|e| JsCast::dyn_into::<Float64Array>(e).map(TypedArray::from))
    }
}

/// Returned when attempting to convert a `JsValue` to a TypedArray.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default, Error)]
#[error(display = "could not convert JsValue to TypedArray")]
pub struct TryFromJsValueError;

impl TryFrom<JsValue> for TypedArray {
    type Error = TryFromJsValueError;
    fn try_from(i: JsValue) -> Result<Self, Self::Error> {
        TypedArray::dyn_into(i).map_err(|_| TryFromJsValueError)
    }
}

impl core::ops::Deref for TypedArray {
    type Target = js_sys::Object;
    fn deref(&self) -> &Self::Target {
        match_every!(self, i, &*i)
    }
}

impl AsRef<JsValue> for TypedArray {
    fn as_ref(&self) -> &JsValue {
        match_every!(self, i, i.as_ref())
    }
}

impl AsRef<js_sys::Object> for TypedArray {
    fn as_ref(&self) -> &js_sys::Object {
        match_every!(self, i, i.as_ref())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_try_from_js_value_error_send() {
        fn assert_send<T: Send>() {}
        assert_send::<TryFromJsValueError>();
    }

    #[test]
    fn test_try_from_js_value_error_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<TryFromJsValueError>();
    }

    #[test]
    fn test_try_from_typed_array_error_send() {
        fn assert_send<T: Send>() {}
        assert_send::<TryFromTypedArrayError>();
    }

    #[test]
    fn test_try_from_typed_array_error_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<TryFromTypedArrayError>();
    }
}
