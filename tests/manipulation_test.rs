use std::path::PathBuf;

use commandblock::nbt::{read_from_file, Compression, NbtValue};

#[test]
fn test_create_compound() {
    let mut compound = NbtValue::new();
    compound.insert("key1".to_string(), NbtValue::Int(1));
    compound.insert("key2".to_string(), NbtValue::String("value2".to_string()));

    if let NbtValue::Compound(map) = compound {
        assert_eq!(map.get("key1"), Some(&NbtValue::Int(1)));
        assert_eq!(
            map.get("key2"),
            Some(&NbtValue::String("value2".to_string()))
        );
    } else {
        panic!("Expected NbtValue::Compound");
    }
}

#[test]
fn test_edit_compound() {
    let mut compound = NbtValue::new();

    // Test insert
    compound.insert("key1".to_string(), NbtValue::Int(1));
    compound.insert("key2".to_string(), NbtValue::String("value2".to_string()));
    assert_eq!(compound.get("key1"), Some(&NbtValue::Int(1)));
    assert_eq!(
        compound.get("key2"),
        Some(&NbtValue::String("value2".to_string()))
    );

    // Test updating via insert
    compound.insert("key1".to_string(), NbtValue::Int(2));
    assert_eq!(compound.get("key1"), Some(&NbtValue::Int(2)));

    // Test remove
    compound.remove("key1");
    assert_eq!(compound.get("key1"), None);
}

#[test]
fn test_into_nbtvalue() {
    let mut compound = NbtValue::new();
    compound.insert("key1".to_string(), 1);
    compound.insert("key2".to_string(), "value2");

    if let NbtValue::Compound(map) = compound {
        assert_eq!(map.get("key1"), Some(&NbtValue::Int(1)));
        assert_eq!(
            map.get("key2"),
            Some(&NbtValue::String("value2".to_string()))
        );
    } else {
        panic!("Expected NbtValue::Compound");
    }
}

#[test]
fn test_reading_and_manipulating_nbtvalue() {
    let java_data_path = PathBuf::from("tests").join("data").join("java_level.dat");
    let bedrock_data_path = PathBuf::from("tests")
        .join("data")
        .join("bedrock_level.dat");

    match read_from_file(
        java_data_path,
        Compression::Gzip,
        commandblock::nbt::Endian::Big,
    ) {
        Ok((_, mut java_result)) => {
            java_result.insert("LevelName".to_string(), "Java Data Test (modified)");

            if let NbtValue::Compound(map) = java_result {
                assert_eq!(
                    map.get("LevelName"),
                    Some(&NbtValue::String("Java Data Test (modified)".to_string()))
                );
            } else {
                panic!("Expected NbtValue::Compound");
            }
        }
        Err(error) => {
            assert!(false, "Failed to read NBT data from file: {:?}", error);
        }
    }

    match read_from_file(
        bedrock_data_path,
        Compression::Uncompressed,
        commandblock::nbt::Endian::Little,
    ) {
        Ok((_, mut bedrock_result)) => {
            bedrock_result.insert("LevelName".to_string(), "Bedrock Data Test (modified)");

            if let NbtValue::Compound(map) = bedrock_result {
                assert_eq!(
                    map.get("LevelName"),
                    Some(&NbtValue::String(
                        "Bedrock Data Test (modified)".to_string()
                    ))
                );
            } else {
                panic!("Expected NbtValue::Compound");
            }
        }
        Err(error) => {
            assert!(false, "Failed to read NBT data from file: {:?}", error);
        }
    }
}
