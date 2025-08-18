use indexmap::IndexMap;
use serde::{
    de::{self, Deserializer, IntoDeserializer, MapAccess, Visitor},
    forward_to_deserialize_any,
};

pub type ByteTag = i8;
pub type ShortTag = i16;
pub type IntTag = i32;
pub type LongTag = i64;
pub type FloatTag = f32;
pub type DoubleTag = f64;
#[derive(Debug)]
pub struct ByteArrayTag(pub Vec<i8>);
pub type StringTag = String;
pub type ListTag<T> = Vec<T>;
pub type CompoundTag = IndexMap<String, Tag>;
#[derive(Debug)]
pub struct IntArrayTag(pub Vec<i32>);
#[derive(Debug)]
pub struct LongArrayTag(pub Vec<i64>);

/// Represents an NBT tag type.
#[repr(u8)]
#[derive(Debug)]
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

impl<'de> Deserializer<'de> for Tag {
    type Error = de::value::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Tag::Byte(b) => visitor.visit_i8(b),
            Tag::String(s) => visitor.visit_string(s),
            Tag::Compound(map) => {
                // Tell Serde "this is a map-like structure"
                struct CompoundAccess {
                    iter: indexmap::map::IntoIter<String, Tag>,
                    value: Option<Tag>,
                }

                impl<'de> MapAccess<'de> for CompoundAccess {
                    type Error = de::value::Error;

                    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
                    where
                        K: de::DeserializeSeed<'de>,
                    {
                        if let Some((k, v)) = self.iter.next() {
                            self.value = Some(v);
                            seed.deserialize(k.into_deserializer()).map(Some)
                        } else {
                            Ok(None)
                        }
                    }

                    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
                    where
                        V: de::DeserializeSeed<'de>,
                    {
                        seed.deserialize(self.value.take().unwrap())
                    }
                }

                let access = CompoundAccess {
                    iter: map.into_iter(),
                    value: None,
                };
                visitor.visit_map(access)
            }
            _ => Err(de::Error::custom("unsupported tag in this sketch")),
        }
    }

    // For simplicity, forward to `deserialize_any`.
    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Tag::Byte(b) => visitor.visit_i8(b),
            other => Err(de::Error::invalid_type(
                de::Unexpected::Other(&format!("{:?}", other)),
                &"i8",
            )),
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Tag::String(s) => visitor.visit_str(&s),
            other => Err(de::Error::invalid_type(
                de::Unexpected::Other(&format!("{:?}", other)),
                &"string",
            )),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Tag::String(s) => visitor.visit_string(s),
            other => Err(de::Error::invalid_type(
                de::Unexpected::Other(&format!("{:?}", other)),
                &"string",
            )),
        }
    }

    // You can leave the rest unimplemented for now, or forward them to `deserialize_any`.
    forward_to_deserialize_any! {
        bool i16 i32 i64 u8 u16 u32 u64 f32 f64 char bytes byte_buf option unit unit_struct
        newtype_struct seq tuple tuple_struct map struct enum identifier ignored_any
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

#[derive(Debug)]
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
