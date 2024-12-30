use crate::{tag::Tag, ByteArrayTag, CompoundTag, IntArrayTag, ListTag, LongArrayTag, StringTag};
use std::io::{Cursor, Result, Write};

/// Writes an NBT file to a byte vector, starting with the root compound tag.
pub fn write(tag: &Tag, root_name: &str) -> Result<Vec<u8>> {
    let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    write_u8(&mut cursor, tag.id())?;
    write_u16(&mut cursor, root_name.len() as u16)?;
    cursor.write_all(root_name.as_bytes())?;
    write_tag(&mut cursor, tag)?;
    Ok(cursor.into_inner())
}

/// Writes a single NBT tag to the given writer.
fn write_tag<W: Write>(writer: &mut W, tag: &Tag) -> Result<()> {
    match tag {
        Tag::End => Ok(()), // End tag has no payload.
        Tag::Byte(value) => write_i8(writer, *value),
        Tag::Short(value) => write_i16(writer, *value),
        Tag::Int(value) => write_i32(writer, *value),
        Tag::Long(value) => write_i64(writer, *value),
        Tag::Float(value) => write_f32(writer, *value),
        Tag::Double(value) => write_f64(writer, *value),
        Tag::ByteArray(data) => write_byte_array(writer, data),
        Tag::String(value) => write_string(writer, value),
        Tag::List(list) => write_list(writer, list),
        Tag::Compound(compound) => write_compound(writer, compound),
        Tag::IntArray(data) => write_int_array(writer, data),
        Tag::LongArray(data) => write_long_array(writer, data),
    }
}

/// Helper functions to write various data types to a writer.
fn write_u8<W: Write>(writer: &mut W, value: u8) -> Result<()> {
    writer.write_all(&[value])
}

fn write_i8<W: Write>(writer: &mut W, value: i8) -> Result<()> {
    write_u8(writer, value as u8)
}

fn write_u16<W: Write>(writer: &mut W, value: u16) -> Result<()> {
    writer.write_all(&value.to_be_bytes())
}

fn write_i16<W: Write>(writer: &mut W, value: i16) -> Result<()> {
    write_u16(writer, value as u16)
}

fn write_i32<W: Write>(writer: &mut W, value: i32) -> Result<()> {
    writer.write_all(&value.to_be_bytes())
}

fn write_i64<W: Write>(writer: &mut W, value: i64) -> Result<()> {
    writer.write_all(&value.to_be_bytes())
}

fn write_f32<W: Write>(writer: &mut W, value: f32) -> Result<()> {
    writer.write_all(&value.to_be_bytes())
}

fn write_f64<W: Write>(writer: &mut W, value: f64) -> Result<()> {
    writer.write_all(&value.to_be_bytes())
}

fn write_byte_array<W: Write>(writer: &mut W, data: &ByteArrayTag) -> Result<()> {
    write_i32(writer, data.len() as i32)?;
    for value in data {
        write_i8(writer, *value)?;
    }
    Ok(())
}

fn write_string<W: Write>(writer: &mut W, value: &StringTag) -> Result<()> {
    write_u16(writer, value.len() as u16)?;
    writer.write_all(value.as_bytes())
}

fn write_list<W: Write>(writer: &mut W, list: &ListTag<Tag>) -> Result<()> {
    if let Some(first_item) = list.first() {
        write_u8(writer, first_item.id())?;
        write_i32(writer, list.len() as i32)?;
        for item in list {
            write_tag(writer, item)?;
        }
    } else {
        write_u8(writer, 0)?; // Empty list type.
        write_i32(writer, 0)?; // Empty list length.
    }
    Ok(())
}

fn write_compound<W: Write>(writer: &mut W, compound: &CompoundTag) -> Result<()> {
    for (key, value) in compound {
        write_u8(writer, value.id())?;
        write_u16(writer, key.len() as u16)?;
        writer.write_all(key.as_bytes())?;
        write_tag(writer, value)?;
    }
    write_u8(writer, 0) // End tag for compound.
}

fn write_int_array<W: Write>(writer: &mut W, data: &IntArrayTag) -> Result<()> {
    write_i32(writer, data.len() as i32)?;
    for value in data {
        write_i32(writer, *value)?;
    }
    Ok(())
}

fn write_long_array<W: Write>(writer: &mut W, data: &LongArrayTag) -> Result<()> {
    write_i32(writer, data.len() as i32)?;
    for value in data {
        write_i64(writer, *value)?;
    }
    Ok(())
}
