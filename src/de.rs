use indexmap::map;
use serde::{
    Deserializer,
    de::{self, DeserializeOwned, IntoDeserializer, MapAccess, SeqAccess, value::SeqDeserializer},
};
use std::{
    error,
    fmt::{self, Debug},
    io,
};

use crate::Tag;

#[derive(Debug)]
pub enum DeserializeError {
    ExpectedByte,
    ExpectedBoolean,
    ExpectedShort,
    ExpectedInt,
    ExpectedLong,
    ExpectedFloat,
    ExpectedDouble,
    ExpectedByteArray,
    ExpectedString,
    ExpectedList,
    ExpectedCompound,
    ExpectedIntArray,
    ExpectedLongArray,
    ValueMissing,
    Custom(String),
}

impl fmt::Display for DeserializeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeserializeError::ExpectedByte => write!(f, "Expected Byte tag"),
            DeserializeError::ExpectedBoolean => write!(f, "Expected Boolean tag"),
            DeserializeError::ExpectedShort => write!(f, "Expected Short tag"),
            DeserializeError::ExpectedInt => write!(f, "Expected Int tag"),
            DeserializeError::ExpectedLong => write!(f, "Expected Long tag"),
            DeserializeError::ExpectedFloat => write!(f, "Expected Float tag"),
            DeserializeError::ExpectedDouble => write!(f, "Expected Double tag"),
            DeserializeError::ExpectedByteArray => write!(f, "Expected ByteArray tag"),
            DeserializeError::ExpectedString => write!(f, "Expected String tag"),
            DeserializeError::ExpectedList => write!(f, "Expected List tag"),
            DeserializeError::ExpectedCompound => write!(f, "Expected Compound tag"),
            DeserializeError::ExpectedIntArray => write!(f, "Expected IntArray tag"),
            DeserializeError::ExpectedLongArray => write!(f, "Expected LongArray tag"),
            DeserializeError::ValueMissing => write!(f, "Value Missing"),
            DeserializeError::Custom(msg) => write!(f, "Other error: {msg}"),
        }
    }
}

impl error::Error for DeserializeError {}

impl de::Error for DeserializeError {
    fn custom<T>(msg: T) -> Self
    where
        T: fmt::Display,
    {
        DeserializeError::Custom(msg.to_string())
    }
}

impl From<DeserializeError> for io::Error {
    fn from(value: DeserializeError) -> Self {
        io::Error::new(io::ErrorKind::InvalidData, format!("{:?}", value))
    }
}

pub struct TagDeserializer<'a> {
    input: &'a Tag,
}

impl<'a> TagDeserializer<'a> {
    pub fn new(input: &'a Tag) -> Self {
        TagDeserializer { input }
    }
}

pub fn from_tag<T: DeserializeOwned>(tag: Tag) -> Result<T, DeserializeError> {
    let deserializer: TagDeserializer = TagDeserializer::new(&tag);
    T::deserialize(deserializer)
}

struct ByteArrayAccess<'a> {
    iter: std::slice::Iter<'a, i8>,
}

impl<'de, 'a> SeqAccess<'de> for ByteArrayAccess<'a> {
    type Error = DeserializeError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        if let Some(b) = self.iter.next() {
            // Feed each i8 into Serde's machinery
            let d: de::value::I8Deserializer<DeserializeError> = (*b).into_deserializer();
            seed.deserialize(d).map(Some)
        } else {
            Ok(None)
        }
    }
}

struct IntArrayAccess<'a> {
    iter: std::slice::Iter<'a, i32>,
}

impl<'de, 'a> SeqAccess<'de> for IntArrayAccess<'a> {
    type Error = DeserializeError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        if let Some(b) = self.iter.next() {
            // Feed each i8 into Serde's machinery
            let d: de::value::I32Deserializer<DeserializeError> = (*b).into_deserializer();
            seed.deserialize(d).map(Some)
        } else {
            Ok(None)
        }
    }
}

struct LongArrayAccess<'a> {
    iter: std::slice::Iter<'a, i64>,
}

impl<'de, 'a> SeqAccess<'de> for LongArrayAccess<'a> {
    type Error = DeserializeError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        if let Some(b) = self.iter.next() {
            // Feed each i8 into Serde's machinery
            let d: de::value::I64Deserializer<DeserializeError> = (*b).into_deserializer();
            seed.deserialize(d).map(Some)
        } else {
            Ok(None)
        }
    }
}
struct ListAccess<'a> {
    iter: std::slice::Iter<'a, Tag>,
}

impl<'de, 'a> SeqAccess<'de> for ListAccess<'a> {
    type Error = DeserializeError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        if let Some(tag) = self.iter.next() {
            let de: TagDeserializer<'_> = TagDeserializer::new(tag);
            seed.deserialize(de).map(Some)
        } else {
            Ok(None)
        }
    }
}
struct CompoundAccess<'a> {
    iter: map::Iter<'a, String, Tag>,
    value: Option<&'a Tag>,
}

