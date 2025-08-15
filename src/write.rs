use crate::{
    BedrockHeader, ByteArrayTag, ByteTag, CompoundTag, DoubleTag, FloatTag, IntArrayTag, IntTag,
    ListTag, LongArrayTag, LongTag, ShortTag, Tag, TagId,
};
use byteorder::{ByteOrder, WriteBytesExt};
use std::io::{self, Cursor, Write};

#[derive(Debug)]
pub enum WriteError {
    IoError(io::Error),
    ExpectedInt,
    ExpectedCompound,
    ExpectedStorageVersion,
}

impl From<io::Error> for WriteError {
    fn from(value: io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<WriteError> for io::Error {
    fn from(value: WriteError) -> Self {
        match value {
            WriteError::IoError(e) => e,
            e => io::Error::new(io::ErrorKind::InvalidData, format!("{:?}", e)),
        }
    }
}

/// Writes an NBT file to a byte vector, starting with the root compound tag.
pub fn write_root<E: ByteOrder>(
    tag: &Tag,
    root_name: &str,
    header: BedrockHeader,
) -> Result<Vec<u8>, WriteError> {
    let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    write_tag_id(&mut cursor, tag.id())?;
    write_string::<E>(&mut cursor, root_name)?;
    write_tag::<E>(&mut cursor, tag)?;
    match header {
        BedrockHeader::With => {
            let payload_len: i32 = cursor.get_ref().len() as i32;
            write_bedrock_header::<E>(tag, &mut cursor, payload_len)?
        }
        _ => (),
    }
    Ok(cursor.into_inner())
}

fn write_bedrock_header<E: ByteOrder>(
    data: &Tag,
    writer: &mut impl Write,
    payload_len: i32,
) -> Result<(), WriteError> {
    let storage_version: i32 = get_storage_version(data)?;
    writer.write_u32::<E>(storage_version as u32)?;
    writer.write_u32::<E>(payload_len as u32)?;
    Ok(())
}

fn get_storage_version(data: &Tag) -> Result<i32, WriteError> {
    match data {
        Tag::Compound(tag) => {
            let storage_version: i32 = match tag
                .get("StorageVersion")
                .ok_or(WriteError::ExpectedStorageVersion)?
            {
                Tag::Int(v) => Ok(*v),
                _ => Err(WriteError::ExpectedInt),
            }?;
            Ok(storage_version)
        }
        _ => Err(WriteError::ExpectedCompound),
    }
}

/// Writes a single NBT tag to the given writer.
fn write_tag<E: ByteOrder>(writer: &mut impl Write, tag: &Tag) -> Result<(), WriteError> {
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

fn write_tag_id(writer: &mut impl Write, tag_id: TagId) -> Result<(), WriteError> {
    let value: u8 = tag_id as u8;
    write_unsigned_byte(writer, value)
}

/// Helper functions to write various data types to a writer.
fn write_unsigned_byte(writer: &mut impl Write, value: u8) -> Result<(), WriteError> {
    Ok(writer.write_u8(value)?)
}

fn write_byte(writer: &mut impl Write, value: ByteTag) -> Result<(), WriteError> {
    Ok(writer.write_i8(value)?)
}

fn write_unsigned_short<E: ByteOrder>(
    writer: &mut impl Write,
    value: u16,
) -> Result<(), WriteError> {
    Ok(writer.write_u16::<E>(value)?)
}

fn write_short<E: ByteOrder>(writer: &mut impl Write, value: ShortTag) -> Result<(), WriteError> {
    Ok(writer.write_i16::<E>(value)?)
}

fn write_int<E: ByteOrder>(writer: &mut impl Write, value: IntTag) -> Result<(), WriteError> {
    Ok(writer.write_i32::<E>(value)?)
}

fn write_long<E: ByteOrder>(writer: &mut impl Write, value: LongTag) -> Result<(), WriteError> {
    Ok(writer.write_i64::<E>(value)?)
}

fn write_float<E: ByteOrder>(writer: &mut impl Write, value: FloatTag) -> Result<(), WriteError> {
    Ok(writer.write_f32::<E>(value)?)
}

fn write_double<E: ByteOrder>(writer: &mut impl Write, value: DoubleTag) -> Result<(), WriteError> {
    Ok(writer.write_f64::<E>(value)?)
}

fn write_byte_array<E: ByteOrder>(
    writer: &mut impl Write,
    value: &ByteArrayTag,
) -> Result<(), WriteError> {
    let length: IntTag = value.0.len() as i32;
    write_int::<E>(writer, length)?;
    for entry in &value.0 {
        write_byte(writer, *entry)?;
    }
    Ok(())
}

fn write_string<E: ByteOrder>(writer: &mut impl Write, value: &str) -> Result<(), WriteError> {
    let entry: &[u8] = value.as_bytes();
    let length: u16 = value.len() as u16;
    write_unsigned_short::<E>(writer, length)?;
    Ok(writer.write_all(entry)?)
}

fn write_list<E: ByteOrder>(
    writer: &mut impl Write,
    value: &ListTag<Tag>,
) -> Result<(), WriteError> {
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

fn write_compound<E: ByteOrder>(
    writer: &mut impl Write,
    value: &CompoundTag,
) -> Result<(), WriteError> {
    for (name, entry) in value {
        let tag_id: TagId = entry.id();
        write_tag_id(writer, tag_id)?;
        write_string::<E>(writer, name)?;
        write_tag::<E>(writer, entry)?;
    }
    write_tag_id(writer, TagId::End) // End tag for compound.
}

fn write_int_array<E: ByteOrder>(
    writer: &mut impl Write,
    value: &IntArrayTag,
) -> Result<(), WriteError> {
    let length: IntTag = value.0.len() as i32;
    write_int::<E>(writer, length)?;
    for entry in &value.0 {
        write_int::<E>(writer, *entry)?;
    }
    Ok(())
}

fn write_long_array<E: ByteOrder>(
    writer: &mut impl Write,
    value: &LongArrayTag,
) -> Result<(), WriteError> {
    let length: IntTag = value.0.len() as i32;
    write_int::<E>(writer, length)?;
    for entry in &value.0 {
        write_long::<E>(writer, *entry)?;
    }
    Ok(())
}
