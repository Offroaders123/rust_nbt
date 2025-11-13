use crate::{
    BedrockHeader, ByteArrayTag, ByteTag, CompoundTag, DoubleTag, Endian, FloatTag, IntArrayTag,
    IntTag, ListTag, LongArrayTag, LongTag, ShortTag, Tag, TagId,
};
use byteorder::{BigEndian, LittleEndian, WriteBytesExt};
use std::io::{self, Cursor, Seek, SeekFrom, Write};

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
pub fn write_root(
    tag: &Tag,
    root_name: &str,
    endian: &Endian,
    header: &BedrockHeader,
) -> Result<Vec<u8>, WriteError> {
    let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());

    // If we’ll write a header, reserve 8 bytes (two u32s) at the start
    let header_bytes: usize = 8usize;
    let wrote_header: bool = matches!(header, BedrockHeader::With);
    if wrote_header {
        cursor.write_all(&[0u8; 8])?; // placeholders
    }

    // Write the payload after the (possible) header space
    write_tag_id(&mut cursor, tag.id())?;
    write_string(&mut cursor, endian, root_name)?;
    write_tag(&mut cursor, endian, tag)?;
    if matches!(header, BedrockHeader::With) {
        let total_len: usize = cursor.get_ref().len();
        let payload_len: i32 = (total_len - header_bytes) as i32;
        cursor.seek(SeekFrom::Start(0))?;
        write_bedrock_header(tag, &mut cursor, payload_len)?
    }
    Ok(cursor.into_inner())
}

