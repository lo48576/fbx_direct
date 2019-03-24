//! Contains common types for reader and writer.
use base64;

use std::borrow::Cow;

/// Format of FBX data.
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
    pub fn borrow(&self) -> Property<'_> {
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

    /// Safe conversion.
    ///
    /// Tries to convert property value into specific type without data loss.
    pub fn get_bool(&self) -> Option<bool> {
        match *self {
            OwnedProperty::Bool(v) => Some(v),
            _ => None,
        }
    }

    /// Safe conversion.
    ///
    /// Tries to convert property value into specific type without data loss.
    pub fn into_bool(self) -> Result<bool, Self> {
        match self {
            OwnedProperty::Bool(v) => Ok(v),
            v => Err(v),
        }
    }

    /// Safe conversion.
    ///
    /// Tries to convert property value into specific type without data loss.
    pub fn get_i16(&self) -> Option<i16> {
        match *self {
            OwnedProperty::Bool(v) => Some(if v { 1 } else { 0 }),
            OwnedProperty::I16(v) => Some(v),
            _ => None,
        }
    }

    /// Safe conversion.
    ///
    /// Tries to convert property value into specific type without data loss.
    pub fn into_i16(self) -> Result<i16, Self> {
        match self {
            OwnedProperty::Bool(v) => Ok(if v { 1 } else { 0 }),
            OwnedProperty::I16(v) => Ok(v),
            v => Err(v),
        }
    }

    /// Safe conversion.
    ///
    /// Tries to convert property value into specific type without data loss.
    pub fn get_i32(&self) -> Option<i32> {
        match *self {
            OwnedProperty::Bool(v) => Some(if v { 1 } else { 0 }),
            OwnedProperty::I16(v) => Some(i32::from(v)),
            OwnedProperty::I32(v) => Some(v),
            _ => None,
        }
    }

    /// Safe conversion.
    ///
    /// Tries to convert property value into specific type without data loss.
    pub fn into_i32(self) -> Result<i32, Self> {
        match self {
            OwnedProperty::Bool(v) => Ok(if v { 1 } else { 0 }),
            OwnedProperty::I16(v) => Ok(i32::from(v)),
            OwnedProperty::I32(v) => Ok(v),
            v => Err(v),
        }
    }

    /// Safe conversion.
    ///
    /// Tries to convert property value into specific type without data loss.
    pub fn get_i64(&self) -> Option<i64> {
        match *self {
            OwnedProperty::Bool(v) => Some(if v { 1 } else { 0 }),
            OwnedProperty::I16(v) => Some(i64::from(v)),
            OwnedProperty::I32(v) => Some(i64::from(v)),
            OwnedProperty::I64(v) => Some(v),
            _ => None,
        }
    }

    /// Safe conversion.
    ///
    /// Tries to convert property value into specific type without data loss.
    pub fn into_i64(self) -> Result<i64, Self> {
        match self {
            OwnedProperty::Bool(v) => Ok(if v { 1 } else { 0 }),
            OwnedProperty::I16(v) => Ok(i64::from(v)),
            OwnedProperty::I32(v) => Ok(i64::from(v)),
            OwnedProperty::I64(v) => Ok(v),
            v => Err(v),
        }
    }

    /// Safe conversion.
    ///
    /// Tries to convert property value into specific type without data loss.
    pub fn get_f32(&self) -> Option<f32> {
        match *self {
            OwnedProperty::F32(v) => Some(v),
            OwnedProperty::F64(v) => Some(v as f32),
            _ => None,
        }
    }

    /// Safe conversion.
    ///
    /// Tries to convert property value into specific type without data loss.
    pub fn into_f32(self) -> Result<f32, Self> {
        match self {
            OwnedProperty::F32(v) => Ok(v),
            OwnedProperty::F64(v) => Ok(v as f32),
            v => Err(v),
        }
    }

    /// Safe conversion.
    ///
    /// Tries to convert property value into specific type without data loss.
    pub fn get_f64(&self) -> Option<f64> {
        match *self {
            OwnedProperty::F32(v) => Some(f64::from(v)),
            OwnedProperty::F64(v) => Some(v),
            _ => None,
        }
    }

    /// Safe conversion.
    ///
    /// Tries to convert property value into specific type without data loss.
    pub fn into_f64(self) -> Result<f64, Self> {
        match self {
            OwnedProperty::F32(v) => Ok(f64::from(v)),
            OwnedProperty::F64(v) => Ok(v),
            v => Err(v),
        }
    }

    /// Safe conversion.
    ///
    /// Tries to convert property value into specific type without data loss.
    pub fn get_vec_bool(&self) -> Option<&[bool]> {
        match *self {
            OwnedProperty::VecBool(ref v) => Some(&v[..]),
            _ => None,
        }
    }

    /// Safe conversion.
    ///
    /// Tries to convert property value into specific type without data loss.
    pub fn into_vec_bool(self) -> Result<Vec<bool>, Self> {
        match self {
            OwnedProperty::VecBool(v) => Ok(v),
            v => Err(v),
        }
    }

    /// Safe conversion.
    ///
    /// Tries to convert property value into specific type without data loss.
    pub fn get_vec_i32(&self) -> Option<Cow<'_, [i32]>> {
        match *self {
            OwnedProperty::VecBool(ref v) => Some(Cow::Owned(
                v.iter().map(|&v| if v { 1 } else { 0 }).collect(),
            )),
            OwnedProperty::VecI32(ref v) => Some(Cow::Borrowed(&v)),
            _ => None,
        }
    }

    /// Safe conversion.
    ///
    /// Tries to convert property value into specific type without data loss.
    pub fn into_vec_i32(self) -> Result<Vec<i32>, Self> {
        match self {
            OwnedProperty::VecBool(v) => Ok(v.into_iter().map(|v| if v { 1 } else { 0 }).collect()),
            OwnedProperty::VecI32(v) => Ok(v),
            v => Err(v),
        }
    }

    /// Safe conversion.
    ///
    /// Tries to convert property value into specific type without data loss.
    pub fn get_vec_i64(&self) -> Option<Cow<'_, [i64]>> {
        match *self {
            OwnedProperty::VecBool(ref v) => Some(Cow::Owned(
                v.iter().map(|&v| if v { 1 } else { 0 }).collect(),
            )),
            OwnedProperty::VecI32(ref v) => Some(Cow::Owned(v.iter().map(|&v| i64::from(v)).collect())),
            OwnedProperty::VecI64(ref v) => Some(Cow::Borrowed(&v)),
            _ => None,
        }
    }

    /// Safe conversion.
    ///
    /// Tries to convert property value into specific type without data loss.
    pub fn into_vec_i64(self) -> Result<Vec<i64>, Self> {
        match self {
            OwnedProperty::VecBool(v) => Ok(v.into_iter().map(|v| if v { 1 } else { 0 }).collect()),
            OwnedProperty::VecI32(v) => Ok(v.into_iter().map(i64::from).collect()),
            OwnedProperty::VecI64(v) => Ok(v),
            v => Err(v),
        }
    }

    /// Safe conversion.
    ///
    /// Tries to convert property value into specific type without data loss.
    pub fn get_vec_f32(&self) -> Option<Cow<'_, [f32]>> {
        match *self {
            OwnedProperty::VecF32(ref v) => Some(Cow::Borrowed(&v)),
            OwnedProperty::VecF64(ref v) => Some(Cow::Owned(v.iter().map(|&v| v as f32).collect())),
            _ => None,
        }
    }

    /// Safe conversion.
    ///
    /// Tries to convert property value into specific type without data loss.
    pub fn into_vec_f32(self) -> Result<Vec<f32>, Self> {
        match self {
            OwnedProperty::VecF32(v) => Ok(v),
            OwnedProperty::VecF64(v) => Ok(v.into_iter().map(|v| v as f32).collect()),
            v => Err(v),
        }
    }

    /// Safe conversion.
    ///
    /// Tries to convert property value into specific type without data loss.
    pub fn get_vec_f64(&self) -> Option<Cow<'_, [f64]>> {
        match *self {
            OwnedProperty::VecF32(ref v) => Some(Cow::Owned(v.iter().map(|&v| f64::from(v)).collect())),
            OwnedProperty::VecF64(ref v) => Some(Cow::Borrowed(&v)),
            _ => None,
        }
    }

    /// Safe conversion.
    ///
    /// Tries to convert property value into specific type without data loss.
    pub fn into_vec_f64(self) -> Result<Vec<f64>, Self> {
        match self {
            OwnedProperty::VecF32(v) => Ok(v.into_iter().map(f64::from).collect()),
            OwnedProperty::VecF64(v) => Ok(v),
            v => Err(v),
        }
    }

    /// Get string value if possible.
    pub fn get_string(&self) -> Option<&String> {
        match *self {
            OwnedProperty::String(ref v) => Some(&v),
            _ => None,
        }
    }

    /// Get string value if possible.
    pub fn into_string(self) -> Result<String, Self> {
        match self {
            OwnedProperty::String(v) => Ok(v),
            v => Err(v),
        }
    }

    /// Get binary value if possible.
    pub fn get_binary(&self, from_string: bool) -> Option<Cow<'_, [u8]>> {
        match *self {
            OwnedProperty::String(ref v) => {
                // In ASCII FBX, binary value is represented as base64-encoded string.
                if from_string {
                    base64::decode(v).ok().map(Cow::Owned)
                } else {
                    None
                }
            }
            OwnedProperty::Binary(ref v) => Some(Cow::Borrowed(&v[..])),
            _ => None,
        }
    }

    /// Get binary value if possible.
    pub fn into_binary(self, from_string: bool) -> Result<Vec<u8>, Self> {
        match self {
            OwnedProperty::String(v) => {
                // In ASCII FBX, binary value is represented as base64-encoded string.
                if from_string {
                    base64::decode(&v).or_else(|_| Err(OwnedProperty::String(v)))
                } else {
                    Err(OwnedProperty::String(v))
                }
            }
            OwnedProperty::Binary(v) => Ok(v),
            v => Err(v),
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

impl<'a> Property<'a> {
    /// Safe conversion.
    ///
    /// Tries to convert property value into specific type without data loss.
    pub fn get_bool(&self) -> Option<bool> {
        match *self {
            Property::Bool(v) => Some(v),
            _ => None,
        }
    }

    /// Safe conversion.
    ///
    /// Tries to convert property value into specific type without data loss.
    pub fn get_i16(&self) -> Option<i16> {
        match *self {
            Property::Bool(v) => Some(if v { 1 } else { 0 }),
            Property::I16(v) => Some(v),
            _ => None,
        }
    }

    /// Safe conversion.
    ///
    /// Tries to convert property value into specific type without data loss.
    pub fn get_i32(&self) -> Option<i32> {
        match *self {
            Property::Bool(v) => Some(if v { 1 } else { 0 }),
            Property::I16(v) => Some(i32::from(v)),
            Property::I32(v) => Some(v),
            _ => None,
        }
    }

    /// Safe conversion.
    ///
    /// Tries to convert property value into specific type without data loss.
    pub fn get_i64(&self) -> Option<i64> {
        match *self {
            Property::Bool(v) => Some(if v { 1 } else { 0 }),
            Property::I16(v) => Some(i64::from(v)),
            Property::I32(v) => Some(i64::from(v)),
            Property::I64(v) => Some(v),
            _ => None,
        }
    }

    /// Safe conversion.
    ///
    /// Tries to convert property value into specific type without data loss.
    pub fn get_f32(&self) -> Option<f32> {
        match *self {
            Property::F32(v) => Some(v),
            Property::F64(v) => Some(v as f32),
            _ => None,
        }
    }

    /// Safe conversion.
    ///
    /// Tries to convert property value into specific type without data loss.
    pub fn get_f64(&self) -> Option<f64> {
        match *self {
            Property::F32(v) => Some(f64::from(v)),
            Property::F64(v) => Some(v),
            _ => None,
        }
    }

    /// Safe conversion.
    ///
    /// Tries to convert property value into specific type without data loss.
    pub fn get_vec_bool(&self) -> Option<&[bool]> {
        match *self {
            Property::VecBool(v) => Some(v),
            _ => None,
        }
    }

    /// Safe conversion.
    ///
    /// Tries to convert property value into specific type without data loss.
    pub fn get_vec_i32(&self) -> Option<Cow<'_, [i32]>> {
        match *self {
            Property::VecBool(v) => Some(Cow::Owned(
                v.iter().map(|&v| if v { 1 } else { 0 }).collect(),
            )),
            Property::VecI32(v) => Some(Cow::Borrowed(v)),
            _ => None,
        }
    }

    /// Safe conversion.
    ///
    /// Tries to convert property value into specific type without data loss.
    pub fn get_vec_i64(&self) -> Option<Cow<'_, [i64]>> {
        match *self {
            Property::VecBool(v) => Some(Cow::Owned(
                v.iter().map(|&v| if v { 1 } else { 0 }).collect(),
            )),
            Property::VecI32(v) => Some(Cow::Owned(v.iter().map(|&v| i64::from(v)).collect())),
            Property::VecI64(v) => Some(Cow::Borrowed(v)),
            _ => None,
        }
    }

    /// Safe conversion.
    ///
    /// Tries to convert property value into specific type without data loss.
    pub fn get_vec_f32(&self) -> Option<Cow<'_, [f32]>> {
        match *self {
            Property::VecF32(v) => Some(Cow::Borrowed(v)),
            Property::VecF64(v) => Some(Cow::Owned(v.iter().map(|&v| v as f32).collect())),
            _ => None,
        }
    }

    /// Safe conversion.
    ///
    /// Tries to convert property value into specific type without data loss.
    pub fn get_vec_f64(&self) -> Option<Cow<'_, [f64]>> {
        match *self {
            Property::VecF32(v) => Some(Cow::Owned(v.iter().map(|&v| f64::from(v)).collect())),
            Property::VecF64(v) => Some(Cow::Borrowed(v)),
            _ => None,
        }
    }

    /// Get string value if possible.
    pub fn get_string(&self) -> Option<&str> {
        match *self {
            Property::String(v) => Some(v),
            _ => None,
        }
    }

    /// Get binary value if possible.
    pub fn get_binary(&self, from_string: bool) -> Option<Cow<'_, [u8]>> {
        match *self {
            Property::String(v) => {
                // In ASCII FBX, binary value is represented as base64-encoded string.
                if from_string {
                    base64::decode(v).ok().map(Cow::Owned)
                } else {
                    None
                }
            }
            Property::Binary(v) => Some(Cow::Borrowed(v)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod property_tests {
    use super::OwnedProperty;

    #[test]
    fn owned_vec_i32_to_vec_i64() {
        let vec_i32: Vec<i32> = vec![1, -1, 2, -3, 5, -8, 13, -21, 34];
        let vec_i64 = vec_i32.iter().map(|&v| i64::from(v)).collect::<Vec<_>>();
        let src = OwnedProperty::VecI32(vec_i32.clone());
        let dst = src.get_vec_i64().unwrap().into_owned();
        let dst2 = src.into_vec_i64().unwrap();
        assert_eq!(vec_i64, dst);
        assert_eq!(vec_i64, dst2);
    }

    #[test]
    fn borrowed_vec_i32_to_vec_i64() {
        let vec_i32: Vec<i32> = vec![1, -1, 2, -3, 5, -8, 13, -21, 34];
        let vec_i64 = vec_i32.iter().map(|&v| i64::from(v)).collect::<Vec<_>>();
        let src_owned = OwnedProperty::VecI32(vec_i32.clone());
        let src = src_owned.borrow();
        let dst = src.get_vec_i64().unwrap().into_owned();
        assert_eq!(vec_i64, dst);
    }
}
