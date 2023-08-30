use std::path::PathBuf;

use commandblock::nbt::{read_from_file, write_to_file, Compression, Endian};

#[test]
fn test_writing_new_uncompressed_data() {
    let (_, value) = read_from_file(
        PathBuf::from("tests/data/java_level.dat"),
        Compression::Gzip,
        Endian::Big,
        false,
    )
    .unwrap();

    let file_path = PathBuf::from("tests/data/test.dat");

    write_to_file(
        None,
        value.clone(),
        file_path.clone(),
        Compression::Gzip,
        Endian::Big,
        false,
    )
    .unwrap();

    let (_, read_value) =
        read_from_file(file_path.clone(), Compression::Gzip, Endian::Big, false).unwrap();

    assert_eq!(value, read_value);

    std::fs::remove_file(file_path).unwrap();
}
