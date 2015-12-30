//! Contains common types for reader and writer.

/// Format of FBX data
#[derive(Debug, Clone, Copy)]
pub enum FbxFormatType {
    /// Binary FBX, with version (for example, `7400` for FBX 7.4).
    Binary(u32),
    /// ASCII FBX.
    Ascii,
}

/// A property type of the FBX node.
#[derive(Debug, Clone, PartialEq)]
pub enum OwnedProperty {
    /// Boolean.
    Bool(bool),
    /// 2 byte signed integer.
    I16(i16),
    /// 4 byte signed integer.
    I32(i32),
    /// 8 byte signed integer.
    I64(i64),
    /// 4 byte single-precision IEEE 754 floating-point number.
    F32(f32),
    /// 8 byte double-precision IEEE 754 floating-point number.
    F64(f64),
    /// Array of boolean.
    VecBool(Vec<bool>),
    /// Array of 4 byte signed integer.
    VecI32(Vec<i32>),
    /// Array of 8 byte signed integer.
    VecI64(Vec<i64>),
    /// Array of 4 byte single-precision IEEE 754 number.
    VecF32(Vec<f32>),
    /// Array of 8 byte double-precision IEEE 754 number.
    VecF64(Vec<f64>),
    /// String.
    ///
    /// Note that the string can contain special character like `\u{0}`.
    String(String),
    /// Raw binary data.
    Binary(Vec<u8>),
}

impl OwnedProperty {
    pub fn borrow(&self) -> Property {
        use std::borrow::Borrow;
        match *self {
            OwnedProperty::Bool(v) => Property::Bool(v),
            OwnedProperty::I16(v) => Property::I16(v),
            OwnedProperty::I32(v) => Property::I32(v),
            OwnedProperty::I64(v) => Property::I64(v),
            OwnedProperty::F32(v) => Property::F32(v),
            OwnedProperty::F64(v) => Property::F64(v),
            OwnedProperty::VecBool(ref v) => Property::VecBool(&v),
            OwnedProperty::VecI32(ref v) => Property::VecI32(&v),
            OwnedProperty::VecI64(ref v) => Property::VecI64(&v),
            OwnedProperty::VecF32(ref v) => Property::VecF32(&v),
            OwnedProperty::VecF64(ref v) => Property::VecF64(&v),
            OwnedProperty::String(ref v) => Property::String(&v),
            OwnedProperty::Binary(ref v) => Property::Binary(&v),
        }
    }
}

/// A property type of the FBX node.
#[derive(Debug, Clone, PartialEq)]
pub enum Property<'a> {
    /// Boolean.
    Bool(bool),
    /// 2 byte signed integer.
    I16(i16),
    /// 4 byte signed integer.
    I32(i32),
    /// 8 byte signed integer.
    I64(i64),
    /// 4 byte single-precision IEEE 754 floating-point number.
    F32(f32),
    /// 8 byte double-precision IEEE 754 floating-point number.
    F64(f64),
    /// Array of boolean.
    VecBool(&'a [bool]),
    /// Array of 4 byte signed integer.
    VecI32(&'a [i32]),
    /// Array of 8 byte signed integer.
    VecI64(&'a [i64]),
    /// Array of 4 byte single-precision IEEE 754 number.
    VecF32(&'a [f32]),
    /// Array of 8 byte double-precision IEEE 754 number.
    VecF64(&'a [f64]),
    /// String.
    ///
    /// Note that the string can contain special character like `\u{0}`.
    String(&'a str),
    /// Raw binary data.
    Binary(&'a [u8]),
}
