use std::{collections::HashMap, io::Write, path::PathBuf};

use byteorder::{BigEndian, LittleEndian, WriteBytesExt};

use crate::nbt::types::{Compression, Endian, NbtError, NbtValue};

/// `NbtWriter` is a struct that writes NBT data to a writer encoding it according to the specified endian style.
///
/// While this struct can be used directly, it is recommended to use the `write_to_file` or `write_to_writer` functions
/// for most cases as they will handle compression and error handling for you.
///
/// # Fields
///
/// * `writer: W` - The writer to which NBT data is written. This writer must implement the `Write` trait.
/// * `endian: Endian` - The endian style (Big or Little) used to write the NBT data.
///
/// # Examples
///
/// ```
/// use commandblock::nbt::{NbtValue, NbtWriter, Endian};
/// use std::io::Cursor;
///
/// let mut value = NbtValue::new();
/// value.insert("test".to_string(), "test string");
///
/// let mut writer = NbtWriter::new(Cursor::new(Vec::new()), Endian::Little);
/// writer.write_data(Some("root"), value).unwrap();
/// ```
pub struct NbtWriter<W: Write> {
    writer: W,
    endian: Endian,
}

impl<W: Write> NbtWriter<W> {
    /// Creates a new NbtWriter.
    ///
    /// # Arguments
    ///
    /// * `writer: W` - The writer to which NBT data is written. This writer must implement the `Write` trait.
    /// * `endian: Endian` - The endian style (Big or Little) used to write the NBT data.
    ///
    /// # Examples
    ///
    /// ```
    /// use commandblock::nbt::{NbtWriter, Endian};
    /// use std::io::Cursor;
    ///
    /// let writer = NbtWriter::new(Cursor::new(Vec::new()), Endian::Little);
    /// ```
    ///
    /// # Returns
    ///
    /// * `Self` - A new instance of NbtWriter.
    pub fn new(writer: W, endian: Endian) -> Self {
        NbtWriter { writer, endian }
    }

    /// Writes an NbtValue to the writer.
    ///
    /// # Arguments
    ///
    /// * `value: NbtValue` - The NBT value to be written.
    ///
    /// # Examples
    ///
    /// ```
    /// use commandblock::nbt::{NbtValue, NbtWriter, Endian};
    /// use std::io::Cursor;
    ///
    /// let mut value = NbtValue::new();
    /// value.insert("test".to_string(), "test string");
    ///
    /// let mut writer = NbtWriter::new(Cursor::new(Vec::new()), Endian::Little);
    /// writer.write_nbt_value(value).unwrap();
    /// ```
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the NBT value is successfully written.
    /// * `Err(NbtError)` - If there is an error while writing the NBT value.
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

    /// Writes an NbtValue with a name to the writer.
    ///
    /// # Arguments
    ///
    /// * `data_name: Option<&str>` - Optional name for the root tag of the NBT data.
    /// * `value: NbtValue` - The NBT value to be written.
    ///
    /// # Examples
    ///
    /// ```
    /// use commandblock::nbt::{NbtValue, NbtWriter, Endian};
    /// use std::io::Cursor;
    ///
    /// let mut value = NbtValue::new();
    /// value.insert("test".to_string(), "test string");
    ///
    /// let mut writer = NbtWriter::new(Cursor::new(Vec::new()), Endian::Little);
    /// writer.write_data(Some("root"), value).unwrap();
    /// ```
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the NBT value is successfully written.
    /// * `Err(NbtError)` - If there is an error while writing the NBT value.
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

/// Writes an NBT file to the requested path using the given compression and endian style.
/// If the file already exists, it will overwrite the existing data.
///
/// # Arguments
///
/// * `data_name: Option<&str>` - Optional name for the root tag of the NBT data.
/// * `value: NbtValue` - The NBT value to write.
/// * `path: PathBuf` - The path to the file to write to.
/// * `compression: Compression` - The compression method to use.
/// * `endian: Endian` - The byte order to use.
///
/// # Example
///
/// ```
/// use commandblock::nbt::{NbtValue, write_to_file, Compression, Endian};
/// use std::path::PathBuf;
///
/// let mut value = NbtValue::new();
/// value.insert("test".to_string(), "test string");
///
/// let path = PathBuf::from("./tests/data/test.dat");
///
/// write_to_file(None, value, path, Compression::Uncompressed, Endian::Little).unwrap();
/// ```
///
/// # Returns
///
/// * `Ok(())` - If the NBT value is successfully written to the file.
/// * `Err(NbtError)` - If there is an error while writing the NBT value to the file.
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

/// Writes an NBT data to the given writer using the given compression and endian style.
///
/// # Arguments
///
/// * `data_name: Option<&str>` - Optional name for the root tag of the NBT data.
/// * `value: NbtValue` - The NBT value to write.
/// * `writer: W` - The writer to write to.
/// * `compression: Compression` - The compression method to use.
/// * `endian: Endian` - The byte order to use.
///
/// # Example
///
/// ```
/// use commandblock::nbt::{NbtValue, write_to_writer, Compression, Endian};
/// use std::io::Cursor;
///
/// let mut value = NbtValue::new();
/// value.insert("test".to_string(), "test string");
///
/// let mut writer = Cursor::new(Vec::new());
///
/// write_to_writer(None, value, &mut writer, Compression::Uncompressed, Endian::Little).unwrap();
/// ```
///
/// # Returns
///
/// * `Ok(())` - If the NBT value is successfully written to the writer.
/// * `Err(NbtError)` - If there is an error while writing the NBT value to the writer.
pub fn write_to_writer<W: Write>(
    data_name: Option<&str>,
    value: NbtValue,
    writer: &mut W,
    compression: Compression,
    endian: Endian,
) -> Result<(), NbtError> {
    match compression {
        Compression::Uncompressed => {
            let mut nbt_writer = NbtWriter::new(writer, endian);
            nbt_writer.write_data(data_name, value)?;
        }
        Compression::Gzip => {
            let mut encoder = flate2::write::GzEncoder::new(writer, flate2::Compression::default());
            let mut nbt_writer = NbtWriter::new(&mut encoder, endian);
            nbt_writer.write_data(data_name, value)?;
        }
        Compression::Zlib => {
            let mut encoder =
                flate2::write::ZlibEncoder::new(writer, flate2::Compression::default());
            let mut nbt_writer = NbtWriter::new(&mut encoder, endian);
            nbt_writer.write_data(data_name, value)?;
        }
    }

    Ok(())
}
