use crate::tag::Tag;
use crate::{ByteArrayTag, CompoundTag, ListTag};
use indexmap::IndexMap;
use std::io::{Cursor, Error, ErrorKind, Read, Result};

/// Reads an NBT file from a byte vector and returns its root compound tag.
pub fn read(data: &[u8]) -> Result<Tag> {
    let mut cursor: Cursor<&[u8]> = Cursor::new(&data);
    let root_tag_id: u8 = read_u8(&mut cursor)?;
    let name_length: usize = read_u16(&mut cursor)? as usize;
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
        1 => Ok(Tag::Byte(read_i8(reader)?)),
        2 => Ok(Tag::Short(read_i16(reader)?)),
        3 => Ok(Tag::Int(read_i32(reader)?)),
        4 => Ok(Tag::Long(read_i64(reader)?)),
        5 => Ok(Tag::Float(read_f32(reader)?)),
        6 => Ok(Tag::Double(read_f64(reader)?)),
        7 => {
            let length: usize = read_i32(reader)? as usize;
            let mut data: ByteArrayTag = Vec::with_capacity(length);
            for _ in 0..length {
                data.push(read_i8(reader)?);
            }
            Ok(Tag::ByteArray(data))
        }
        8 => {
            let length: usize = read_u16(reader)? as usize;
            let mut buffer: Vec<u8> = vec![0; length];
            reader.read_exact(&mut buffer)?;
            Ok(Tag::String(String::from_utf8(buffer).unwrap()))
        }
        9 => {
            let item_type: u8 = read_u8(reader)?;
            let length: usize = read_i32(reader)? as usize;
            let mut list: ListTag<Tag> = Vec::with_capacity(length);
            for _ in 0..length {
                list.push(read_tag(reader, item_type)?);
            }
            Ok(Tag::List(list))
        }
        10 => {
            let mut compound: CompoundTag = IndexMap::new();
            loop {
                let next_tag_id: u8 = read_u8(reader)?;
                if next_tag_id == 0 {
                    break;
                }
                let name_length: usize = read_u16(reader)? as usize;
                let mut name_buffer: Vec<u8> = vec![0; name_length];
                reader.read_exact(&mut name_buffer)?;
                let name: String = String::from_utf8(name_buffer).unwrap();
                let tag: Tag = read_tag(reader, next_tag_id)?;
                compound.insert(name, tag);
            }
            Ok(Tag::Compound(compound))
        }
        11 => {
            let length: usize = read_i32(reader)? as usize;
            let mut data: Vec<i32> = Vec::with_capacity(length);
            for _ in 0..length {
                data.push(read_i32(reader)?);
            }
            Ok(Tag::IntArray(data))
        }
        12 => {
            let length: usize = read_i32(reader)? as usize;
            let mut data: Vec<i64> = Vec::with_capacity(length);
            for _ in 0..length {
                data.push(read_i64(reader)?);
            }
            Ok(Tag::LongArray(data))
        }
        _ => Err(Error::new(ErrorKind::InvalidData, "Unknown tag ID")),
    }
}

/// Helper functions to read various data types from a reader.
fn read_u8<R: Read>(reader: &mut R) -> Result<u8> {
    let mut buffer: [u8; 1] = [0; 1];
    reader.read_exact(&mut buffer)?;
    Ok(buffer[0])
}

fn read_i8<R: Read>(reader: &mut R) -> Result<i8> {
    Ok(read_u8(reader)? as i8)
}

fn read_u16<R: Read>(reader: &mut R) -> Result<u16> {
    let mut buffer: [u8; 2] = [0; 2];
    reader.read_exact(&mut buffer)?;
    Ok(u16::from_be_bytes(buffer))
}

fn read_i16<R: Read>(reader: &mut R) -> Result<i16> {
    Ok(read_u16(reader)? as i16)
}

fn read_i32<R: Read>(reader: &mut R) -> Result<i32> {
    let mut buffer: [u8; 4] = [0; 4];
    reader.read_exact(&mut buffer)?;
    Ok(i32::from_be_bytes(buffer))
}

fn read_i64<R: Read>(reader: &mut R) -> Result<i64> {
    let mut buffer: [u8; 8] = [0; 8];
    reader.read_exact(&mut buffer)?;
    Ok(i64::from_be_bytes(buffer))
}

fn read_f32<R: Read>(reader: &mut R) -> Result<f32> {
    let mut buffer: [u8; 4] = [0; 4];
    reader.read_exact(&mut buffer)?;
    Ok(f32::from_be_bytes(buffer))
}

fn read_f64<R: Read>(reader: &mut R) -> Result<f64> {
    let mut buffer: [u8; 8] = [0; 8];
    reader.read_exact(&mut buffer)?;
    Ok(f64::from_be_bytes(buffer))
}
