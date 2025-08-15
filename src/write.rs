use crate::{
    ByteArrayTag, ByteTag, CompoundTag, DoubleTag, FloatTag, IntArrayTag, IntTag, ListTag,
    LongArrayTag, LongTag, ShortTag, StringTag, Tag, TagId,
};
use byteorder::{BigEndian, WriteBytesExt};
use std::io::{Cursor, Result, Write};

/// Writes an NBT file to a byte vector, starting with the root compound tag.
pub fn write(tag: &Tag, root_name: &str) -> Result<Vec<u8>> {
    let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    write_tag_id(&mut cursor, tag.id())?;
    write_unsigned_short(&mut cursor, root_name.len() as u16)?;
    cursor.write_all(root_name.as_bytes())?;
    write_tag(&mut cursor, tag)?;
    Ok(cursor.into_inner())
}

/// Writes a single NBT tag to the given writer.
fn write_tag<W: Write>(writer: &mut W, tag: &Tag) -> Result<()> {
    match tag {
        Tag::Byte(value) => write_byte(writer, *value),
        Tag::Short(value) => write_short(writer, *value),
        Tag::Int(value) => write_int(writer, *value),
        Tag::Long(value) => write_long(writer, *value),
        Tag::Float(value) => write_float(writer, *value),
        Tag::Double(value) => write_double(writer, *value),
        Tag::ByteArray(data) => write_byte_array(writer, data),
        Tag::String(value) => write_string(writer, value),
        Tag::List(list) => write_list(writer, list),
        Tag::Compound(compound) => write_compound(writer, compound),
        Tag::IntArray(data) => write_int_array(writer, data),
        Tag::LongArray(data) => write_long_array(writer, data),
    }
}

fn write_tag_id<W: Write>(writer: &mut W, tag_id: TagId) -> Result<()> {
    let value: u8 = tag_id as u8;
    write_unsigned_byte(writer, value)
}

/// Helper functions to write various data types to a writer.
fn write_unsigned_byte<W: Write>(writer: &mut W, value: u8) -> Result<()> {
    writer.write_u8(value)
}

fn write_byte<W: Write>(writer: &mut W, value: ByteTag) -> Result<()> {
    writer.write_i8(value)
}

fn write_unsigned_short<W: Write>(writer: &mut W, value: u16) -> Result<()> {
    writer.write_u16::<BigEndian>(value)
}

fn write_short<W: Write>(writer: &mut W, value: ShortTag) -> Result<()> {
    writer.write_i16::<BigEndian>(value)
}

fn write_int<W: Write>(writer: &mut W, value: IntTag) -> Result<()> {
    writer.write_i32::<BigEndian>(value)
}

fn write_long<W: Write>(writer: &mut W, value: LongTag) -> Result<()> {
    writer.write_i64::<BigEndian>(value)
}

fn write_float<W: Write>(writer: &mut W, value: FloatTag) -> Result<()> {
    writer.write_f32::<BigEndian>(value)
}

fn write_double<W: Write>(writer: &mut W, value: DoubleTag) -> Result<()> {
    writer.write_f64::<BigEndian>(value)
}

fn write_byte_array<W: Write>(writer: &mut W, value: &ByteArrayTag) -> Result<()> {
    let length: IntTag = value.0.len() as i32;
    write_int(writer, length)?;
    for entry in &value.0 {
        write_byte(writer, *entry)?;
    }
    Ok(())
}

fn write_string<W: Write>(writer: &mut W, value: &StringTag) -> Result<()> {
    let entry: &[u8] = value.as_bytes();
    let length: u16 = value.len() as u16;
    write_unsigned_short(writer, length)?;
    writer.write_all(entry)
}

fn write_list<W: Write>(writer: &mut W, value: &ListTag<Tag>) -> Result<()> {
    if let Some(first_entry) = value.first() {
        let tag_id: TagId = first_entry.id();
        let length: IntTag = value.len() as i32;
        write_tag_id(writer, tag_id)?;
        write_int(writer, length)?;
        for entry in value {
            write_tag(writer, entry)?;
        }
    } else {
        write_tag_id(writer, TagId::End)?; // Empty list type.
        write_int(writer, 0)?; // Empty list length.
    }
    Ok(())
}

fn write_compound<W: Write>(writer: &mut W, value: &CompoundTag) -> Result<()> {
    for (name, entry) in value {
        let tag_id: TagId = entry.id();
        write_tag_id(writer, tag_id)?;
        write_string(writer, name)?;
        write_tag(writer, entry)?;
    }
    write_tag_id(writer, TagId::End) // End tag for compound.
}

fn write_int_array<W: Write>(writer: &mut W, value: &IntArrayTag) -> Result<()> {
    let length: IntTag = value.0.len() as i32;
    write_int(writer, length)?;
    for entry in &value.0 {
        write_int(writer, *entry)?;
    }
    Ok(())
}

fn write_long_array<W: Write>(writer: &mut W, value: &LongArrayTag) -> Result<()> {
    let length: IntTag = value.0.len() as i32;
    write_int(writer, length)?;
    for entry in &value.0 {
        write_long(writer, *entry)?;
    }
    Ok(())
}
