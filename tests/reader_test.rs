use std::io::Cursor;
use std::path::PathBuf;

use commandblock::{parse_nbt_value, read_from_file, Compression, NbtValue};

#[test]
fn test_parse_nbt_value_end() {
    let data = [0x00];
    let result = parse_nbt_value(&mut Cursor::new(&data[..]), 0x00).unwrap();
    assert_eq!(result, NbtValue::End);
}

#[test]
fn test_parse_nbt_value_byte() {
    let data = [0x7F];
    let result = parse_nbt_value(&mut Cursor::new(&data[..]), 0x01).unwrap();
    assert_eq!(result, NbtValue::Byte(127));
}

#[test]
fn test_parse_nbt_value_short() {
    #[cfg(feature = "bedrock_edition")]
    let data = [0xFF, 0x7F];

    #[cfg(feature = "java_edition")]
    let data = [0x7F, 0xFF];

    let result = parse_nbt_value(&mut Cursor::new(&data[..]), 0x02).unwrap();
    assert_eq!(result, NbtValue::Short(32767));
}

#[test]
fn test_read_from_dat_file() {
    #[cfg(feature = "bedrock_edition")]
    let path = PathBuf::from("tests").join("data").join("bedrock_level.dat");

    #[cfg(feature = "bedrock_edition")]
    let result = read_from_file(path, Compression::Uncompressed);

    #[cfg(feature = "java_edition")]
    let path = PathBuf::from("tests").join("data").join("java_level.dat");

    #[cfg(feature = "java_edition")]
    let result = read_from_file(path, Compression::Gzip);

    match result {
        Ok(NbtValue::Compound(value)) => {
            println!("{:?}", value);
            assert!(true)
        }
        Ok(value) => {
            assert!(false, "Expected NbtValue::Compound, but got {:?}", value);
        }
        Err(error) => {
            assert!(false, "Failed to read NBT data from file: {:?}", error);
        }
    }
}
