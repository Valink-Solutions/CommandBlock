use std::{
    collections::HashMap,
    error::Error,
    fmt::{Debug, Display, Formatter},
};

#[cfg(feature = "serde")]
use serde::{ser::SerializeMap, Serialize};

/// Enum representing the different types of compression that can be used.
#[derive(Debug)]
pub enum Compression {
    /// No compression is used.
    Uncompressed,
    /// Gzip compression is used.
    Gzip,
    /// Zlib compression is used.
    Zlib,
}

/// Enum representing the endianness of the data.
///
/// Big is used mostly for Java, while Little is used for everything else, i.e., Bedrock.
#[derive(Debug, PartialEq)]
pub enum Endian {
    Big,
    Little,
}

/// Enum representing the different types of NBT errors that can occur.
#[derive(Debug)]
pub enum NbtError {
    /// Represents an error where an NBT list is empty.
    EmptyList,
    /// Represents an IO error.
    IoError(std::io::Error),
    /// Represents an error where an invalid tag type is used.
    InvalidTagType(u8),
    /// Represents an error where an invalid compression type is used.
    InvalidCompression(u8),
    /// Represents an error where an invalid string is used.
    InvalidString(std::string::FromUtf8Error),
    /// Represents an error where an invalid list type is used.
    InvalidListType(u8),
    /// Represents an error where an invalid compound type is used.
    InvalidCompoundType(u8),
    /// Represents an error where an invalid byte array length is used.
    InvalidByteArrayLength(usize),
    /// Represents an error where an invalid int array length is used.
    InvalidIntArrayLength(usize),
    /// Represents an error where an invalid long array length is used.
    InvalidLongArrayLength(usize),
}

impl From<std::io::Error> for NbtError {
    fn from(e: std::io::Error) -> NbtError {
        NbtError::IoError(e)
    }
}

impl Display for NbtError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match *self {
            NbtError::EmptyList => write!(f, "Empty list"),
            NbtError::IoError(ref err) => write!(f, "IO error: {}", err),
            NbtError::InvalidTagType(ref tag) => write!(f, "Invalid tag type: {}", tag),
            NbtError::InvalidCompression(ref compression) => {
                write!(f, "Invalid compression type: {}", compression)
            }
            NbtError::InvalidString(ref err) => write!(f, "Invalid string: {}", err),
            NbtError::InvalidListType(ref tag) => write!(f, "Invalid list type: {}", tag),
            NbtError::InvalidCompoundType(ref tag) => write!(f, "Invalid compound type: {}", tag),
            NbtError::InvalidByteArrayLength(ref len) => {
                write!(f, "Invalid byte array length: {}", len)
            }
            NbtError::InvalidIntArrayLength(ref len) => {
                write!(f, "Invalid int array length: {}", len)
            }
            NbtError::InvalidLongArrayLength(ref len) => {
                write!(f, "Invalid long array length: {}", len)
            }
        }
    }
}

impl Error for NbtError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            NbtError::EmptyList => None,
            NbtError::IoError(ref err) => Some(err),
            NbtError::InvalidTagType(_) => None,
            NbtError::InvalidCompression(_) => None,
            NbtError::InvalidString(ref err) => Some(err),
            NbtError::InvalidListType(_) => None,
            NbtError::InvalidCompoundType(_) => None,
            NbtError::InvalidByteArrayLength(_) => None,
            NbtError::InvalidIntArrayLength(_) => None,
            NbtError::InvalidLongArrayLength(_) => None,
        }
    }
}

/// Enum representing the different types of NBT (Named Binary Tag) values that can be used.
/// These types are used to represent data in a Minecraft world file.
#[derive(PartialEq, Clone)]
pub enum NbtValue {
    /// Represents the end of a compound NBT tag.
    End,
    /// Represents a byte (8 bits).
    Byte(i8),
    /// Represents a short integer (16 bits).
    Short(i16),
    /// Represents an integer (32 bits).
    Int(i32),
    /// Represents a long integer (64 bits).
    Long(i64),
    /// Represents a floating point number (32 bits).
    Float(f32),
    /// Represents a double precision floating point number (64 bits).
    Double(f64),
    /// Represents an array of bytes.
    ByteArray(Vec<i8>),
    /// Represents a string.
    String(String),
    /// Represents a list of NBT values.
    List(Vec<NbtValue>),
    /// Represents a compound NBT tag, which is a collection of NBT tags.
    Compound(HashMap<String, NbtValue>),
    /// Represents an array of integers (32 bits each).
    IntArray(Vec<i32>),
    /// Represents an array of long integers (64 bits each).
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
    /// Creates a new `NbtValue::Compound` with an empty `HashMap`.
    ///
    /// # Examples
    ///
    /// ```
    /// use commandblock::nbt::NbtValue;
    ///
    /// let nbt = NbtValue::new();
    /// ```
    ///
    /// # Returns
    ///
    /// * `NbtValue::Compound(HashMap::new())`
    pub fn new() -> NbtValue {
        NbtValue::Compound(HashMap::new())
    }

