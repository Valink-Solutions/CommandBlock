use std::collections::HashMap;

#[derive(Debug, PartialEq)]
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
