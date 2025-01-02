use indexmap::IndexMap;
use std::io::{Error, ErrorKind, Result};

/// Represents an NBT tag type.
#[repr(u8)]
#[derive(Clone, Debug)]
pub enum Tag {
    End = 0,
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
    pub fn id(&self) -> u8 {
        match self {
            Tag::End => 0,
            Tag::Byte(_) => 1,
            Tag::Short(_) => 2,
            Tag::Int(_) => 3,
            Tag::Long(_) => 4,
            Tag::Float(_) => 5,
            Tag::Double(_) => 6,
            Tag::ByteArray(_) => 7,
            Tag::String(_) => 8,
            Tag::List(_) => 9,
            Tag::Compound(_) => 10,
            Tag::IntArray(_) => 11,
            Tag::LongArray(_) => 12,
        }
    }
}

#[repr(u8)]
#[derive(Debug)]
pub enum RootTag {
    List(ListTag<Tag>) = 9,
    Compound(CompoundTag),
}

impl RootTag {
    pub fn from_tag(tag: Tag) -> Result<Self> {
        match tag {
            Tag::List(value) => Ok(RootTag::List(value)),
            Tag::Compound(value) => Ok(RootTag::Compound(value)),
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "Expected an opening List or Compound tag at the start of the buffer",
            )),
        }
    }

    pub fn into_tag(&self) -> Tag {
        match self {
            RootTag::List(list) => Tag::List(list.clone()),
            RootTag::Compound(compound) => Tag::Compound(compound.clone()),
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
