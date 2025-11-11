use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use std::io::{Cursor, Error, ErrorKind, Read, Result};

/// --- ENUM to track high-level endian "mode" if you need it at runtime ---
#[derive(Copy, Clone, Debug)]
pub enum Endian {
    Big,
    Little,
    LittleVarInt,
}

/// --- TRAIT: defines how to read various integer types ---
pub trait EndianRead {
    fn read_u16(reader: &mut impl Read) -> Result<u16>;
    fn read_i32(reader: &mut impl Read) -> Result<i32>;
    fn read_i64(reader: &mut impl Read) -> Result<i64>;
}

/// --- IMPLEMENTATION: BigEndian fixed-width ---
impl EndianRead for BigEndian {
    fn read_u16(reader: &mut impl Read) -> Result<u16> {
        reader.read_u16::<BigEndian>()
    }
    fn read_i32(reader: &mut impl Read) -> Result<i32> {
        reader.read_i32::<BigEndian>()
    }
    fn read_i64(reader: &mut impl Read) -> Result<i64> {
        reader.read_i64::<BigEndian>()
    }
}

/// --- IMPLEMENTATION: LittleEndian fixed-width ---
impl EndianRead for LittleEndian {
    fn read_u16(reader: &mut impl Read) -> Result<u16> {
        reader.read_u16::<LittleEndian>()
    }
    fn read_i32(reader: &mut impl Read) -> Result<i32> {
        reader.read_i32::<LittleEndian>()
    }
    fn read_i64(reader: &mut impl Read) -> Result<i64> {
        reader.read_i64::<LittleEndian>()
    }
}

/// --- NEW TYPE for VarInt decoding ---
pub struct LittleVarInt;

impl EndianRead for LittleVarInt {
    fn read_u16(reader: &mut impl Read) -> Result<u16> {
        read_varint(reader).map(|v| v as u16)
    }
    fn read_i32(reader: &mut impl Read) -> Result<i32> {
        read_varint(reader).map(|v| v as i32)
    }
    fn read_i64(reader: &mut impl Read) -> Result<i64> {
        read_varint(reader).map(|v| v as i64)
    }
}

/// --- GENERIC READ FUNCTION ---
/// Works for *any* endian/encoding that implements EndianRead.
pub fn read_int<E: EndianRead>(reader: &mut impl Read) -> Result<i32> {
    E::read_i32(reader)
}

/// --- EXAMPLE VARINT READER ---
/// Common "LEB128"-style (7 bits per byte, MSB = continuation)
fn read_varint(reader: &mut impl Read) -> Result<u64> {
    let mut result: u64 = 0;
    let mut shift = 0;

    loop {
        let byte = reader.read_u8()? as u64;
        result |= (byte & 0x7F) << shift;
        if (byte & 0x80) == 0 {
            break;
        }
        shift += 7;
        if shift >= 64 {
            return Err(Error::new(ErrorKind::InvalidData, "VarInt too long"));
        }
    }

    Ok(result)
}

/// --- DEMO ---
fn main() -> Result<()> {
    let data_big = [0x00, 0x00, 0x01, 0x23];
    let mut cursor_big = Cursor::new(data_big);
    let val_big = read_int::<BigEndian>(&mut cursor_big)?;
    println!("Big endian read: {val_big}");

    let data_little = [0x23, 0x01, 0x00, 0x00];
    let mut cursor_little = Cursor::new(data_little);
    let val_little = read_int::<LittleEndian>(&mut cursor_little)?;
    println!("Little endian read: {val_little}");

    // VarInt example: 0xAC 0x02  => (0x2C = 300 decimal)
    let data_varint = [0xAC, 0x02];
    let mut cursor_varint = Cursor::new(data_varint);
    let val_var = read_int::<LittleVarInt>(&mut cursor_varint)?;
    println!("VarInt read: {val_var}");

    Ok(())
}