impl<'de, 'a> MapAccess<'de> for CompoundAccess<'a> {
    type Error = DeserializeError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        if let Some((k, v)) = self.iter.next() {
            self.value = Some(v);
            let de: de::value::StrDeserializer<'_, DeserializeError> =
                k.as_str().into_deserializer();
            seed.deserialize(de).map(Some)
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        match self.value.take() {
            Some(v) => seed.deserialize(TagDeserializer { input: v }),
            None => Err(DeserializeError::ValueMissing),
        }
    }
}

impl<'de> Deserializer<'de> for TagDeserializer<'_> {
    type Error = DeserializeError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.input {
            Tag::Byte(v) => match v {
                0 | 1 => visitor.visit_bool(*v != 0),
                _ => Err(DeserializeError::ExpectedBoolean),
            },
            _ => Err(DeserializeError::ExpectedBoolean),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.input {
            Tag::Byte(v) => visitor.visit_i8(*v),
            _ => Err(DeserializeError::ExpectedByte),
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.input {
            Tag::Short(v) => visitor.visit_i16(*v),
            _ => Err(DeserializeError::ExpectedShort),
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.input {
            Tag::Int(v) => visitor.visit_i32(*v),
            _ => Err(DeserializeError::ExpectedInt),
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.input {
            Tag::Long(v) => visitor.visit_i64(*v),
            _ => Err(DeserializeError::ExpectedLong),
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.input {
            Tag::Float(v) => visitor.visit_f32(*v),
            _ => Err(DeserializeError::ExpectedFloat),
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.input {
            Tag::Double(v) => visitor.visit_f64(*v),
            _ => Err(DeserializeError::ExpectedDouble),
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.input {
            Tag::String(v) => visitor.visit_string(v.clone()),
            _ => Err(DeserializeError::ExpectedString),
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match (name, self.input) {
            ("ByteArray", Tag::ByteArray(bytes)) => {
                // Drive the visitor with a custom SeqAccess that yields i8s
                let access: ByteArrayAccess<'_> = ByteArrayAccess {
                    iter: bytes.0.iter(),
                };
                visitor.visit_seq(access)
            }
            ("IntArray", Tag::IntArray(bytes)) => {
                // Drive the visitor with a custom SeqAccess that yields i8s
                let access: IntArrayAccess<'_> = IntArrayAccess {
                    iter: bytes.0.iter(),
                };
                visitor.visit_seq(access)
            }
            ("LongArray", Tag::LongArray(bytes)) => {
                // Drive the visitor with a custom SeqAccess that yields i8s
                let access: LongArrayAccess<'_> = LongArrayAccess {
                    iter: bytes.0.iter(),
                };
                visitor.visit_seq(access)
            }
            _ => {
                // Fallback: let the visitor handle this input directly
                visitor.visit_newtype_struct(self)
            }
        }
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        println!("{:?}", self.input);

        match self.input {
            Tag::List(elements) => {
                // Create a SeqAccess wrapper around the list
                let access: ListAccess<'_> = ListAccess {
                    iter: elements.0.iter(),
                };
                visitor.visit_seq(access)
            }
            Tag::ByteArray(bytes) => {
                // Either Option 1: custom SeqAccess
                let access = ByteArrayAccess {
                    iter: bytes.0.iter(),
                };
                visitor.visit_seq(access)

                // Or Option 2 (no ByteArrayAccess needed):
                // use serde::de::value::SeqDeserializer;
                // use serde::de::IntoDeserializer;
                // let seq_de = SeqDeserializer::new(bytes.iter().cloned().map(|b| b.into_deserializer()));
                // visitor.visit_seq(seq_de)
            }
            Tag::IntArray(bytes) => {
                // Either Option 1: custom SeqAccess
                let access = IntArrayAccess {
                    iter: bytes.0.iter(),
                };
                visitor.visit_seq(access)

                // Or Option 2 (no IntArrayAccess needed):
                // use serde::de::value::SeqDeserializer;
                // use serde::de::IntoDeserializer;
                // let seq_de = SeqDeserializer::new(bytes.iter().cloned().map(|b| b.into_deserializer()));
                // visitor.visit_seq(seq_de)
            }
            Tag::LongArray(bytes) => {
                // Either Option 1: custom SeqAccess
                let access = LongArrayAccess {
                    iter: bytes.0.iter(),
                };
                visitor.visit_seq(access)

                // Or Option 2 (no LongArrayAccess needed):
                // use serde::de::value::SeqDeserializer;
                // use serde::de::IntoDeserializer;
                // let seq_de = SeqDeserializer::new(bytes.iter().cloned().map(|b| b.into_deserializer()));
                // visitor.visit_seq(seq_de)
            }
            _ => Err(DeserializeError::ExpectedList),
        }
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.input {
            Tag::Compound(v) => {
                let access: CompoundAccess<'_> = CompoundAccess {
                    iter: v.iter(),
                    value: None,
                };
                visitor.visit_map(access)
            }
            _ => Err(DeserializeError::ExpectedCompound),
        }
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }
}