    /// Inserts a key-value pair into the `NbtValue::Compound`.
    ///
    /// # Arguments
    ///
    /// * `key: String` - A string that holds the key.
    /// * `value: T` - A value that can be converted into `NbtValue`.
    ///
    /// # Examples
    ///
    /// ```
    /// use commandblock::nbt::NbtValue;
    ///
    /// let mut nbt = NbtValue::new();
    /// nbt.insert("key".to_string(), 1);
    /// ```
    pub fn insert<T: Into<NbtValue>>(&mut self, key: String, value: T) {
        match self {
            NbtValue::Compound(ref mut map) => {
                map.insert(key, value.into());
            }
            _ => panic!("Cannot insert into non-compound NBT value"),
        }
    }

    /// Returns a reference to the value corresponding to the key in the `NbtValue::Compound`.
    ///
    /// # Arguments
    ///
    /// * `key: &str` - A string slice that holds the key.
    ///
    /// # Examples
    ///
    /// ```
    /// use commandblock::nbt::NbtValue;
    ///
    /// let mut nbt = NbtValue::new();
    /// nbt.insert("key".to_string(), 1);
    /// let value = nbt.get("key");
    /// ```
    ///
    /// # Returns
    ///
    /// * `Some(&NbtValue)` - If the key was present.
    /// * `None` - If the key was not present.
    pub fn get(&self, key: &str) -> Option<&NbtValue> {
        match self {
            NbtValue::Compound(ref map) => map.get(key),
            _ => panic!("Cannot get from non-compound NBT value"),
        }
    }

    /// Returns a mutable reference to the value corresponding to the key in the `NbtValue::Compound`.
    ///
    /// # Arguments
    ///
    /// * `key: &str` - A string slice that holds the key.
    ///
    /// # Examples
    ///
    /// ```
    /// use commandblock::nbt::NbtValue;
    ///
    /// let mut nbt = NbtValue::new();
    /// nbt.insert("key".to_string(), 1);
    /// let value = nbt.get_mut("key");
    /// ```
    ///
    /// # Returns
    ///
    /// * `Some(&mut NbtValue)` - If the key was present.
    /// * `None` - If the key was not present.
    pub fn get_mut(&mut self, key: &str) -> Option<&mut NbtValue> {
        match self {
            NbtValue::Compound(ref mut map) => map.get_mut(key),
            _ => panic!("Cannot get from non-compound NBT value"),
        }
    }

    /// Removes a key from the `NbtValue::Compound`, returning the value at the key if the key was previously in the `NbtValue::Compound`.
    ///
    /// # Arguments
    ///
    /// * `key: &str` - A string slice that holds the key.
    ///
    /// # Examples
    ///
    /// ```
    /// use commandblock::nbt::NbtValue;
    ///
    /// let mut nbt = NbtValue::new();
    /// nbt.insert("key".to_string(), 1);
    /// let value = nbt.remove("key");
    /// ```
    ///
    /// # Returns
    ///
    /// * `Some(NbtValue)` - If the key was present.
    /// * `None` - If the key was not present.
    pub fn remove(&mut self, key: &str) -> Option<NbtValue> {
        match self {
            NbtValue::Compound(ref mut map) => map.remove(key),
            _ => panic!("Cannot remove from non-compound NBT value"),
        }
    }

    /// Returns the number of elements in the `NbtValue::Compound`.
    ///
    /// # Examples
    ///
    /// ```
    /// use commandblock::nbt::NbtValue;
    ///
    /// let mut nbt = NbtValue::new();
    /// nbt.insert("key".to_string(), 1);
    /// let len = nbt.len();
    /// ```
    ///
    /// # Returns
    ///
    /// * `usize` - The number of elements in the `NbtValue::Compound`.
    pub fn len(&self) -> usize {
        match self {
            NbtValue::Compound(ref map) => map.len(),
            _ => panic!("Cannot get length of non-compound NBT value"),
        }
    }

