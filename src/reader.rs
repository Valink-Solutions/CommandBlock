use std::{collections::HashMap, fs::File, io::Read, path::PathBuf};

use byteorder::ReadBytesExt;

#[cfg(feature = "java_edition")]
use byteorder::BigEndian;

#[cfg(feature = "bedrock_edition")]
use byteorder::LittleEndian;

use crate::types::{Compression, NbtError, NbtValue};

use flate2::read::{GzDecoder, ZlibDecoder};

pub fn parse_nbt_value<R: Read>(reader: &mut R, tag_type: u8) -> Result<NbtValue, NbtError> {
    match tag_type {
        0x00 => Ok(NbtValue::End),
        0x01 => {
            let value = parse_byte(reader)?;
            Ok(NbtValue::Byte(value))
        }
        0x02 => {
            let value = parse_short(reader)?;
            Ok(NbtValue::Short(value))
        }
        0x03 => {
            let value = parse_int(reader)?;
            Ok(NbtValue::Int(value))
        }
        0x04 => {
            let value = parse_long(reader)?;
            Ok(NbtValue::Long(value))
        }
        0x05 => {
            let value = parse_float(reader)?;
            Ok(NbtValue::Float(value))
        }
        0x06 => {
            let value = parse_double(reader)?;
            Ok(NbtValue::Double(value))
        }
        0x07 => {
            let array_length = parse_int(reader)? as usize;
            let mut array = Vec::with_capacity(array_length);

            for _ in 0..array_length {
                let value = parse_byte(reader)?;
                array.push(value);
            }
            Ok(NbtValue::ByteArray(array))
        }
        0x08 => {
            let value = parse_string(reader)?;
            Ok(NbtValue::String(value))
        }
        0x09 => {
            let initial_byte = parse_byte(reader)? as u8;
            let list_tag_type = NbtValue::from_binary(initial_byte);
            let array_length = parse_int(reader)?;

            if list_tag_type.is_none() {
                return Err(NbtError::InvalidTagType(initial_byte));
            }

            let mut vec = Vec::with_capacity(array_length as usize);

            for _ in 0..array_length {
                if let Some(tag_type) = list_tag_type.as_ref() {
                    let value = parse_nbt_value(reader, tag_type.to_binary())?;
                    vec.push(value);
                } else {
                    return Err(NbtError::InvalidTagType(initial_byte));
                }
            }

            Ok(NbtValue::List(vec))
        }
        0x0A => {
            let mut map = HashMap::new();

            loop {
                match parse_tag(reader) {
                    Ok((key, value)) => {
                        if let NbtValue::End = value {
                            break;
                        }
                        map.insert(key, value);
                    }
                    Err(NbtError::InvalidCompoundType(0)) => break,
                    Err(e) => return Err(e),
                }
            }

            Ok(NbtValue::Compound(map))
        }
        0x0B => {
            let array_length = parse_int(reader)? as usize;
            let array_length = array_length as usize;
            let mut array = Vec::with_capacity(array_length);
            for _ in 0..array_length {
                let value = parse_int(reader)?;
                array.push(value);
            }
            Ok(NbtValue::IntArray(array))
        }
        0x0C => {
            let array_length = parse_int(reader)? as usize;
            let mut array = Vec::with_capacity(array_length);
            for _ in 0..array_length {
                let value = parse_long(reader)?;
                array.push(value);
            }
            Ok(NbtValue::LongArray(array))
        }
        _ => return Err(NbtError::InvalidTagType(tag_type)),
    }
}

fn parse_tag<R: Read>(reader: &mut R) -> Result<(String, NbtValue), NbtError> {
    let mut header = [0_u8; 1];
    reader.read_exact(&mut header)?;

    if let Some(tag) = NbtValue::from_binary(header[0]) {
        if let NbtValue::End = tag {
            return Ok((String::new(), NbtValue::End));
        }

        let name = parse_string(reader)?;
        let value = parse_nbt_value(reader, tag.to_binary())?;

        Ok((name, value))
    } else {
        Err(NbtError::InvalidTagType(header[0]))
    }
}

fn parse_byte<R: Read>(reader: &mut R) -> Result<i8, NbtError> {
    let mut data = [0u8; 1];
    reader.read_exact(&mut data)?;
    Ok(data[0] as i8)
}

fn parse_short<R: Read>(reader: &mut R) -> Result<i16, NbtError> {
    #[cfg(feature = "java_edition")]
    {
        let value = reader.read_i16::<BigEndian>()?;
        Ok(value)
    }

    #[cfg(feature = "bedrock_edition")]
    {
        let value = reader.read_i16::<LittleEndian>()?;
        Ok(value)
    }
}

fn parse_int<R: Read>(reader: &mut R) -> Result<i32, NbtError> {
    #[cfg(feature = "java_edition")]
    {
        let value = reader.read_i32::<BigEndian>()?;
        Ok(value)
    }

    #[cfg(feature = "bedrock_edition")]
    {
        let value = reader.read_i32::<LittleEndian>()?;
        Ok(value)
    }
}

fn parse_long<R: Read>(reader: &mut R) -> Result<i64, NbtError> {
    #[cfg(feature = "java_edition")]
    {
        let value = reader.read_i64::<BigEndian>()?;
        Ok(value)
    }

    #[cfg(feature = "bedrock_edition")]
    {
        let value = reader.read_i64::<LittleEndian>()?;
        Ok(value)
    }
}

fn parse_float<R: Read>(reader: &mut R) -> Result<f32, NbtError> {
    #[cfg(feature = "java_edition")]
    {
        let value = reader.read_f32::<BigEndian>()?;
        Ok(value)
    }

    #[cfg(feature = "bedrock_edition")]
    {
        let value = reader.read_f32::<LittleEndian>()?;
        Ok(value)
    }
}

fn parse_double<R: Read>(reader: &mut R) -> Result<f64, NbtError> {
    #[cfg(feature = "java_edition")]
    {
        let value = reader.read_f64::<BigEndian>()?;
        Ok(value)
    }

    #[cfg(feature = "bedrock_edition")]
    {
        let value = reader.read_f64::<LittleEndian>()?;
        Ok(value)
    }
}

fn parse_string<R: Read>(reader: &mut R) -> Result<String, NbtError> {
    let string_length = parse_short(reader)? as usize;
    let mut string = String::with_capacity(string_length);
    let mut buffer = vec![0u8; string_length];
    reader.read_exact(&mut buffer)?;
    string.push_str(&String::from_utf8_lossy(&buffer[..]));
    Ok(string)
}

pub fn read_from_file(path: PathBuf, compression: Compression) -> Result<NbtValue, NbtError> {
    let mut file = File::open(path)?;

    match compression {
        Compression::Uncompressed => {
            // handle parsing data and consumed bytes
            let (_, value) = parse_tag(&mut file)?;
            Ok(value)
        }
        Compression::Gzip => {
            let mut decoder = GzDecoder::new(&mut file);
            let (_, value) = parse_tag(&mut decoder)?;
            Ok(value)
        }
        Compression::Zlib => {
            let mut decoder = ZlibDecoder::new(&mut file);
            let (_, value) = parse_tag(&mut decoder)?;
            Ok(value)
        }
    }
}
