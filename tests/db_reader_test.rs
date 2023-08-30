use commandblock::db::*;
//  use std::fs;
use std::path::Path;

#[test]
fn test_local_player_data() {
    let path = Path::new("tests/data/db");
    let mut db_reader = DbReader::new(path.to_str().unwrap(), 10);
    let key = "~local_player".as_bytes();
    let player_data = db_reader.get(key);
    assert!(player_data.is_some(), "Local player data should exist");
}
