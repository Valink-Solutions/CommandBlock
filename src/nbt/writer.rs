use std::{collections::HashMap, io::Write, path::PathBuf};

use byteorder::{BigEndian, LittleEndian, WriteBytesExt};

use crate::nbt::types::{Compression, Endian, NbtError, NbtValue};

pub struct NbtWriter<W: Write> {
    writer: W,
    endian: Endian,
}

impl<W: Write> NbtWriter<W> {
    fn new(writer: W, endian: Endian) -> Self {
        NbtWriter { writer, endian }
    }

    pub fn write_nbt_value(&mut self, value: NbtValue) -> Result<(), NbtError> {
        match value {
            NbtValue::End => {
                self.write_byte(0x00)?;
            }
            NbtValue::Byte(value) => {
                self.write_byte(value)?;
            }
            NbtValue::Short(value) => {
                self.write_short(value)?;
            }
            NbtValue::Int(value) => {
                self.write_int(value)?;
            }
            NbtValue::Long(value) => {
                self.write_long(value)?;
            }
            NbtValue::Float(value) => {
                self.write_float(value)?;
            }
            NbtValue::Double(value) => {
                self.write_double(value)?;
            }
            NbtValue::ByteArray(value) => {
                self.write_byte_array(value)?;
            }
            NbtValue::String(value) => {
                self.write_string(value)?;
            }
            NbtValue::List(value) => {
                self.write_list(value)?;
            }
            NbtValue::Compound(value) => {
                self.write_compound(value)?;
            }
            NbtValue::IntArray(value) => {
                self.write_int_array(value)?;
            }
            NbtValue::LongArray(value) => {
                self.write_long_array(value)?;
            }
        }

        Ok(())
    }

    pub fn write_data(&mut self, data_name: Option<&str>, value: NbtValue) -> Result<(), NbtError> {
        match self.endian {
            Endian::Big => {
                self.write_byte(0x0A)?;
                self.write_string(data_name.unwrap_or("Data").to_string())?;
                self.write_nbt_value(value)?;
            }
            Endian::Little => {
                self.write_int(3)?;
                self.write_int(value.len() as i32)?;
                self.write_byte(0x0A)?;
                self.write_string(data_name.unwrap_or("").to_string())?;
                self.write_nbt_value(value)?;
            }
        }

        Ok(())
    }

    fn write_byte(&mut self, value: i8) -> Result<(), NbtError> {
        self.writer.write_i8(value)?;
        Ok(())
    }

    fn write_short(&mut self, value: i16) -> Result<(), NbtError> {
        match self.endian {
            Endian::Big => {
                self.writer.write_i16::<BigEndian>(value)?;
            }
            Endian::Little => {
                self.writer.write_i16::<LittleEndian>(value)?;
            }
        }
        Ok(())
    }

    fn write_int(&mut self, value: i32) -> Result<(), NbtError> {
        match self.endian {
            Endian::Big => {
                self.writer.write_i32::<BigEndian>(value)?;
            }
            Endian::Little => {
                self.writer.write_i32::<LittleEndian>(value)?;
            }
        }
        Ok(())
    }

    fn write_long(&mut self, value: i64) -> Result<(), NbtError> {
        match self.endian {
            Endian::Big => {
                self.writer.write_i64::<BigEndian>(value)?;
            }
            Endian::Little => {
                self.writer.write_i64::<LittleEndian>(value)?;
            }
        }
        Ok(())
    }

    fn write_float(&mut self, value: f32) -> Result<(), NbtError> {
        match self.endian {
            Endian::Big => {
                self.writer.write_f32::<BigEndian>(value)?;
            }
            Endian::Little => {
                self.writer.write_f32::<LittleEndian>(value)?;
            }
        }
        Ok(())
    }

    fn write_double(&mut self, value: f64) -> Result<(), NbtError> {
        match self.endian {
            Endian::Big => {
                self.writer.write_f64::<BigEndian>(value)?;
            }
            Endian::Little => {
                self.writer.write_f64::<LittleEndian>(value)?;
            }
        }
        Ok(())
    }

    fn write_byte_array(&mut self, value: Vec<i8>) -> Result<(), NbtError> {
        self.write_int(value.len() as i32)?;
        for byte in value {
            self.write_byte(byte)?;
        }
        Ok(())
    }

    fn write_string(&mut self, value: String) -> Result<(), NbtError> {
        self.write_short(value.len() as i16)?;
        self.writer.write_all(value.as_bytes())?;
        Ok(())
    }

    fn write_list(&mut self, value: Vec<NbtValue>) -> Result<(), NbtError> {
        if let Some(first_value) = value.first() {
            let tag = first_value.to_binary();
            self.write_byte(tag as i8)?;
            self.write_int(value.len() as i32)?;
            for item in &value {
                if item.to_binary() != tag {
                    return Err(NbtError::InvalidListType(item.to_binary()));
                }
                self.write_nbt_value(item.clone())?;
            }
        } else {
            self.write_byte(0)?;
            self.write_int(0)?;
        }
        Ok(())
    }

    fn write_compound(&mut self, value: HashMap<String, NbtValue>) -> Result<(), NbtError> {
        for (name, value) in value {
            self.write_byte(value.to_binary() as i8)?;
            self.write_string(name)?;
            self.write_nbt_value(value)?;
        }
        self.write_nbt_value(NbtValue::End)?;
        Ok(())
    }

    fn write_int_array(&mut self, value: Vec<i32>) -> Result<(), NbtError> {
        self.write_int(value.len() as i32)?;
        for item in value {
            self.write_int(item)?;
        }
        Ok(())
    }

    fn write_long_array(&mut self, value: Vec<i64>) -> Result<(), NbtError> {
        self.write_int(value.len() as i32)?;
        for item in value {
            self.write_long(item)?;
        }
        Ok(())
    }
}

pub fn write_to_file(
    data_name: Option<&str>,
    value: NbtValue,
    path: PathBuf,
    compression: Compression,
    endian: Endian,
) -> Result<(), NbtError> {
    let file = std::fs::File::create(path)?;

    match compression {
        Compression::Uncompressed => {
            let mut writer = NbtWriter::new(file, endian);
            writer.write_data(data_name, value)?;
        }
        Compression::Gzip => {
            let mut encoder = flate2::write::GzEncoder::new(file, flate2::Compression::default());
            let mut writer = NbtWriter::new(&mut encoder, endian);
            writer.write_data(data_name, value)?;
        }
        Compression::Zlib => {
            let mut encoder = flate2::write::ZlibEncoder::new(file, flate2::Compression::default());
            let mut writer = NbtWriter::new(&mut encoder, endian);
            writer.write_data(data_name, value)?;
        }
    }

    Ok(())
}
