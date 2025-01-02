use crate::tag::Tag;
use crate::{ByteArrayTag, CompoundTag, IntArrayTag, ListTag, LongArrayTag, StringTag};
use indexmap::IndexMap;
use std::io::{Cursor, Error, ErrorKind, Read, Result};

/// Reads an NBT file from a byte vector and returns its root compound tag.
pub fn read(data: &[u8]) -> Result<Tag> {
    let mut cursor: Cursor<&[u8]> = Cursor::new(&data);
    let root_tag_id: u8 = read_unsigned_byte(&mut cursor)?;
    let name_length: usize = read_unsigned_short(&mut cursor)? as usize;
    let mut name_buffer: Vec<u8> = vec![0; name_length];
    cursor.read_exact(&mut name_buffer)?;
    let root_name: String = String::from_utf8(name_buffer).unwrap();
    println!("{:?}", root_name);
    read_tag(&mut cursor, root_tag_id)
}

/// Reads a single NBT tag from the given reader.
fn read_tag<R: Read>(reader: &mut R, tag_id: u8) -> Result<Tag> {
    match tag_id {
        0 => Ok(Tag::End),
        1 => Ok(Tag::Byte(read_byte(reader)?)),
        2 => Ok(Tag::Short(read_short(reader)?)),
        3 => Ok(Tag::Int(read_int(reader)?)),
        4 => Ok(Tag::Long(read_long(reader)?)),
        5 => Ok(Tag::Float(read_float(reader)?)),
        6 => Ok(Tag::Double(read_double(reader)?)),
        7 => Ok(Tag::ByteArray(read_byte_array(reader)?)),
        8 => Ok(Tag::String(read_string(reader)?)),
        9 => Ok(Tag::List(read_list(reader)?)),
        10 => Ok(Tag::Compound(read_compound(reader)?)),
        11 => Ok(Tag::IntArray(read_int_array(reader)?)),
        12 => Ok(Tag::LongArray(read_long_array(reader)?)),
        _ => Err(Error::new(ErrorKind::InvalidData, "Unknown tag ID")),
    }
}

/// Helper functions to read various data types from a reader.
fn read_unsigned_byte<R: Read>(reader: &mut R) -> Result<u8> {
    let mut buffer: [u8; 1] = [0; 1];
    reader.read_exact(&mut buffer)?;
    Ok(buffer[0])
}

fn read_byte<R: Read>(reader: &mut R) -> Result<i8> {
    Ok(read_unsigned_byte(reader)? as i8)
}

fn read_unsigned_short<R: Read>(reader: &mut R) -> Result<u16> {
    let mut buffer: [u8; 2] = [0; 2];
    reader.read_exact(&mut buffer)?;
    Ok(u16::from_be_bytes(buffer))
}

fn read_short<R: Read>(reader: &mut R) -> Result<i16> {
    Ok(read_unsigned_short(reader)? as i16)
}

fn read_int<R: Read>(reader: &mut R) -> Result<i32> {
    let mut buffer: [u8; 4] = [0; 4];
    reader.read_exact(&mut buffer)?;
    Ok(i32::from_be_bytes(buffer))
}

fn read_long<R: Read>(reader: &mut R) -> Result<i64> {
    let mut buffer: [u8; 8] = [0; 8];
    reader.read_exact(&mut buffer)?;
    Ok(i64::from_be_bytes(buffer))
}

fn read_float<R: Read>(reader: &mut R) -> Result<f32> {
    let mut buffer: [u8; 4] = [0; 4];
    reader.read_exact(&mut buffer)?;
    Ok(f32::from_be_bytes(buffer))
}

fn read_double<R: Read>(reader: &mut R) -> Result<f64> {
    let mut buffer: [u8; 8] = [0; 8];
    reader.read_exact(&mut buffer)?;
    Ok(f64::from_be_bytes(buffer))
}

fn read_byte_array<R: Read>(reader: &mut R) -> Result<ByteArrayTag> {
    let length: usize = read_int(reader)? as usize;
    let mut data: ByteArrayTag = Vec::with_capacity(length);
    for _ in 0..length {
        data.push(read_byte(reader)?);
    }
    Ok(data)
}

fn read_string<R: Read>(reader: &mut R) -> Result<StringTag> {
    let length: usize = read_unsigned_short(reader)? as usize;
    let mut buffer: Vec<u8> = vec![0; length];
    reader.read_exact(&mut buffer)?;
    Ok(String::from_utf8(buffer).unwrap())
}

fn read_list<R: Read>(reader: &mut R) -> Result<ListTag<Tag>> {
    let item_type: u8 = read_unsigned_byte(reader)?;
    let length: usize = read_int(reader)? as usize;
    let mut list: ListTag<Tag> = Vec::with_capacity(length);
    for _ in 0..length {
        list.push(read_tag(reader, item_type)?);
    }
    Ok(list)
}

fn read_compound<R: Read>(reader: &mut R) -> Result<CompoundTag> {
    let mut compound: CompoundTag = IndexMap::new();
    loop {
        let next_tag_id: u8 = read_unsigned_byte(reader)?;
        if next_tag_id == 0 {
            break;
        }
        let name_length: usize = read_unsigned_short(reader)? as usize;
        let mut name_buffer: Vec<u8> = vec![0; name_length];
        reader.read_exact(&mut name_buffer)?;
        let name: String = String::from_utf8(name_buffer).unwrap();
        let tag: Tag = read_tag(reader, next_tag_id)?;
        compound.insert(name, tag);
    }
    Ok(compound)
}

fn read_int_array<R: Read>(reader: &mut R) -> Result<IntArrayTag> {
    let length: usize = read_int(reader)? as usize;
    let mut data: Vec<i32> = Vec::with_capacity(length);
    for _ in 0..length {
        data.push(read_int(reader)?);
    }
    Ok(data)
}

fn read_long_array<R: Read>(reader: &mut R) -> Result<LongArrayTag> {
    let length: usize = read_int(reader)? as usize;
    let mut data: Vec<i64> = Vec::with_capacity(length);
    for _ in 0..length {
        data.push(read_long(reader)?);
    }
    Ok(data)
}
