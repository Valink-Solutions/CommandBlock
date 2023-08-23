/// This module provides functionality for reading, manipulating, and writing NBT data.
/// It supports both Java and Bedrock data formats.
///
/// # Example
///
/// ```
/// use commandblock::nbt::{read_from_file, write_to_file, NbtValue, Compression, Endian};
/// use std::collections::HashMap;
/// use std::path::PathBuf;
///
/// // Read NBT data from a file
/// let path = PathBuf::from("./tests/data/bedrock_level.dat");
/// let (name, mut value) = read_from_file(path, Compression::Uncompressed, Endian::Little).unwrap();
///
/// // Manipulate the NBT data which automatically converts to NbtValue's
/// value.insert("test".to_string(), "Hello, world!");
///
/// let mut inner_compound = HashMap::new();
/// inner_compound.insert("isTest".to_string(), NbtValue::Byte(1));
/// inner_compound.insert("numberTests".to_string(), NbtValue::Int(123));
/// value.insert("test2".to_string(), inner_compound);
///
/// // Write the manipulated NBT data to a new file
/// let path = PathBuf::from("./tests/data/test.dat");
/// write_to_file(Some(&name), value, path, Compression::Uncompressed, Endian::Little).unwrap();
/// ```
pub mod nbt;
