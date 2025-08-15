use crate::{
    BedrockHeader, ByteArrayTag, ByteTag, CompoundTag, DoubleTag, FloatTag, IntArrayTag, IntTag,
    ListTag, LongArrayTag, LongTag, ShortTag, StringTag, Tag, TagIDError, TagId,
};
use byteorder::{ByteOrder, ReadBytesExt};
use indexmap::IndexMap;
use std::io::{self, Cursor, Read};

pub enum ReadError {
    IoError(io::Error),
    TagIDError(TagIDError),
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
        }
    }
}

/// Reads an NBT file from a byte vector and returns its root compound tag.
pub fn read_root<E: ByteOrder>(data: &[u8], header: BedrockHeader) -> Result<Tag, ReadError> {
    let mut cursor: Cursor<&[u8]> = Cursor::new(&data);
    match header {
        BedrockHeader::With => {
            let (storage_version, payload_len): (u32, u32) = read_bedrock_header::<E>(&mut cursor)?;
            println!("{storage_version}, {payload_len}");
        }
        _ => (),
    }
    let root_tag_id: TagId = read_tag_id(&mut cursor)?;
    let root_name: String = read_string::<E>(&mut cursor)?;
    println!("{:?}", root_name);
    read_tag::<E>(&mut cursor, &root_tag_id)
}

pub fn read_bedrock_header<E: ByteOrder>(reader: &mut impl Read) -> Result<(u32, u32), ReadError> {
    let storage_version: u32 = reader.read_u32::<E>()?;
    let payload_len: u32 = reader.read_u32::<E>()?;
    Ok((storage_version, payload_len))
}

/// Reads a single NBT tag from the given reader.
fn read_tag<E: ByteOrder>(reader: &mut impl Read, tag_id: &TagId) -> Result<Tag, ReadError> {
    match tag_id {
        TagId::End => Err(ReadError::TagIDError(TagIDError::UnexpectedEnd)),
        TagId::Byte => Ok(Tag::Byte(read_byte(reader)?)),
        TagId::Short => Ok(Tag::Short(read_short::<E>(reader)?)),
        TagId::Int => Ok(Tag::Int(read_int::<E>(reader)?)),
        TagId::Long => Ok(Tag::Long(read_long::<E>(reader)?)),
        TagId::Float => Ok(Tag::Float(read_float::<E>(reader)?)),
        TagId::Double => Ok(Tag::Double(read_double::<E>(reader)?)),
        TagId::ByteArray => Ok(Tag::ByteArray(read_byte_array::<E>(reader)?)),
        TagId::String => Ok(Tag::String(read_string::<E>(reader)?)),
        TagId::List => Ok(Tag::List(read_list::<E>(reader)?)),
        TagId::Compound => Ok(Tag::Compound(read_compound::<E>(reader)?)),
        TagId::IntArray => Ok(Tag::IntArray(read_int_array::<E>(reader)?)),
        TagId::LongArray => Ok(Tag::LongArray(read_long_array::<E>(reader)?)),
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

fn read_unsigned_short<E: ByteOrder>(reader: &mut impl Read) -> Result<u16, ReadError> {
    Ok(reader.read_u16::<E>()?)
}

fn read_short<E: ByteOrder>(reader: &mut impl Read) -> Result<ShortTag, ReadError> {
    Ok(reader.read_i16::<E>()?)
}

fn read_int<E: ByteOrder>(reader: &mut impl Read) -> Result<IntTag, ReadError> {
    Ok(reader.read_i32::<E>()?)
}

fn read_long<E: ByteOrder>(reader: &mut impl Read) -> Result<LongTag, ReadError> {
    Ok(reader.read_i64::<E>()?)
}

fn read_float<E: ByteOrder>(reader: &mut impl Read) -> Result<FloatTag, ReadError> {
    Ok(reader.read_f32::<E>()?)
}

fn read_double<E: ByteOrder>(reader: &mut impl Read) -> Result<DoubleTag, ReadError> {
    Ok(reader.read_f64::<E>()?)
}

fn read_byte_array<E: ByteOrder>(reader: &mut impl Read) -> Result<ByteArrayTag, ReadError> {
    let length: usize = read_int::<E>(reader)? as usize;
    let mut value: ByteArrayTag = ByteArrayTag(Vec::with_capacity(length));
    for _ in 0..length {
        value.0.push(read_byte(reader)?);
    }
    Ok(value)
}

fn read_string<E: ByteOrder>(reader: &mut impl Read) -> Result<StringTag, ReadError> {
    let length: usize = read_unsigned_short::<E>(reader)? as usize;
    let mut buffer: Vec<u8> = vec![0; length];
    reader.read_exact(&mut buffer)?;
    Ok(String::from_utf8(buffer).unwrap())
}

fn read_list<E: ByteOrder>(reader: &mut impl Read) -> Result<ListTag<Tag>, ReadError> {
    let tag_id: TagId = read_tag_id(reader)?;
    let length: usize = read_int::<E>(reader)? as usize;
    let mut value: ListTag<Tag> = Vec::with_capacity(length);
    for _ in 0..length {
        value.push(read_tag::<E>(reader, &tag_id)?);
    }
    Ok(value)
}

fn read_compound<E: ByteOrder>(reader: &mut impl Read) -> Result<CompoundTag, ReadError> {
    let mut value: CompoundTag = IndexMap::new();
    loop {
        let tag_id: TagId = read_tag_id(reader)?;
        match tag_id {
            TagId::End => break,
            _ => (),
        }
        let name: String = read_string::<E>(reader)?;
        let entry: Tag = read_tag::<E>(reader, &tag_id)?;
        value.insert(name, entry);
    }
    Ok(value)
}

fn read_int_array<E: ByteOrder>(reader: &mut impl Read) -> Result<IntArrayTag, ReadError> {
    let length: usize = read_int::<E>(reader)? as usize;
    let mut value: IntArrayTag = IntArrayTag(Vec::with_capacity(length));
    for _ in 0..length {
        value.0.push(read_int::<E>(reader)?);
    }
    Ok(value)
}

fn read_long_array<E: ByteOrder>(reader: &mut impl Read) -> Result<LongArrayTag, ReadError> {
    let length: usize = read_int::<E>(reader)? as usize;
    let mut value: LongArrayTag = LongArrayTag(Vec::with_capacity(length));
    for _ in 0..length {
        value.0.push(read_long::<E>(reader)?);
    }
    Ok(value)
}
