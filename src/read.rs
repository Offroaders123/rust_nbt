use crate::{
    BedrockHeader, ByteArrayTag, ByteTag, CompoundTag, DoubleTag, Endian, FloatTag, IntArrayTag,
    IntTag, ListTag, LongArrayTag, LongTag, ShortTag, StringTag, Tag, TagIDError, TagId,
};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use indexmap::IndexMap;
use std::io::{self, Cursor, Read};

pub enum ReadError {
    IoError(io::Error),
    TagIDError(TagIDError),
    VarIntRange,
    VarLongRange,
}

impl From<io::Error> for ReadError {
    fn from(value: io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<TagIDError> for ReadError {
    fn from(value: TagIDError) -> Self {
        Self::TagIDError(value)
    }
}

impl From<ReadError> for io::Error {
    fn from(value: ReadError) -> Self {
        match value {
            ReadError::IoError(e) => e,
            ReadError::TagIDError(e) => {
                io::Error::new(io::ErrorKind::InvalidData, format!("{:?}", e))
            }
            ReadError::VarIntRange => {
                io::Error::new(io::ErrorKind::InvalidData, "VarInt size is too large")
            }
            ReadError::VarLongRange => {
                io::Error::new(io::ErrorKind::InvalidData, "VarLong size is too large")
            }
        }
    }
}

/// Reads an NBT file from a byte vector and returns its root compound tag.
pub fn read_root(data: &[u8], endian: &Endian, header: &BedrockHeader) -> Result<Tag, ReadError> {
    let mut cursor: Cursor<&[u8]> = Cursor::new(&data);
    if matches!(header, BedrockHeader::With) {
        let (storage_version, payload_len): (u32, u32) = read_bedrock_header(&mut cursor)?;
        println!("{storage_version}, {payload_len}");
    }
    let root_tag_id: TagId = read_tag_id(&mut cursor)?;
    let root_name: String = read_string(&mut cursor, endian)?;
    println!("{:?}", root_name);
    read_tag(&mut cursor, endian, &root_tag_id)
}

pub fn read_bedrock_header(reader: &mut impl Read) -> Result<(u32, u32), ReadError> {
    let storage_version: u32 = reader.read_u32::<LittleEndian>()?;
    let payload_len: u32 = reader.read_u32::<LittleEndian>()?;
    Ok((storage_version, payload_len))
}

/// Reads a single NBT tag from the given reader.
fn read_tag(reader: &mut impl Read, endian: &Endian, tag_id: &TagId) -> Result<Tag, ReadError> {
    match tag_id {
        TagId::End => Err(ReadError::TagIDError(TagIDError::UnexpectedEnd)),
        TagId::Byte => Ok(Tag::Byte(read_byte(reader)?)),
        TagId::Short => Ok(Tag::Short(read_short(reader, endian)?)),
        TagId::Int => Ok(Tag::Int(read_int(reader, endian)?)),
        TagId::Long => Ok(Tag::Long(read_long(reader, endian)?)),
        TagId::Float => Ok(Tag::Float(read_float(reader, endian)?)),
        TagId::Double => Ok(Tag::Double(read_double(reader, endian)?)),
        TagId::ByteArray => Ok(Tag::ByteArray(read_byte_array(reader, endian)?)),
        TagId::String => Ok(Tag::String(read_string(reader, endian)?)),
        TagId::List => Ok(Tag::List(read_list(reader, endian)?)),
        TagId::Compound => Ok(Tag::Compound(read_compound(reader, endian)?)),
        TagId::IntArray => Ok(Tag::IntArray(read_int_array(reader, endian)?)),
        TagId::LongArray => Ok(Tag::LongArray(read_long_array(reader, endian)?)),
    }
}

fn read_tag_id(reader: &mut impl Read) -> Result<TagId, ReadError> {
    let value: u8 = read_unsigned_byte(reader)?;
    Ok(TagId::try_from(value)?)
}

/// Helper functions to read various data types from a reader.
fn read_unsigned_byte(reader: &mut impl Read) -> Result<u8, ReadError> {
    Ok(reader.read_u8()?)
}

fn read_byte(reader: &mut impl Read) -> Result<ByteTag, ReadError> {
    Ok(reader.read_i8()?)
}

fn read_unsigned_short(reader: &mut impl Read, endian: &Endian) -> Result<u16, ReadError> {
    match endian {
        Endian::Big => Ok(reader.read_u16::<BigEndian>()?),
        Endian::Little | Endian::LittleVarInt => Ok(reader.read_u16::<LittleEndian>()?),
    }
}

fn read_short(reader: &mut impl Read, endian: &Endian) -> Result<ShortTag, ReadError> {
    match endian {
        Endian::Big => Ok(reader.read_i16::<BigEndian>()?),
        Endian::Little | Endian::LittleVarInt => Ok(reader.read_i16::<LittleEndian>()?),
    }
}

fn read_int(reader: &mut impl Read, endian: &Endian) -> Result<IntTag, ReadError> {
    match endian {
        Endian::Big => Ok(reader.read_i32::<BigEndian>()?),
        Endian::Little => Ok(reader.read_i32::<LittleEndian>()?),
        Endian::LittleVarInt => read_var_int_zig_zag(reader),
    }
}

fn read_var_int(reader: &mut impl Read) -> Result<IntTag, ReadError> {
    let mut value: i32 = 0;
    let mut shift: i32 = 0;

    loop {
        let byte: u8 = reader.read_u8()?;
        value |= ((byte & 0x7F) as i32) << shift;

        if (byte & 0x80) == 0 {
            break;
        }

        shift += 7;
        if shift >= 32 {
            return Err(ReadError::VarIntRange);
        }
    }

    Ok(value)
}

fn read_var_int_zig_zag(reader: &mut impl Read) -> Result<IntTag, ReadError> {
    let mut result: i32 = 0;
    let mut shift: i32 = 0;

    loop {
        let byte: u8 = reader.read_u8()?;
        result |= ((byte & 0x7F) as i32) << shift;

        if (byte & 0x80) == 0 {
            break;
        }

        shift += 7;
        if shift > 63 {
            return Err(ReadError::VarIntRange);
        }
    }

    let zigzag: i32 = ((((result << 31) >> 31) ^ result) >> 1) ^ (result & (1 << 31));
    Ok(zigzag)
}

fn read_long(reader: &mut impl Read, endian: &Endian) -> Result<LongTag, ReadError> {
    match endian {
        Endian::Big => Ok(reader.read_i64::<BigEndian>()?),
        Endian::Little => Ok(reader.read_i64::<LittleEndian>()?),
        Endian::LittleVarInt => read_var_long_zig_zag(reader),
    }
}

fn read_var_long_zig_zag(reader: &mut impl Read) -> Result<LongTag, ReadError> {
    let mut result: u64 = 0;
    let mut shift: i32 = 0;

    loop {
        let byte: u8 = reader.read_u8()?;
        result |= ((byte & 0x7F) as u64) << shift;

        if (byte & 0x80) == 0 {
            break;
        }

        shift += 7;
        if shift > 63 {
            return Err(ReadError::VarLongRange);
        }
    }

    let zigzag: i64 = ((result >> 1) as i64) ^ (-((result & 1) as i64));
    Ok(zigzag)
}

fn read_float(reader: &mut impl Read, endian: &Endian) -> Result<FloatTag, ReadError> {
    match endian {
        Endian::Big => Ok(reader.read_f32::<BigEndian>()?),
        Endian::Little | Endian::LittleVarInt => Ok(reader.read_f32::<LittleEndian>()?),
    }
}

fn read_double(reader: &mut impl Read, endian: &Endian) -> Result<DoubleTag, ReadError> {
    match endian {
        Endian::Big => Ok(reader.read_f64::<BigEndian>()?),
        Endian::Little | Endian::LittleVarInt => Ok(reader.read_f64::<LittleEndian>()?),
    }
}

fn read_byte_array(reader: &mut impl Read, endian: &Endian) -> Result<ByteArrayTag, ReadError> {
    let length: usize = match endian {
        Endian::LittleVarInt => read_var_int_zig_zag(reader)? as usize,
        _ => read_int(reader, endian)? as usize,
    };
    let mut value: ByteArrayTag = ByteArrayTag(Vec::with_capacity(length));
    for _ in 0..length {
        value.0.push(read_byte(reader)?);
    }
    Ok(value)
}

fn read_string(reader: &mut impl Read, endian: &Endian) -> Result<StringTag, ReadError> {
    let length: usize = match endian {
        Endian::LittleVarInt => read_var_int(reader)? as usize,
        _ => read_unsigned_short(reader, endian)? as usize,
    };
    let mut buffer: Vec<u8> = vec![0; length];
    reader.read_exact(&mut buffer)?;
    Ok(String::from_utf8(buffer).unwrap())
}

fn read_list(reader: &mut impl Read, endian: &Endian) -> Result<ListTag<Tag>, ReadError> {
    let tag_id: TagId = read_tag_id(reader)?;
    let length: usize = match endian {
        Endian::LittleVarInt => read_var_int_zig_zag(reader)? as usize,
        _ => read_int(reader, endian)? as usize,
    };
    let mut value: ListTag<Tag> = Vec::with_capacity(length);
    for _ in 0..length {
        value.push(read_tag(reader, endian, &tag_id)?);
    }
    Ok(value)
}

fn read_compound(reader: &mut impl Read, endian: &Endian) -> Result<CompoundTag, ReadError> {
    let mut value: CompoundTag = IndexMap::new();
    loop {
        let tag_id: TagId = read_tag_id(reader)?;
        if matches!(tag_id, TagId::End) {
            break;
        }
        let name: String = read_string(reader, endian)?;
        let entry: Tag = read_tag(reader, endian, &tag_id)?;
        value.insert(name, entry);
    }
    Ok(value)
}

fn read_int_array(reader: &mut impl Read, endian: &Endian) -> Result<IntArrayTag, ReadError> {
    let length: usize = match endian {
        Endian::LittleVarInt => read_var_int_zig_zag(reader)? as usize,
        _ => read_int(reader, endian)? as usize,
    };
    let mut value: IntArrayTag = IntArrayTag(Vec::with_capacity(length));
    for _ in 0..length {
        value.0.push(read_int(reader, endian)?);
    }
    Ok(value)
}

fn read_long_array(reader: &mut impl Read, endian: &Endian) -> Result<LongArrayTag, ReadError> {
    let length: usize = match endian {
        Endian::LittleVarInt => read_var_int_zig_zag(reader)? as usize,
        _ => read_int(reader, endian)? as usize,
    };
    let mut value: LongArrayTag = LongArrayTag(Vec::with_capacity(length));
    for _ in 0..length {
        value.0.push(read_long(reader, endian)?);
    }
    Ok(value)
}
