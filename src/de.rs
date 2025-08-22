use indexmap::map;
use serde::{
    Deserializer,
    de::{self, DeserializeOwned, IntoDeserializer, MapAccess},
};
use std::{error, fmt};

use crate::Tag;

#[derive(Debug)]
pub struct DeserializeError(String);

impl fmt::Display for DeserializeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeserializeError(msg) => write!(f, "{}", msg),
        }
    }
}

impl error::Error for DeserializeError {}

impl de::Error for DeserializeError {
    fn custom<T>(msg: T) -> Self
    where
        T: fmt::Display,
    {
        DeserializeError(msg.to_string())
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
            None => Err(DeserializeError("Value missing".to_string())),
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
                _ => Err(DeserializeError("Expected Boolean tag".to_string())),
            },
            _ => Err(DeserializeError("Expected Boolean tag".to_string())),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.input {
            Tag::Byte(v) => visitor.visit_i8(*v),
            _ => Err(DeserializeError("Expected Byte tag".to_string())),
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.input {
            Tag::Short(v) => visitor.visit_i16(*v),
            _ => Err(DeserializeError("Expected Short tag".to_string())),
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.input {
            Tag::Int(v) => visitor.visit_i32(*v),
            _ => Err(DeserializeError("Expected Int tag".to_string())),
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.input {
            Tag::Long(v) => visitor.visit_i64(*v),
            _ => Err(DeserializeError("Expected Long tag".to_string())),
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
            _ => Err(DeserializeError("Expected Float tag".to_string())),
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.input {
            Tag::Double(v) => visitor.visit_f64(*v),
            _ => Err(DeserializeError("Expected Double tag".to_string())),
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
            _ => Err(DeserializeError("Expected String tag".to_string())),
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
        todo!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
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
            _ => Err(DeserializeError("Expected Compound tag".to_string())),
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
