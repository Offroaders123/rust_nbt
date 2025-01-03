use indexmap::IndexMap;
use std::io::{Error, ErrorKind, Result};

/// Represents an NBT tag type.
#[repr(u8)]
#[derive(Debug)]
pub enum Tag {
    End,
    Byte(ByteTag),
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

impl Tag {
    pub fn id(&self) -> TagID {
        match self {
            Tag::End => TagID::End,
            Tag::Byte(_) => TagID::Byte,
            Tag::Short(_) => TagID::Short,
            Tag::Int(_) => TagID::Int,
            Tag::Long(_) => TagID::Long,
            Tag::Float(_) => TagID::Float,
            Tag::Double(_) => TagID::Double,
            Tag::ByteArray(_) => TagID::ByteArray,
            Tag::String(_) => TagID::String,
            Tag::List(_) => TagID::List,
            Tag::Compound(_) => TagID::Compound,
            Tag::IntArray(_) => TagID::IntArray,
            Tag::LongArray(_) => TagID::LongArray,
        }
    }
}

pub type ByteTag = i8;
pub type ShortTag = i16;
pub type IntTag = i32;
pub type LongTag = i64;
pub type FloatTag = f32;
pub type DoubleTag = f64;
pub type ByteArrayTag = Vec<i8>;
pub type StringTag = String;
pub type ListTag<T> = Vec<T>;
pub type CompoundTag = IndexMap<String, Tag>;
pub type IntArrayTag = Vec<i32>;
pub type LongArrayTag = Vec<i64>;

pub enum TagID {
    End = 0,
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
    ByteArray,
    String,
    List,
    Compound,
    IntArray,
    LongArray,
}

impl TagID {
    pub fn to_id(value: u8) -> Result<Self> {
        match value {
            0 => Ok(TagID::End),
            1 => Ok(TagID::Byte),
            2 => Ok(TagID::Short),
            3 => Ok(TagID::Int),
            4 => Ok(TagID::Long),
            5 => Ok(TagID::Float),
            6 => Ok(TagID::Double),
            7 => Ok(TagID::ByteArray),
            8 => Ok(TagID::String),
            9 => Ok(TagID::List),
            10 => Ok(TagID::Compound),
            11 => Ok(TagID::IntArray),
            12 => Ok(TagID::LongArray),
            _ => Err(Error::new(ErrorKind::InvalidData, "Unknown tag ID")),
        }
    }
}
