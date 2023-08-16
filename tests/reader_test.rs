use std::path::PathBuf;

use commandblock::{read_from_file, Compression, NbtReader, NbtValue};

#[test]
fn test_parse_nbt_value_end() {
    let data = [0x00];
    let result = NbtReader::new(&data[..], commandblock::Endian::Big)
        .parse_nbt_value(0x00)
        .unwrap();
    assert_eq!(result, NbtValue::End);
}

#[test]
fn test_parse_nbt_value_byte() {
    let data = [0x7F];
    let result = NbtReader::new(&data[..], commandblock::Endian::Big)
        .parse_nbt_value(0x01)
        .unwrap();
    assert_eq!(result, NbtValue::Byte(127));
}

#[test]
fn test_parse_nbt_value_short() {
    let java_data = [0x7F, 0xFF];
    let java_result = NbtReader::new(&java_data[..], commandblock::Endian::Big)
        .parse_nbt_value(0x02)
        .unwrap();

    let bedrock_data = [0xFF, 0x7F];
    let bedrock_result = NbtReader::new(&bedrock_data[..], commandblock::Endian::Little)
        .parse_nbt_value(0x02)
        .unwrap();

    assert_eq!(java_result, NbtValue::Short(32767));
    assert_eq!(bedrock_result, NbtValue::Short(32767));
}

#[test]
fn test_parse_nbt_value_int() {
    let java_data = [0x7F, 0xFF, 0xFF, 0xFF];
    let java_result = NbtReader::new(&java_data[..], commandblock::Endian::Big)
        .parse_nbt_value(0x03)
        .unwrap();

    let bedrock_data = [0xFF, 0xFF, 0xFF, 0x7F];
    let bedrock_result = NbtReader::new(&bedrock_data[..], commandblock::Endian::Little)
        .parse_nbt_value(0x03)
        .unwrap();

    assert_eq!(java_result, NbtValue::Int(2147483647));
    assert_eq!(bedrock_result, NbtValue::Int(2147483647));
}

#[test]
fn test_parse_nbt_value_long() {
    let java_data = [0x7F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    let java_result = NbtReader::new(&java_data[..], commandblock::Endian::Big)
        .parse_nbt_value(0x04)
        .unwrap();

    let bedrock_data = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x7F];
    let bedrock_result = NbtReader::new(&bedrock_data[..], commandblock::Endian::Little)
        .parse_nbt_value(0x04)
        .unwrap();

    assert_eq!(java_result, NbtValue::Long(9223372036854775807));
    assert_eq!(bedrock_result, NbtValue::Long(9223372036854775807));
}

#[test]
fn test_parse_nbt_value_float() {
    let java_data = [0x7F, 0x7F, 0xFF, 0xFF];
    let java_result = NbtReader::new(&java_data[..], commandblock::Endian::Big)
        .parse_nbt_value(0x05)
        .unwrap();

    let bedrock_data = [0xFF, 0xFF, 0x7F, 0x7F];
    let bedrock_result = NbtReader::new(&bedrock_data[..], commandblock::Endian::Little)
        .parse_nbt_value(0x05)
        .unwrap();

    assert_eq!(java_result, NbtValue::Float(3.4028235e38));
    assert_eq!(bedrock_result, NbtValue::Float(3.4028235e38));
}

#[test]
fn test_parse_nbt_value_double() {
    let java_data = [0x7F, 0xEF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    let java_result = NbtReader::new(&java_data[..], commandblock::Endian::Big)
        .parse_nbt_value(0x06)
        .unwrap();

    let bedrock_data = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xEF, 0x7F];
    let bedrock_result = NbtReader::new(&bedrock_data[..], commandblock::Endian::Little)
        .parse_nbt_value(0x06)
        .unwrap();

    assert_eq!(java_result, NbtValue::Double(1.7976931348623157e308));
    assert_eq!(bedrock_result, NbtValue::Double(1.7976931348623157e308));
}

#[test]
fn test_parse_nbt_value_byte_array() {
    let java_data = [
        0x00, 0x00, 0x00, 0x02, // array length
        0x7F, 0x7F, // array values
    ];
    let java_result = NbtReader::new(&java_data[..], commandblock::Endian::Big)
        .parse_nbt_value(0x07)
        .unwrap();

    let bedrock_data = [
        0x02, 0x00, 0x00, 0x00, // array length
        0x7F, 0x7F, // array values
    ];
    let bedrock_result = NbtReader::new(&bedrock_data[..], commandblock::Endian::Little)
        .parse_nbt_value(0x07)
        .unwrap();

    assert_eq!(java_result, NbtValue::ByteArray(vec![127, 127]));
    assert_eq!(bedrock_result, NbtValue::ByteArray(vec![127, 127]));
}

#[test]
fn test_parse_nbt_value_string() {
    let java_data = [
        0x00, 0x02, // string length
        0x41, 0x42, // string value
    ];
    let java_result = NbtReader::new(&java_data[..], commandblock::Endian::Big)
        .parse_nbt_value(0x08)
        .unwrap();

    let bedrock_data = [
        0x02, 0x00, // string length
        0x41, 0x42, // string value
    ];
    let bedrock_result = NbtReader::new(&bedrock_data[..], commandblock::Endian::Little)
        .parse_nbt_value(0x08)
        .unwrap();

    assert_eq!(java_result, NbtValue::String("AB".to_string()));
    assert_eq!(bedrock_result, NbtValue::String("AB".to_string()));
}

