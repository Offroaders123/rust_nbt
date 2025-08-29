use indexmap::IndexMap;
use serde::{Deserialize, Deserializer, Serialize};

pub type ByteTag = i8;
pub type ShortTag = i16;
pub type IntTag = i32;
pub type LongTag = i64;
pub type FloatTag = f32;
pub type DoubleTag = f64;
#[derive(Debug, Serialize)]
pub struct ByteArrayTag(pub Vec<i8>);
pub type StringTag = String;
pub type ListTag<T> = Vec<T>;
pub type CompoundTag = IndexMap<String, Tag>;
#[derive(Debug, Serialize, Deserialize)]
pub struct IntArrayTag(pub Vec<i32>);
#[derive(Debug, Serialize, Deserialize)]
pub struct LongArrayTag(pub Vec<i64>);

impl<'de> Deserialize<'de> for ByteArrayTag {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Just delegate to Vec<i8>
        let v: Vec<i8> = Vec::<i8>::deserialize(deserializer)?;
        Ok(ByteArrayTag(v))
    }
}

/// Represents an NBT tag type.
#[repr(u8)]
#[derive(Debug, Serialize, Deserialize)]
pub enum Tag {
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
    pub fn id(&self) -> TagId {
        match self {
            Tag::Byte(_) => TagId::Byte,
            Tag::Short(_) => TagId::Short,
            Tag::Int(_) => TagId::Int,
            Tag::Long(_) => TagId::Long,
            Tag::Float(_) => TagId::Float,
            Tag::Double(_) => TagId::Double,
            Tag::ByteArray(_) => TagId::ByteArray,
            Tag::String(_) => TagId::String,
            Tag::List(_) => TagId::List,
            Tag::Compound(_) => TagId::Compound,
            Tag::IntArray(_) => TagId::IntArray,
            Tag::LongArray(_) => TagId::LongArray,
        }
    }
}

pub enum TagId {
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

#[derive(Debug, Serialize, Deserialize)]
pub enum TagIDError {
    UnexpectedEnd,
    UnknownType,
}

impl TryFrom<u8> for TagId {
    type Error = TagIDError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TagId::End),
            1 => Ok(TagId::Byte),
            2 => Ok(TagId::Short),
            3 => Ok(TagId::Int),
            4 => Ok(TagId::Long),
            5 => Ok(TagId::Float),
            6 => Ok(TagId::Double),
            7 => Ok(TagId::ByteArray),
            8 => Ok(TagId::String),
            9 => Ok(TagId::List),
            10 => Ok(TagId::Compound),
            11 => Ok(TagId::IntArray),
            12 => Ok(TagId::LongArray),
            _ => Err(TagIDError::UnknownType),
        }
    }
}
