use std::collections::HashMap;
use std::io::{self, Cursor, Read};

/// Represents an NBT tag type.
#[repr(u8)]
#[derive(Debug)]
pub enum Tag {
    End,
    Byte(ByteTag) = 1,
    Short(ShortTag),
    Int(IntTag),
    Long(LongTag),
    Float(FloatTag),
    Double(DoubleTag),
    ByteArray(ByteArrayTag),
    String(StringTag),
    List(ListTag<Tag>),
    Compound(CompoundTag),
    IntArray(IntArrayTag),
    LongArray(LongArrayTag),
}

pub type ByteTag = i8;
pub type ShortTag = i16;
pub type IntTag = i32;
pub type LongTag = i64;
pub type FloatTag = f32;
pub type DoubleTag = f64;
pub type ByteArrayTag = Vec<u8>; // temp
pub type StringTag = String;
pub type ListTag<T> = Vec<T>;
pub type CompoundTag = HashMap<String, Tag>;
pub type IntArrayTag = Vec<i32>;
pub type LongArrayTag = Vec<i64>;

/// Reads a single NBT tag from the given reader.
fn read_tag<R: Read>(reader: &mut R, tag_id: u8) -> io::Result<Tag> {
    match tag_id {
        0 => Ok(Tag::End),
        1 => Ok(Tag::Byte(read_i8(reader)?)),
        2 => Ok(Tag::Short(read_i16(reader)?)),
        3 => Ok(Tag::Int(read_i32(reader)?)),
        4 => Ok(Tag::Long(read_i64(reader)?)),
        5 => Ok(Tag::Float(read_f32(reader)?)),
        6 => Ok(Tag::Double(read_f64(reader)?)),
        7 => {
            let length = read_i32(reader)? as usize;
            let mut data = vec![0; length];
            reader.read_exact(&mut data)?;
            Ok(Tag::ByteArray(data))
        }
        8 => {
            let length = read_u16(reader)? as usize;
            let mut buffer = vec![0; length];
            reader.read_exact(&mut buffer)?;
            Ok(Tag::String(String::from_utf8(buffer).unwrap()))
        }
        9 => {
            let item_type = read_u8(reader)?;
            let length = read_i32(reader)? as usize;
            let mut list = Vec::with_capacity(length);
            for _ in 0..length {
                list.push(read_tag(reader, item_type)?);
            }
            Ok(Tag::List(list))
        }
        10 => {
            let mut compound = HashMap::new();
            loop {
                let next_tag_id = read_u8(reader)?;
                if next_tag_id == 0 {
                    break;
                }
                let name_length = read_u16(reader)? as usize;
                let mut name_buffer = vec![0; name_length];
                reader.read_exact(&mut name_buffer)?;
                let name = String::from_utf8(name_buffer).unwrap();
                let tag = read_tag(reader, next_tag_id)?;
                compound.insert(name, tag);
            }
            Ok(Tag::Compound(compound))
        }
        11 => {
            let length = read_i32(reader)? as usize;
            let mut data = Vec::with_capacity(length);
            for _ in 0..length {
                data.push(read_i32(reader)?);
            }
            Ok(Tag::IntArray(data))
        }
        12 => {
            let length = read_i32(reader)? as usize;
            let mut data = Vec::with_capacity(length);
            for _ in 0..length {
                data.push(read_i64(reader)?);
            }
            Ok(Tag::LongArray(data))
        }
        _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Unknown tag ID")),
    }
}

/// Reads an NBT file from a byte vector and returns its root compound tag.
pub fn read_nbt_file(data: Vec<u8>) -> io::Result<Tag> {
    let mut cursor = Cursor::new(data);
    let root_tag_id = read_u8(&mut cursor)?;
    let name_length = read_u16(&mut cursor)? as usize;
    let mut name_buffer = vec![0; name_length];
    cursor.read_exact(&mut name_buffer)?;
    let _root_name = String::from_utf8(name_buffer).unwrap();
    read_tag(&mut cursor, root_tag_id)
}

/// Helper functions to read various data types from a reader.
fn read_u8<R: Read>(reader: &mut R) -> io::Result<u8> {
    let mut buffer = [0; 1];
    reader.read_exact(&mut buffer)?;
    Ok(buffer[0])
}

fn read_i8<R: Read>(reader: &mut R) -> io::Result<i8> {
    Ok(read_u8(reader)? as i8)
}

fn read_u16<R: Read>(reader: &mut R) -> io::Result<u16> {
    let mut buffer = [0; 2];
    reader.read_exact(&mut buffer)?;
    Ok(u16::from_be_bytes(buffer))
}

fn read_i16<R: Read>(reader: &mut R) -> io::Result<i16> {
    Ok(read_u16(reader)? as i16)
}

fn read_i32<R: Read>(reader: &mut R) -> io::Result<i32> {
    let mut buffer = [0; 4];
    reader.read_exact(&mut buffer)?;
    Ok(i32::from_be_bytes(buffer))
}

fn read_i64<R: Read>(reader: &mut R) -> io::Result<i64> {
    let mut buffer = [0; 8];
    reader.read_exact(&mut buffer)?;
    Ok(i64::from_be_bytes(buffer))
}

fn read_f32<R: Read>(reader: &mut R) -> io::Result<f32> {
    let mut buffer = [0; 4];
    reader.read_exact(&mut buffer)?;
    Ok(f32::from_be_bytes(buffer))
}

fn read_f64<R: Read>(reader: &mut R) -> io::Result<f64> {
    let mut buffer = [0; 8];
    reader.read_exact(&mut buffer)?;
    Ok(f64::from_be_bytes(buffer))
}
