use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};

#[cfg(feature = "serde")]
use serde::{ser::SerializeMap, Serialize};

#[derive(PartialEq, Clone)]
pub enum NbtValue {
    End,
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    String(String),
    List(Vec<NbtValue>),
    Compound(HashMap<String, NbtValue>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

#[cfg(feature = "serde")]
impl Serialize for NbtValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            NbtValue::End => serializer.serialize_unit(),
            NbtValue::Byte(b) => serializer.serialize_i8(b),
            NbtValue::Short(s) => serializer.serialize_i16(s),
            NbtValue::Int(i) => serializer.serialize_i32(i),
            NbtValue::Long(l) => serializer.serialize_i64(l),
            NbtValue::Float(f) => serializer.serialize_f32(f),
            NbtValue::Double(d) => serializer.serialize_f64(d),
            NbtValue::ByteArray(ref vec) => serializer.collect_seq(vec),
            NbtValue::String(ref s) => serializer.serialize_str(s),
            NbtValue::List(ref vec) => serializer.collect_seq(vec),
            NbtValue::Compound(ref map) => {
                let mut map_serializer = serializer.serialize_map(Some(map.len()))?;
                for (k, v) in map {
                    map_serializer.serialize_entry(k, v)?;
                }
                map_serializer.end()
            }
            NbtValue::IntArray(ref vec) => serializer.collect_seq(vec),
            NbtValue::LongArray(ref vec) => serializer.collect_seq(vec),
        }
    }
}

impl Display for NbtValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NbtValue::End => write!(f, "End"),
            NbtValue::Byte(v) => write!(f, "{}b", v),
            NbtValue::Short(v) => write!(f, "{}s", v),
            NbtValue::Int(v) => write!(f, "{}", v),
            NbtValue::Long(v) => write!(f, "{}l", v),
            NbtValue::Float(v) => write!(f, "{}f", v),
            NbtValue::Double(v) => write!(f, "{}d", v),
            NbtValue::ByteArray(v) => write!(f, "{:?}", v),
            NbtValue::String(v) => write!(f, "{}", v),
            NbtValue::List(v) => write!(f, "{:?}", v),
            NbtValue::Compound(v) => write!(f, "{:?}", v),
            NbtValue::IntArray(v) => write!(f, "{:?}", v),
            NbtValue::LongArray(v) => write!(f, "{:?}", v),
        }
    }
}

impl Debug for NbtValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NbtValue::End => write!(f, "End"),
            NbtValue::Byte(v) => write!(f, "{}b", v),
            NbtValue::Short(v) => write!(f, "{}s", v),
            NbtValue::Int(v) => write!(f, "{}", v),
            NbtValue::Long(v) => write!(f, "{}l", v),
            NbtValue::Float(v) => write!(f, "{}f", v),
            NbtValue::Double(v) => write!(f, "{}d", v),
            NbtValue::ByteArray(v) => write!(f, "{:?}", v),
            NbtValue::String(v) => write!(f, "{}", v),
            NbtValue::List(v) => write!(f, "{:?}", v),
            NbtValue::Compound(v) => write!(f, "{:?}", v),
            NbtValue::IntArray(v) => write!(f, "{:?}", v),
            NbtValue::LongArray(v) => write!(f, "{:?}", v),
        }
    }
}

impl NbtValue {
    pub fn from_binary(value: u8) -> Option<NbtValue> {
        match value {
            0x0 => Some(NbtValue::End),
            0x1 => Some(NbtValue::Byte(0)),
            0x2 => Some(NbtValue::Short(0)),
            0x3 => Some(NbtValue::Int(0)),
            0x4 => Some(NbtValue::Long(0)),
            0x5 => Some(NbtValue::Float(0.0)),
            0x6 => Some(NbtValue::Double(0.0)),
            0x7 => Some(NbtValue::ByteArray(Vec::new())),
            0x8 => Some(NbtValue::String(String::new())),
            0x9 => Some(NbtValue::List(Vec::new())),
            0xA => Some(NbtValue::Compound(HashMap::new())),
            0xB => Some(NbtValue::IntArray(Vec::new())),
            0xC => Some(NbtValue::LongArray(Vec::new())),
            _ => None,
        }
    }

    pub fn to_binary(&self) -> u8 {
        match self {
            NbtValue::End => 0x0,
            NbtValue::Byte(_) => 0x1,
            NbtValue::Short(_) => 0x2,
            NbtValue::Int(_) => 0x3,
            NbtValue::Long(_) => 0x4,
            NbtValue::Float(_) => 0x5,
            NbtValue::Double(_) => 0x6,
            NbtValue::ByteArray(_) => 0x7,
            NbtValue::String(_) => 0x8,
            NbtValue::List(_) => 0x9,
            NbtValue::Compound(_) => 0xA,
            NbtValue::IntArray(_) => 0xB,
            NbtValue::LongArray(_) => 0xC,
        }
    }

    pub fn to_snbt(&self) -> String {
        format!("{:?}", self)
    }
}

#[derive(Debug)]
pub enum Compression {
    Uncompressed,
    Gzip,
    Zlib,
}

#[derive(Debug)]
pub enum NbtError {
    IoError(std::io::Error),
    InvalidTagType(u8),
    InvalidCompression(u8),
    InvalidString(std::string::FromUtf8Error),
    InvalidListType(u8),
    InvalidCompoundType(u8),
    InvalidByteArrayLength(usize),
    InvalidIntArrayLength(usize),
    InvalidLongArrayLength(usize),
}

impl From<std::io::Error> for NbtError {
    fn from(e: std::io::Error) -> NbtError {
        NbtError::IoError(e)
    }
}

#[derive(Debug, PartialEq)]
pub enum Endian {
    Big,
    Little,
}
