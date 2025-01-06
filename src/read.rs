use crate::{
    ByteArrayTag, ByteTag, CompoundTag, DoubleTag, FloatTag, IntArrayTag, IntTag, ListTag,
    LongArrayTag, LongTag, ShortTag, StringTag, Tag, TagID,
};
use indexmap::IndexMap;
use std::io::{Cursor, Read, Result};

/// Reads an NBT file from a byte vector and returns its root compound tag.
pub fn read(data: &[u8]) -> Result<Tag> {
    let mut cursor: Cursor<&[u8]> = Cursor::new(&data);
    let root_tag_id: TagID = read_tag_id(&mut cursor)?;
    let name_length: usize = read_unsigned_short(&mut cursor)? as usize;
    let mut name_buffer: Vec<u8> = vec![0; name_length];
    cursor.read_exact(&mut name_buffer)?;
    let root_name: String = String::from_utf8(name_buffer).unwrap();
    println!("{:?}", root_name);
    read_tag(&mut cursor, &root_tag_id)
}

/// Reads a single NBT tag from the given reader.
fn read_tag<R: Read>(reader: &mut R, tag_id: &TagID) -> Result<Tag> {
    match tag_id {
        TagID::End => Ok(Tag::End),
        TagID::Byte => Ok(Tag::Byte(read_byte(reader)?)),
        TagID::Short => Ok(Tag::Short(read_short(reader)?)),
        TagID::Int => Ok(Tag::Int(read_int(reader)?)),
        TagID::Long => Ok(Tag::Long(read_long(reader)?)),
        TagID::Float => Ok(Tag::Float(read_float(reader)?)),
        TagID::Double => Ok(Tag::Double(read_double(reader)?)),
        TagID::ByteArray => Ok(Tag::ByteArray(read_byte_array(reader)?)),
        TagID::String => Ok(Tag::String(read_string(reader)?)),
        TagID::List => Ok(Tag::List(read_list(reader)?)),
        TagID::Compound => Ok(Tag::Compound(read_compound(reader)?)),
        TagID::IntArray => Ok(Tag::IntArray(read_int_array(reader)?)),
        TagID::LongArray => Ok(Tag::LongArray(read_long_array(reader)?)),
    }
}

fn read_tag_id<R: Read>(reader: &mut R) -> Result<TagID> {
    let value: u8 = read_unsigned_byte(reader)?;
    TagID::try_from(value)
}

/// Helper functions to read various data types from a reader.
fn read_unsigned_byte<R: Read>(reader: &mut R) -> Result<u8> {
    let mut buffer: [u8; 1] = [0; 1];
    reader.read_exact(&mut buffer)?;
    Ok(buffer[0])
}

fn read_byte<R: Read>(reader: &mut R) -> Result<ByteTag> {
    Ok(read_unsigned_byte(reader)? as i8)
}

fn read_unsigned_short<R: Read>(reader: &mut R) -> Result<u16> {
    let mut buffer: [u8; 2] = [0; 2];
    reader.read_exact(&mut buffer)?;
    Ok(u16::from_be_bytes(buffer))
}

fn read_short<R: Read>(reader: &mut R) -> Result<ShortTag> {
    Ok(read_unsigned_short(reader)? as i16)
}

fn read_int<R: Read>(reader: &mut R) -> Result<IntTag> {
    let mut buffer: [u8; 4] = [0; 4];
    reader.read_exact(&mut buffer)?;
    Ok(i32::from_be_bytes(buffer))
}

fn read_long<R: Read>(reader: &mut R) -> Result<LongTag> {
    let mut buffer: [u8; 8] = [0; 8];
    reader.read_exact(&mut buffer)?;
    Ok(i64::from_be_bytes(buffer))
}

fn read_float<R: Read>(reader: &mut R) -> Result<FloatTag> {
    let mut buffer: [u8; 4] = [0; 4];
    reader.read_exact(&mut buffer)?;
    Ok(f32::from_be_bytes(buffer))
}

fn read_double<R: Read>(reader: &mut R) -> Result<DoubleTag> {
    let mut buffer: [u8; 8] = [0; 8];
    reader.read_exact(&mut buffer)?;
    Ok(f64::from_be_bytes(buffer))
}

fn read_byte_array<R: Read>(reader: &mut R) -> Result<ByteArrayTag> {
    let length: usize = read_int(reader)? as usize;
    let mut value: ByteArrayTag = Vec::with_capacity(length);
    for _ in 0..length {
        value.push(read_byte(reader)?);
    }
    Ok(value)
}

fn read_string<R: Read>(reader: &mut R) -> Result<StringTag> {
    let length: usize = read_unsigned_short(reader)? as usize;
    let mut buffer: Vec<u8> = vec![0; length];
    reader.read_exact(&mut buffer)?;
    Ok(String::from_utf8(buffer).unwrap())
}

fn read_list<R: Read>(reader: &mut R) -> Result<ListTag<Tag>> {
    let tag_id: TagID = read_tag_id(reader)?;
    let length: usize = read_int(reader)? as usize;
    let mut value: ListTag<Tag> = Vec::with_capacity(length);
    for _ in 0..length {
        value.push(read_tag(reader, &tag_id)?);
    }
    Ok(value)
}

fn read_compound<R: Read>(reader: &mut R) -> Result<CompoundTag> {
    let mut value: CompoundTag = IndexMap::new();
    loop {
        let tag_id: TagID = read_tag_id(reader)?;
        match tag_id {
            TagID::End => break,
            _ => (),
        }
        let name: String = read_string(reader)?;
        let entry: Tag = read_tag(reader, &tag_id)?;
        value.insert(name, entry);
    }
    Ok(value)
}

fn read_int_array<R: Read>(reader: &mut R) -> Result<IntArrayTag> {
    let length: usize = read_int(reader)? as usize;
    let mut value: IntArrayTag = Vec::with_capacity(length);
    for _ in 0..length {
        value.push(read_int(reader)?);
    }
    Ok(value)
}

fn read_long_array<R: Read>(reader: &mut R) -> Result<LongArrayTag> {
    let length: usize = read_int(reader)? as usize;
    let mut value: LongArrayTag = Vec::with_capacity(length);
    for _ in 0..length {
        value.push(read_long(reader)?);
    }
    Ok(value)
}
