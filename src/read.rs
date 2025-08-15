use crate::{
    ByteArrayTag, ByteTag, CompoundTag, DoubleTag, FloatTag, IntArrayTag, IntTag, ListTag,
    LongArrayTag, LongTag, ShortTag, StringTag, Tag, TagIDError, TagId,
};
use byteorder::{BigEndian, ReadBytesExt};
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
pub fn read_root(data: &[u8]) -> Result<Tag, ReadError> {
    let mut cursor: Cursor<&[u8]> = Cursor::new(&data);
    let root_tag_id: TagId = read_tag_id(&mut cursor)?;
    let root_name: String = read_string(&mut cursor)?;
    println!("{:?}", root_name);
    read_tag(&mut cursor, &root_tag_id)
}

/// Reads a single NBT tag from the given reader.
fn read_tag<R: Read>(reader: &mut R, tag_id: &TagId) -> Result<Tag, ReadError> {
    match tag_id {
        TagId::End => Err(ReadError::TagIDError(TagIDError::UnexpectedEnd)),
        TagId::Byte => Ok(Tag::Byte(read_byte(reader)?)),
        TagId::Short => Ok(Tag::Short(read_short(reader)?)),
        TagId::Int => Ok(Tag::Int(read_int(reader)?)),
        TagId::Long => Ok(Tag::Long(read_long(reader)?)),
        TagId::Float => Ok(Tag::Float(read_float(reader)?)),
        TagId::Double => Ok(Tag::Double(read_double(reader)?)),
        TagId::ByteArray => Ok(Tag::ByteArray(read_byte_array(reader)?)),
        TagId::String => Ok(Tag::String(read_string(reader)?)),
        TagId::List => Ok(Tag::List(read_list(reader)?)),
        TagId::Compound => Ok(Tag::Compound(read_compound(reader)?)),
        TagId::IntArray => Ok(Tag::IntArray(read_int_array(reader)?)),
        TagId::LongArray => Ok(Tag::LongArray(read_long_array(reader)?)),
    }
}

fn read_tag_id<R: Read>(reader: &mut R) -> Result<TagId, ReadError> {
    let value: u8 = read_unsigned_byte(reader)?;
    Ok(TagId::try_from(value)?)
}

/// Helper functions to read various data types from a reader.
fn read_unsigned_byte<R: Read>(reader: &mut R) -> Result<u8, ReadError> {
    Ok(reader.read_u8()?)
}

fn read_byte<R: Read>(reader: &mut R) -> Result<ByteTag, ReadError> {
    Ok(reader.read_i8()?)
}

fn read_unsigned_short<R: Read>(reader: &mut R) -> Result<u16, ReadError> {
    Ok(reader.read_u16::<BigEndian>()?)
}

fn read_short<R: Read>(reader: &mut R) -> Result<ShortTag, ReadError> {
    Ok(reader.read_i16::<BigEndian>()?)
}

fn read_int<R: Read>(reader: &mut R) -> Result<IntTag, ReadError> {
    Ok(reader.read_i32::<BigEndian>()?)
}

fn read_long<R: Read>(reader: &mut R) -> Result<LongTag, ReadError> {
    Ok(reader.read_i64::<BigEndian>()?)
}

fn read_float<R: Read>(reader: &mut R) -> Result<FloatTag, ReadError> {
    Ok(reader.read_f32::<BigEndian>()?)
}

fn read_double<R: Read>(reader: &mut R) -> Result<DoubleTag, ReadError> {
    Ok(reader.read_f64::<BigEndian>()?)
}

fn read_byte_array<R: Read>(reader: &mut R) -> Result<ByteArrayTag, ReadError> {
    let length: usize = read_int(reader)? as usize;
    let mut value: ByteArrayTag = ByteArrayTag(Vec::with_capacity(length));
    for _ in 0..length {
        value.0.push(read_byte(reader)?);
    }
    Ok(value)
}

fn read_string<R: Read>(reader: &mut R) -> Result<StringTag, ReadError> {
    let length: usize = read_unsigned_short(reader)? as usize;
    let mut buffer: Vec<u8> = vec![0; length];
    reader.read_exact(&mut buffer)?;
    Ok(String::from_utf8(buffer).unwrap())
}

fn read_list<R: Read>(reader: &mut R) -> Result<ListTag<Tag>, ReadError> {
    let tag_id: TagId = read_tag_id(reader)?;
    let length: usize = read_int(reader)? as usize;
    let mut value: ListTag<Tag> = Vec::with_capacity(length);
    for _ in 0..length {
        value.push(read_tag(reader, &tag_id)?);
    }
    Ok(value)
}

fn read_compound<R: Read>(reader: &mut R) -> Result<CompoundTag, ReadError> {
    let mut value: CompoundTag = IndexMap::new();
    loop {
        let tag_id: TagId = read_tag_id(reader)?;
        match tag_id {
            TagId::End => break,
            _ => (),
        }
        let name: String = read_string(reader)?;
        let entry: Tag = read_tag(reader, &tag_id)?;
        value.insert(name, entry);
    }
    Ok(value)
}

fn read_int_array<R: Read>(reader: &mut R) -> Result<IntArrayTag, ReadError> {
    let length: usize = read_int(reader)? as usize;
    let mut value: IntArrayTag = IntArrayTag(Vec::with_capacity(length));
    for _ in 0..length {
        value.0.push(read_int(reader)?);
    }
    Ok(value)
}

fn read_long_array<R: Read>(reader: &mut R) -> Result<LongArrayTag, ReadError> {
    let length: usize = read_int(reader)? as usize;
    let mut value: LongArrayTag = LongArrayTag(Vec::with_capacity(length));
    for _ in 0..length {
        value.0.push(read_long(reader)?);
    }
    Ok(value)
}