fn write_bedrock_header(
    data: &Tag,
    writer: &mut impl Write,
    payload_len: i32,
) -> Result<(), WriteError> {
    let storage_version: i32 = get_storage_version(data)?;
    writer.write_u32::<LittleEndian>(storage_version as u32)?;
    writer.write_u32::<LittleEndian>(payload_len as u32)?;
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
fn write_tag(writer: &mut impl Write, endian: &Endian, tag: &Tag) -> Result<(), WriteError> {
    match tag {
        Tag::Byte(value) => write_byte(writer, *value),
        Tag::Short(value) => write_short(writer, endian, *value),
        Tag::Int(value) => write_int(writer, endian, *value),
        Tag::Long(value) => write_long(writer, endian, *value),
        Tag::Float(value) => write_float(writer, endian, *value),
        Tag::Double(value) => write_double(writer, endian, *value),
        Tag::ByteArray(data) => write_byte_array(writer, endian, data),
        Tag::String(value) => write_string(writer, endian, value),
        Tag::List(list) => write_list(writer, endian, list),
        Tag::Compound(compound) => write_compound(writer, endian, compound),
        Tag::IntArray(data) => write_int_array(writer, endian, data),
        Tag::LongArray(data) => write_long_array(writer, endian, data),
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

fn write_unsigned_short(
    writer: &mut impl Write,
    endian: &Endian,
    value: u16,
) -> Result<(), WriteError> {
    match endian {
        Endian::Big => Ok(writer.write_u16::<BigEndian>(value)?),
        Endian::Little => Ok(writer.write_u16::<LittleEndian>(value)?),
        Endian::LittleVarInt => Ok(writer.write_u16::<LittleEndian>(value)?),
    }
}

fn write_short(
    writer: &mut impl Write,
    endian: &Endian,
    value: ShortTag,
) -> Result<(), WriteError> {
    match endian {
        Endian::Big => Ok(writer.write_i16::<BigEndian>(value)?),
        Endian::Little => Ok(writer.write_i16::<LittleEndian>(value)?),
        Endian::LittleVarInt => Ok(writer.write_i16::<LittleEndian>(value)?),
    }
}

fn write_int(writer: &mut impl Write, endian: &Endian, value: IntTag) -> Result<(), WriteError> {
    match endian {
        Endian::Big => Ok(writer.write_i32::<BigEndian>(value)?),
        Endian::Little => Ok(writer.write_i32::<LittleEndian>(value)?),
        Endian::LittleVarInt => Ok(writer.write_i32::<LittleEndian>(value)?),
    }
}

fn write_long(writer: &mut impl Write, endian: &Endian, value: LongTag) -> Result<(), WriteError> {
    match endian {
        Endian::Big => Ok(writer.write_i64::<BigEndian>(value)?),
        Endian::Little => Ok(writer.write_i64::<LittleEndian>(value)?),
        Endian::LittleVarInt => Ok(writer.write_i64::<LittleEndian>(value)?),
    }
}

fn write_float(
    writer: &mut impl Write,
    endian: &Endian,
    value: FloatTag,
) -> Result<(), WriteError> {
    match endian {
        Endian::Big => Ok(writer.write_f32::<BigEndian>(value)?),
        Endian::Little => Ok(writer.write_f32::<LittleEndian>(value)?),
        Endian::LittleVarInt => Ok(writer.write_f32::<LittleEndian>(value)?),
    }
}

fn write_double(
    writer: &mut impl Write,
    endian: &Endian,
    value: DoubleTag,
) -> Result<(), WriteError> {
    match endian {
        Endian::Big => Ok(writer.write_f64::<BigEndian>(value)?),
        Endian::Little => Ok(writer.write_f64::<LittleEndian>(value)?),
        Endian::LittleVarInt => Ok(writer.write_f64::<LittleEndian>(value)?),
    }
}

fn write_byte_array(
    writer: &mut impl Write,
    endian: &Endian,
    value: &ByteArrayTag,
) -> Result<(), WriteError> {
    let length: IntTag = value.0.len() as i32;
    write_int(writer, endian, length)?;
    for entry in &value.0 {
        write_byte(writer, *entry)?;
    }
    Ok(())
}

fn write_string(writer: &mut impl Write, endian: &Endian, value: &str) -> Result<(), WriteError> {
    let entry: &[u8] = value.as_bytes();
    let length: u16 = value.len() as u16;
    write_unsigned_short(writer, endian, length)?;
    Ok(writer.write_all(entry)?)
}

fn write_list(
    writer: &mut impl Write,
    endian: &Endian,
    value: &ListTag<Tag>,
) -> Result<(), WriteError> {
    if let Some(first_entry) = value.first() {
        let tag_id: TagId = first_entry.id();
        let length: IntTag = value.len() as i32;
        write_tag_id(writer, tag_id)?;
        write_int(writer, endian, length)?;
        for entry in value {
            write_tag(writer, endian, entry)?;
        }
    } else {
        write_tag_id(writer, TagId::End)?; // Empty list type.
        write_int(writer, endian, 0)?; // Empty list length.
    }
    Ok(())
}

fn write_compound(
    writer: &mut impl Write,
    endian: &Endian,
    value: &CompoundTag,
) -> Result<(), WriteError> {
    for (name, entry) in value {
        let tag_id: TagId = entry.id();
        write_tag_id(writer, tag_id)?;
        write_string(writer, endian, name)?;
        write_tag(writer, endian, entry)?;
    }
    write_tag_id(writer, TagId::End) // End tag for compound.
}

fn write_int_array(
    writer: &mut impl Write,
    endian: &Endian,
    value: &IntArrayTag,
) -> Result<(), WriteError> {
    let length: IntTag = value.0.len() as i32;
    write_int(writer, endian, length)?;
    for entry in &value.0 {
        write_int(writer, endian, *entry)?;
    }
    Ok(())
}

fn write_long_array(
    writer: &mut impl Write,
    endian: &Endian,
    value: &LongArrayTag,
) -> Result<(), WriteError> {
    let length: IntTag = value.0.len() as i32;
    write_int(writer, endian, length)?;
    for entry in &value.0 {
        write_long(writer, endian, *entry)?;
    }
    Ok(())
}