#[test]
fn test_parse_nbt_value_list() {
    let java_data = [
        0x01, // lists tag type
        0x00, 0x00, 0x00, 0x02, // array length
        0x7F, 0x7F, // array values
    ];
    let java_result = NbtReader::new(&java_data[..], commandblock::Endian::Big)
        .parse_nbt_value(0x09)
        .unwrap();

    let bedrock_data = [
        0x01, // lists tag type
        0x02, 0x00, 0x00, 0x00, // array length
        0x7F, 0x7F, // array values
    ];
    let bedrock_result = NbtReader::new(&bedrock_data[..], commandblock::Endian::Little)
        .parse_nbt_value(0x09)
        .unwrap();

    assert_eq!(
        java_result,
        NbtValue::List(vec![NbtValue::Byte(127), NbtValue::Byte(127)])
    );
    assert_eq!(
        bedrock_result,
        NbtValue::List(vec![NbtValue::Byte(127), NbtValue::Byte(127)])
    );
}

#[test]
fn test_parse_nbt_value_compound() {
    let java_data = [
        0x01, // tag type
        0x00, 0x02, // key length
        0x41, 0x42, // key value
        0x7F, // value
        0x00, // end tag
    ];
    let java_result = NbtReader::new(&java_data[..], commandblock::Endian::Big)
        .parse_nbt_value(0x0A)
        .unwrap();

    let bedrock_data = [
        0x01, // tag type
        0x02, 0x00, // key length
        0x41, 0x42, // key value
        0x7F, // value
        0x00, // end tag
    ];
    let bedrock_result = NbtReader::new(&bedrock_data[..], commandblock::Endian::Little)
        .parse_nbt_value(0x0A)
        .unwrap();

    let mut map = std::collections::HashMap::new();
    map.insert("AB".to_string(), NbtValue::Byte(127));

    assert_eq!(java_result, NbtValue::Compound(map));

    let mut map = std::collections::HashMap::new();
    map.insert("AB".to_string(), NbtValue::Byte(127));

    assert_eq!(bedrock_result, NbtValue::Compound(map));
}

#[test]
fn test_parse_nbt_value_int_array() {
    let java_data = [
        0x00, 0x00, 0x00, 0x02, // array length
        0x00, 0x00, 0x00, 0x01, // array values
        0x00, 0x00, 0x00, 0x02, // array values
    ];
    let java_result = NbtReader::new(&java_data[..], commandblock::Endian::Big)
        .parse_nbt_value(0x0B)
        .unwrap();

    let bedrock_data = [
        0x02, 0x00, 0x00, 0x00, // array length
        0x01, 0x00, 0x00, 0x00, // array values
        0x02, 0x00, 0x00, 0x00, // array values
    ];
    let bedrock_result = NbtReader::new(&bedrock_data[..], commandblock::Endian::Little)
        .parse_nbt_value(0x0B)
        .unwrap();

    assert_eq!(java_result, NbtValue::IntArray(vec![1, 2]));
    assert_eq!(bedrock_result, NbtValue::IntArray(vec![1, 2]));
}

#[test]
fn test_parse_nbt_value_long_array() {
    let java_data = [
        0x00, 0x00, 0x00, 0x02, // array length
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // array values
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
    ];
    let java_result = NbtReader::new(&java_data[..], commandblock::Endian::Big)
        .parse_nbt_value(0x0C)
        .unwrap();

    let bedrock_data = [
        0x02, 0x00, 0x00, 0x00, // array length
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // array values
        0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    let bedrock_result = NbtReader::new(&bedrock_data[..], commandblock::Endian::Little)
        .parse_nbt_value(0x0C)
        .unwrap();

    assert_eq!(java_result, NbtValue::LongArray(vec![0, 1]));
    assert_eq!(bedrock_result, NbtValue::LongArray(vec![0, 1]));
}

#[test]
fn test_read_from_dat_file() {
    let java_data_path = PathBuf::from("tests").join("data").join("java_level.dat");
    let bedrock_data_path = PathBuf::from("tests")
        .join("data")
        .join("bedrock_level.dat");

    let java_result = read_from_file(java_data_path, Compression::Gzip, commandblock::Endian::Big);

    let bedrock_result = read_from_file(
        bedrock_data_path,
        Compression::Uncompressed,
        commandblock::Endian::Little,
    );

    match java_result {
        Ok(NbtValue::Compound(value)) => {
            println!("Java Data: {:?} \n", value);
            assert!(true)
        }
        Ok(value) => {
            assert!(false, "Expected NbtValue::Compound, but got {:?}", value);
        }
        Err(error) => {
            assert!(false, "Failed to read NBT data from file: {:?}", error);
        }
    }

    match bedrock_result {
        Ok(NbtValue::Compound(value)) => {
            println!("Bedrock Data: {:?} \n", value);
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
