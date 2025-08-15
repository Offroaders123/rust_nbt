use crate::{
    ByteArrayTag, ByteTag, CompoundTag, DoubleTag, FloatTag, IntArrayTag, IntTag, ListTag,
    LongArrayTag, LongTag, ShortTag, StringTag, Tag, TagId,
};
use byteorder::{ByteOrder, WriteBytesExt};
use std::io::{Cursor, Result, Write};

/// Writes an NBT file to a byte vector, starting with the root compound tag.
pub fn write_root<E: ByteOrder>(tag: &Tag, root_name: &str) -> Result<Vec<u8>> {
    let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    write_tag_id(&mut cursor, tag.id())?;
    write_unsigned_short::<E>(&mut cursor, root_name.len() as u16)?;
    cursor.write_all(root_name.as_bytes())?;
    write_tag::<E>(&mut cursor, tag)?;
    Ok(cursor.into_inner())
}

/// Writes a single NBT tag to the given writer.
fn write_tag<E: ByteOrder>(writer: &mut impl Write, tag: &Tag) -> Result<()> {
    match tag {
        Tag::Byte(value) => write_byte(writer, *value),
        Tag::Short(value) => write_short::<E>(writer, *value),
        Tag::Int(value) => write_int::<E>(writer, *value),
        Tag::Long(value) => write_long::<E>(writer, *value),
        Tag::Float(value) => write_float::<E>(writer, *value),
        Tag::Double(value) => write_double::<E>(writer, *value),
        Tag::ByteArray(data) => write_byte_array::<E>(writer, data),
        Tag::String(value) => write_string::<E>(writer, value),
        Tag::List(list) => write_list::<E>(writer, list),
        Tag::Compound(compound) => write_compound::<E>(writer, compound),
        Tag::IntArray(data) => write_int_array::<E>(writer, data),
        Tag::LongArray(data) => write_long_array::<E>(writer, data),
    }
}

fn write_tag_id(writer: &mut impl Write, tag_id: TagId) -> Result<()> {
    let value: u8 = tag_id as u8;
    write_unsigned_byte(writer, value)
}

/// Helper functions to write various data types to a writer.
fn write_unsigned_byte(writer: &mut impl Write, value: u8) -> Result<()> {
    writer.write_u8(value)
}

fn write_byte(writer: &mut impl Write, value: ByteTag) -> Result<()> {
    writer.write_i8(value)
}

fn write_unsigned_short<E: ByteOrder>(writer: &mut impl Write, value: u16) -> Result<()> {
    writer.write_u16::<E>(value)
}

fn write_short<E: ByteOrder>(writer: &mut impl Write, value: ShortTag) -> Result<()> {
    writer.write_i16::<E>(value)
}

fn write_int<E: ByteOrder>(writer: &mut impl Write, value: IntTag) -> Result<()> {
    writer.write_i32::<E>(value)
}

fn write_long<E: ByteOrder>(writer: &mut impl Write, value: LongTag) -> Result<()> {
    writer.write_i64::<E>(value)
}

fn write_float<E: ByteOrder>(writer: &mut impl Write, value: FloatTag) -> Result<()> {
    writer.write_f32::<E>(value)
}

fn write_double<E: ByteOrder>(writer: &mut impl Write, value: DoubleTag) -> Result<()> {
    writer.write_f64::<E>(value)
}

fn write_byte_array<E: ByteOrder>(writer: &mut impl Write, value: &ByteArrayTag) -> Result<()> {
    let length: IntTag = value.0.len() as i32;
    write_int::<E>(writer, length)?;
    for entry in &value.0 {
        write_byte(writer, *entry)?;
    }
    Ok(())
}

fn write_string<E: ByteOrder>(writer: &mut impl Write, value: &StringTag) -> Result<()> {
    let entry: &[u8] = value.as_bytes();
    let length: u16 = value.len() as u16;
    write_unsigned_short::<E>(writer, length)?;
    writer.write_all(entry)
}

fn write_list<E: ByteOrder>(writer: &mut impl Write, value: &ListTag<Tag>) -> Result<()> {
    if let Some(first_entry) = value.first() {
        let tag_id: TagId = first_entry.id();
        let length: IntTag = value.len() as i32;
        write_tag_id(writer, tag_id)?;
        write_int::<E>(writer, length)?;
        for entry in value {
            write_tag::<E>(writer, entry)?;
        }
    } else {
        write_tag_id(writer, TagId::End)?; // Empty list type.
        write_int::<E>(writer, 0)?; // Empty list length.
    }
    Ok(())
}

fn write_compound<E: ByteOrder>(writer: &mut impl Write, value: &CompoundTag) -> Result<()> {
    for (name, entry) in value {
        let tag_id: TagId = entry.id();
        write_tag_id(writer, tag_id)?;
        write_string::<E>(writer, name)?;
        write_tag::<E>(writer, entry)?;
    }
    write_tag_id(writer, TagId::End) // End tag for compound.
}

fn write_int_array<E: ByteOrder>(writer: &mut impl Write, value: &IntArrayTag) -> Result<()> {
    let length: IntTag = value.0.len() as i32;
    write_int::<E>(writer, length)?;
    for entry in &value.0 {
        write_int::<E>(writer, *entry)?;
    }
    Ok(())
}

fn write_long_array<E: ByteOrder>(writer: &mut impl Write, value: &LongArrayTag) -> Result<()> {
    let length: IntTag = value.0.len() as i32;
    write_int::<E>(writer, length)?;
    for entry in &value.0 {
        write_long::<E>(writer, *entry)?;
    }
    Ok(())
}
