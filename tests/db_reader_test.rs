use commandblock::db::*;
//  use std::fs;
use std::path::Path;

#[test]
fn test_player_data() {
    let path = Path::new("tests/data/db");
    let mut db_reader = DbReader::new(path.to_str().unwrap(), 10);
    let key = "~local_player".as_bytes();
    let local_player_data = db_reader.get(key);
    let remote_player_data = db_reader.parse_remote_players();

    assert!(remote_player_data.is_some(), "Remote player data should exist");
    assert!(local_player_data.is_some(), "Local player data should exist");
}
