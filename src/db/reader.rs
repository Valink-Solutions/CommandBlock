use rusty_leveldb::{BloomPolicy, Compressor, CompressorList, LdbIterator, Options, DB};

use crate::nbt::{read_from_reader, Compression, Endian, NbtValue};

use miniz_oxide::deflate::{compress_to_vec, compress_to_vec_zlib};
use miniz_oxide::inflate::{decompress_to_vec, decompress_to_vec_zlib};
use std::rc::Rc;

struct ZlibCompressor(u8);

impl ZlibCompressor {
    /// level 0-10
    pub fn new(level: u8) -> Self {
        assert!(level <= 10);
        Self(level)
    }
}

impl Compressor for ZlibCompressor {
    fn encode(&self, block: Vec<u8>) -> rusty_leveldb::Result<Vec<u8>> {
        Ok(compress_to_vec_zlib(&block, self.0))
    }

    fn decode(&self, block: Vec<u8>) -> rusty_leveldb::Result<Vec<u8>> {
        decompress_to_vec_zlib(&block).map_err(|e| rusty_leveldb::Status {
            code: rusty_leveldb::StatusCode::CompressionError,
            err: e.to_string(),
        })
    }
}

struct RawZlibCompressor(u8);

impl RawZlibCompressor {
    /// level 0-10
    pub fn new(level: u8) -> Self {
        assert!(level <= 10);
        Self(level)
    }
}

impl Compressor for RawZlibCompressor {
    fn encode(&self, block: Vec<u8>) -> rusty_leveldb::Result<Vec<u8>> {
        Ok(compress_to_vec(&block, self.0))
    }

    fn decode(&self, block: Vec<u8>) -> rusty_leveldb::Result<Vec<u8>> {
        decompress_to_vec(&block).map_err(|e| rusty_leveldb::Status {
            code: rusty_leveldb::StatusCode::CompressionError,
            err: e.to_string(),
        })
    }
}

pub struct DbReader {
    db: DB,
}

impl DbReader {
    pub fn new(path: &str, compression_level: u8) -> DbReader {
        let mut options = Options::default();

        options.compressor = 0;
        options.create_if_missing = false;
        options.write_buffer_size = 4 * 1024 * 1024 as usize;

        let mut compressor_list = CompressorList::new();
        compressor_list.set_with_id(0, RawZlibCompressor::new(compression_level));
        compressor_list.set_with_id(1, ZlibCompressor::new(compression_level));
        options.compressor_list = Rc::new(compressor_list);

        options.filter_policy = Rc::new(Box::new(BloomPolicy::new(10)));

        let db = DB::open(path, options).unwrap();

        DbReader { db }
    }

    pub fn get(&mut self, key: &[u8]) -> Option<NbtValue> {
        if is_local_player_key(key) {
            self.parse_local_player(key)
        } else if is_player_key(key) {
            self.parse_local_player(key)
        } else {
            None
        }
    }

    pub fn put(&mut self, key: &[u8], value: &[u8]) {
        match self.db.put(key, value) {
            Ok(_) => (),
            Err(e) => println!("Error: {:?}", e),
        }
    }

    fn parse_local_player(&mut self, key: &[u8]) -> Option<NbtValue> {
        match self.db.get(key) {
            Some(data) => {
                let value = match read_from_reader(
                    data.as_slice(),
                    Compression::Uncompressed,
                    Endian::Little,
                    false,
                ) {
                    Ok((_, value)) => value,
                    Err(e) => {
                        println!("Error: {:?}", e);
                        return None;
                    }
                };

                Some(value)
            }
            None => None,
        }
    }

    pub fn parse_remote_players(&mut self) -> Option<NbtValue> {
        let mut parent = NbtValue::new();

        let mut iter = self.db.new_iter().unwrap();

        while let Some((key, value)) = iter.next() {
            if is_player_key(key.as_slice()) {
                let value = match read_from_reader(
                    value.as_slice(),
                    Compression::Uncompressed,
                    Endian::Little,
                    false,
                ) {
                    Ok((_, value)) => value,
                    Err(e) => {
                        println!("Error: {:?}", e);
                        return None;
                    }
                };

                parent.insert("".to_string(), value);
            }
        }

        Some(parent)
    }
}

fn is_local_player_key(key: &[u8]) -> bool {
    let key_string = String::from_utf8_lossy(key);

    key_string.starts_with("~local_player")
}

fn is_player_key(key: &[u8]) -> bool {
    let key_string = String::from_utf8_lossy(key);

    key_string.starts_with("player_")
}