    /// Returns `true` if the `NbtValue::Compound` contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use commandblock::nbt::NbtValue;
    ///
    /// let nbt = NbtValue::new();
    /// let is_empty = nbt.is_empty();
    /// ```
    ///
    /// # Returns
    ///
    /// * `true` - If the `NbtValue::Compound` contains no elements.
    /// * `false` - Otherwise.
    pub fn is_empty(&self) -> bool {
        match self {
            NbtValue::Compound(ref map) => map.is_empty(),
            _ => panic!("Cannot get length of non-compound NBT value"),
        }
    }

    /// Returns a vector containing references to the keys of the `NbtValue::Compound`.
    ///
    /// # Examples
    ///
    /// ```
    /// use commandblock::nbt::NbtValue;
    ///
    /// let mut nbt = NbtValue::new();
    /// nbt.insert("key".to_string(), 1);
    /// let keys = nbt.keys();
    /// ```
    ///
    /// # Returns
    ///
    /// * `Vec<&String>` - A vector containing references to the keys of the `NbtValue::Compound`.
    pub fn keys(&self) -> Vec<&String> {
        match self {
            NbtValue::Compound(ref map) => map.keys().collect(),
            _ => panic!("Cannot get keys of non-compound NBT value"),
        }
    }

    /// Returns a vector containing references to the values of the `NbtValue::Compound`.
    ///
    /// # Examples
    ///
    /// ```
    /// use commandblock::nbt::NbtValue;
    ///
    /// let mut nbt = NbtValue::new();
    /// nbt.insert("key".to_string(), 1);
    /// let values = nbt.values();
    /// ```
    ///
    /// # Returns
    ///
    /// * `Vec<&NbtValue>` - A vector containing references to the values of the `NbtValue::Compound`.
    pub fn values(&self) -> Vec<&NbtValue> {
        match self {
            NbtValue::Compound(ref map) => map.values().collect(),
            _ => panic!("Cannot get values of non-compound NBT value"),
        }
    }

    /// Returns an iterator over the `NbtValue::Compound`.
    ///
    /// # Examples
    ///
    /// ```
    /// use commandblock::nbt::NbtValue;
    ///
    /// let mut nbt = NbtValue::new();
    /// nbt.insert("key".to_string(), 1);
    /// let iter = nbt.iter();
    /// ```
    ///
    /// # Returns
    ///
    /// * `std::collections::hash_map::Iter<String, NbtValue>` - An iterator over the `NbtValue::Compound`.
    pub fn iter(&self) -> std::collections::hash_map::Iter<String, NbtValue> {
        match self {
            NbtValue::Compound(ref map) => map.iter(),
            _ => panic!("Cannot iterate over non-compound NBT value"),
        }
    }

