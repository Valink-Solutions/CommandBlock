use std::{collections::HashMap, fs::File, io::Read, path::PathBuf};

use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

use crate::{
    types::{Compression, NbtError, NbtValue},
    Endian,
};

use flate2::read::{GzDecoder, ZlibDecoder};

pub struct NbtReader<R: Read> {
    reader: R,
    endian: Endian,
}

impl<R: Read> NbtReader<R> {
    pub fn new(reader: R, endian: Endian) -> Self {
        NbtReader { reader, endian }
    }

    pub fn parse_nbt_value(&mut self, tag_type: u8) -> Result<NbtValue, NbtError> {
        match tag_type {
            0x00 => Ok(NbtValue::End),
            0x01 => {
                let value = self.parse_byte()?;
                Ok(NbtValue::Byte(value))
            }
            0x02 => {
                let value = self.parse_short()?;
                Ok(NbtValue::Short(value))
            }
            0x03 => {
                let value = self.parse_int()?;
                Ok(NbtValue::Int(value))
            }
            0x04 => {
                let value = self.parse_long()?;
                Ok(NbtValue::Long(value))
            }
            0x05 => {
                let value = self.parse_float()?;
                Ok(NbtValue::Float(value))
            }
            0x06 => {
                let value = self.parse_double()?;
                Ok(NbtValue::Double(value))
            }
            0x07 => {
                let value = self.parse_byte_array()?;
                Ok(NbtValue::ByteArray(value))
            }
            0x08 => {
                let value = self.parse_string()?;
                Ok(NbtValue::String(value))
            }
            0x09 => {
                let initial_byte = self.parse_byte()? as u8;
                let list_tag_type = NbtValue::from_binary(initial_byte);
                let array_length = self.parse_int()?;

                if list_tag_type.is_none() {
                    return Err(NbtError::InvalidTagType(initial_byte));
                }

                let mut vec = Vec::with_capacity(array_length as usize);

                for _ in 0..array_length {
                    if let Some(tag_type) = list_tag_type.as_ref() {
                        let value = self.parse_nbt_value(tag_type.to_binary())?;
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
                    match self.parse_nbt_tag() {
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
                let array_length = self.parse_int()? as usize;
                let array_length = array_length as usize;
                let mut array = Vec::with_capacity(array_length);
                for _ in 0..array_length {
                    let value = self.parse_int()?;
                    array.push(value);
                }
                Ok(NbtValue::IntArray(array))
            }
            0x0C => {
                let array_length = self.parse_int()? as usize;
                let mut array = Vec::with_capacity(array_length);
                for _ in 0..array_length {
                    let value = self.parse_long()?;
                    array.push(value);
                }
                Ok(NbtValue::LongArray(array))
            }
            _ => return Err(NbtError::InvalidTagType(tag_type)),
        }
    }

    pub fn parse_data(&mut self) -> Result<(String, NbtValue), NbtError> {
        let mut header = [0_u8; 1];
        self.reader.read_exact(&mut header)?;

        if let Some(tag) = NbtValue::from_binary(header[0]) {
            if let NbtValue::End = tag {
                return Ok((String::new(), NbtValue::End));
            }

            let name = self.parse_string()?;
            let value = if self.endian == Endian::Little {
                let mut header = [0_u8; 8];
                self.reader.read_exact(&mut header)?;
                let _ = i32::from_le_bytes(header[..4].try_into().unwrap());
                let _ = i32::from_le_bytes(header[4..].try_into().unwrap());
                // Parse the file_type and file_length if needed
                self.parse_nbt_value(tag.to_binary())?
            } else {
                self.parse_nbt_value(tag.to_binary())?
            };

            Ok((name, value))
        } else {
            Err(NbtError::InvalidTagType(header[0]))
        }
    }

    fn parse_nbt_tag(&mut self) -> Result<(String, NbtValue), NbtError> {
        let mut header = [0_u8; 1];
        self.reader.read_exact(&mut header)?;

        if let Some(tag) = NbtValue::from_binary(header[0]) {
            if let NbtValue::End = tag {
                return Ok((String::new(), NbtValue::End));
            }

            let name = self.parse_string()?;
            let value = self.parse_nbt_value(tag.to_binary())?;

            Ok((name, value))
        } else {
            Err(NbtError::InvalidTagType(header[0]))
        }
    }

    fn parse_double(&mut self) -> Result<f64, NbtError> {
        let value = match self.endian {
            Endian::Big => self.reader.read_f64::<BigEndian>()?,
            Endian::Little => self.reader.read_f64::<LittleEndian>()?,
        };
        Ok(value)
    }

    fn parse_byte(&mut self) -> Result<i8, NbtError> {
        let mut data = [0u8; 1];
        self.reader.read_exact(&mut data)?;
        Ok(data[0] as i8)
    }

    fn parse_short(&mut self) -> Result<i16, NbtError> {
        let value = match self.endian {
            Endian::Big => self.reader.read_i16::<BigEndian>()?,
            Endian::Little => self.reader.read_i16::<LittleEndian>()?,
        };
        Ok(value)
    }

    fn parse_int(&mut self) -> Result<i32, NbtError> {
        let value = match self.endian {
            Endian::Big => self.reader.read_i32::<BigEndian>()?,
            Endian::Little => self.reader.read_i32::<LittleEndian>()?,
        };
        Ok(value)
    }

    fn parse_long(&mut self) -> Result<i64, NbtError> {
        let value = match self.endian {
            Endian::Big => self.reader.read_i64::<BigEndian>()?,
            Endian::Little => self.reader.read_i64::<LittleEndian>()?,
        };
        Ok(value)
    }

    fn parse_float(&mut self) -> Result<f32, NbtError> {
        let value = match self.endian {
            Endian::Big => self.reader.read_f32::<BigEndian>()?,
            Endian::Little => self.reader.read_f32::<LittleEndian>()?,
        };
        Ok(value)
    }

    fn parse_string(&mut self) -> Result<String, NbtError> {
        let string_length = self.parse_short()? as usize;
        let mut string = String::with_capacity(string_length);
        let mut buffer = vec![0u8; string_length];
        self.reader.read_exact(&mut buffer)?;
        string.push_str(&String::from_utf8_lossy(&buffer[..]));
        Ok(string)
    }

    fn parse_byte_array(&mut self) -> Result<Vec<i8>, NbtError> {
        let array_length = self.parse_int()? as usize;
        let mut array = Vec::with_capacity(array_length);
        for _ in 0..array_length {
            let value = self.parse_byte()?;
            array.push(value);
        }
        Ok(array)
    }
}

pub fn read_from_file(
    path: PathBuf,
    compression: Compression,
    endian_style: Endian,
) -> Result<NbtValue, NbtError> {
    let mut file = File::open(path)?;

    match compression {
        Compression::Uncompressed => {
            let mut parser = NbtReader::new(file, endian_style);
            let (_, value) = parser.parse_data()?;
            Ok(value)
        }
        Compression::Gzip => {
            let mut decoder = GzDecoder::new(&mut file);
            let mut parser = NbtReader::new(&mut decoder, endian_style);
            let (_, value) = parser.parse_data()?;
            Ok(value)
        }
        Compression::Zlib => {
            let mut decoder = ZlibDecoder::new(&mut file);
            let mut parser = NbtReader::new(&mut decoder, endian_style);
            let (_, value) = parser.parse_data()?;
            Ok(value)
        }
    }
}