    /// Returns a mutable iterator over the `NbtValue::Compound`.
    ///
    /// # Examples
    ///
    /// ```
    /// use commandblock::nbt::NbtValue;
    ///
    /// let mut nbt = NbtValue::new();
    /// nbt.insert("key".to_string(), 1);
    /// let iter_mut = nbt.iter_mut();
    /// ```
    ///
    /// # Returns
    ///
    /// * `std::collections::hash_map::IterMut<String, NbtValue>` - A mutable iterator over the `NbtValue::Compound`.
    pub fn iter_mut(&mut self) -> std::collections::hash_map::IterMut<String, NbtValue> {
        match self {
            NbtValue::Compound(ref mut map) => map.iter_mut(),
            _ => panic!("Cannot iterate over non-compound NBT value"),
        }
    }

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

// Explicitly implement From for all types that can be converted to NbtValue

impl From<bool> for NbtValue {
    fn from(value: bool) -> Self {
        NbtValue::Byte(if value { 1 } else { 0 })
    }
}

impl From<i8> for NbtValue {
    fn from(value: i8) -> Self {
        NbtValue::Byte(value)
    }
}

impl From<i16> for NbtValue {
    fn from(value: i16) -> Self {
        NbtValue::Short(value)
    }
}

impl From<i32> for NbtValue {
    fn from(value: i32) -> Self {
        NbtValue::Int(value)
    }
}

impl From<i64> for NbtValue {
    fn from(value: i64) -> Self {
        NbtValue::Long(value)
    }
}

impl From<f32> for NbtValue {
    fn from(value: f32) -> Self {
        NbtValue::Float(value)
    }
}

impl From<f64> for NbtValue {
    fn from(value: f64) -> Self {
        NbtValue::Double(value)
    }
}

impl From<Vec<i8>> for NbtValue {
    fn from(value: Vec<i8>) -> Self {
        NbtValue::ByteArray(value)
    }
}

impl From<&str> for NbtValue {
    fn from(item: &str) -> Self {
        NbtValue::String(item.to_string())
    }
}

impl From<&String> for NbtValue {
    fn from(item: &String) -> Self {
        NbtValue::String(item.clone())
    }
}

impl From<String> for NbtValue {
    fn from(item: String) -> Self {
        NbtValue::String(item)
    }
}

impl From<Vec<NbtValue>> for NbtValue {
    fn from(value: Vec<NbtValue>) -> Self {
        NbtValue::List(value)
    }
}

impl From<HashMap<String, NbtValue>> for NbtValue {
    fn from(value: HashMap<String, NbtValue>) -> Self {
        NbtValue::Compound(value)
    }
}

impl From<Vec<i32>> for NbtValue {
    fn from(value: Vec<i32>) -> Self {
        NbtValue::IntArray(value)
    }
}

impl From<Vec<i64>> for NbtValue {
    fn from(value: Vec<i64>) -> Self {
        NbtValue::LongArray(value)
    }
}

// Explicitly implement From for all types that can be converted from NbtValue

impl From<&NbtValue> for i8 {
    fn from(value: &NbtValue) -> Self {
        match value {
            NbtValue::Byte(v) => *v,
            _ => panic!("Cannot convert {:?} to i8", value),
        }
    }
}

impl From<&NbtValue> for i16 {
    fn from(value: &NbtValue) -> Self {
        match value {
            NbtValue::Short(v) => *v,
            _ => panic!("Cannot convert {:?} to i16", value),
        }
    }
}

impl From<&NbtValue> for i32 {
    fn from(value: &NbtValue) -> Self {
        match value {
            NbtValue::Int(v) => *v,
            _ => panic!("Cannot convert {:?} to i32", value),
        }
    }
}

impl From<&NbtValue> for i64 {
    fn from(value: &NbtValue) -> Self {
        match value {
            NbtValue::Long(v) => *v,
            _ => panic!("Cannot convert {:?} to i64", value),
        }
    }
}

impl From<&NbtValue> for f32 {
    fn from(value: &NbtValue) -> Self {
        match value {
            NbtValue::Float(v) => *v,
            _ => panic!("Cannot convert {:?} to f32", value),
        }
    }
}

impl From<&NbtValue> for f64 {
    fn from(value: &NbtValue) -> Self {
        match value {
            NbtValue::Double(v) => *v,
            _ => panic!("Cannot convert {:?} to f64", value),
        }
    }
}

impl From<&NbtValue> for Vec<i8> {
    fn from(value: &NbtValue) -> Self {
        match value {
            NbtValue::ByteArray(v) => v.clone(),
            _ => panic!("Cannot convert {:?} to Vec<i8>", value),
        }
    }
}

impl From<&NbtValue> for String {
    fn from(value: &NbtValue) -> Self {
        match value {
            NbtValue::String(v) => v.clone(),
            _ => panic!("Cannot convert {:?} to String", value),
        }
    }
}

impl From<&NbtValue> for Vec<NbtValue> {
    fn from(value: &NbtValue) -> Self {
        match value {
            NbtValue::List(v) => v.clone(),
            _ => panic!("Cannot convert {:?} to Vec<NbtValue>", value),
        }
    }
}

impl From<&NbtValue> for HashMap<String, NbtValue> {
    fn from(value: &NbtValue) -> Self {
        match value {
            NbtValue::Compound(v) => v.clone(),
            _ => panic!("Cannot convert {:?} to HashMap<String, NbtValue>", value),
        }
    }
}

impl From<&NbtValue> for Vec<i32> {
    fn from(value: &NbtValue) -> Self {
        match value {
            NbtValue::IntArray(v) => v.clone(),
            _ => panic!("Cannot convert {:?} to Vec<i32>", value),
        }
    }
}

impl From<&NbtValue> for Vec<i64> {
    fn from(value: &NbtValue) -> Self {
        match value {
            NbtValue::LongArray(v) => v.clone(),
            _ => panic!("Cannot convert {:?} to Vec<i64>", value),
        }
    }
}
