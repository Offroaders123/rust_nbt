use serde::ser::{self, Serialize, Serializer};
use std::{error, fmt, io};

use crate::{ByteArrayTag, CompoundTag, ListTag, Tag};

#[derive(Debug)]
pub struct SerializeError(String);

impl fmt::Display for SerializeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SerializeError(msg) => write!(f, "{}", msg),
        }
    }
}

impl error::Error for SerializeError {}

impl ser::Error for SerializeError {
    fn custom<T>(msg: T) -> Self
    where
        T: fmt::Display,
    {
        SerializeError(msg.to_string())
    }
}

impl From<SerializeError> for io::Error {
    fn from(value: SerializeError) -> Self {
        match value {
            SerializeError(e) => io::Error::new(io::ErrorKind::InvalidData, format!("{:?}", e)),
        }
    }
}

pub struct TagSerializer;

pub fn to_tag<T: Serialize>(value: &T) -> Result<Tag, SerializeError> {
    value.serialize(TagSerializer)
}

impl From<&[u8]> for ByteArrayTag {
    fn from(slice: &[u8]) -> Self {
        ByteArrayTag(slice.iter().map(|&b| b as i8).collect())
    }
}

impl Serializer for TagSerializer {
    type Ok = Tag;
    type Error = SerializeError;
    type SerializeSeq = SerializeList;
    type SerializeTuple = SerializeList;
    type SerializeTupleStruct = SerializeList;
    type SerializeTupleVariant = SerializeList;
    type SerializeMap = SerializeCompound;
    type SerializeStruct = SerializeCompound;
    type SerializeStructVariant = SerializeCompound;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(Tag::Byte(if v { 1 } else { 0 }))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(Tag::Byte(v))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(Tag::Short(v))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(Tag::Int(v))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(Tag::Long(v))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(Tag::Byte(v as i8))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(Tag::Short(v as i16))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(Tag::Int(v as i32))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(Tag::Long(v as i64))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(Tag::Float(v))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(Tag::Double(v))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(Tag::String(v.to_string()))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(Tag::String(v.to_string()))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(Tag::ByteArray(ByteArrayTag::from(v)))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError("Cannot serialize type 'none'".to_string()))
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(TagSerializer)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError("Cannot serialize type 'unit'".to_string()))
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError(
            "Cannot serialize type 'unit_struct'".to_string(),
        ))
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(Tag::String(variant.to_string()))
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        println!("{name}");

        value.serialize(TagSerializer)
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let mut map: CompoundTag = CompoundTag::new();
        map.insert(variant.to_string(), value.serialize(TagSerializer)?);
        Ok(Tag::Compound(map))
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(SerializeList {
            elements: ListTag::new(),
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(SerializeList {
            elements: ListTag::with_capacity(len),
        })
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(SerializeCompound {
            fields: CompoundTag::new(),
        })
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(SerializeCompound {
            fields: CompoundTag::new(),
        })
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        let mut fields: CompoundTag = CompoundTag::new();
        fields.insert(variant.to_string(), Tag::Compound(CompoundTag::new()));
        Ok(SerializeCompound { fields })
    }
}

pub struct SerializeList {
    elements: Vec<Tag>,
}

impl ser::SerializeSeq for SerializeList {
    type Ok = Tag;
    type Error = SerializeError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.elements.push(value.serialize(TagSerializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Tag::List(self.elements))
    }
}

impl ser::SerializeTuple for SerializeList {
    type Ok = Tag;
    type Error = SerializeError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        ser::SerializeSeq::end(self)
    }
}

impl ser::SerializeTupleStruct for SerializeList {
    type Ok = Tag;
    type Error = SerializeError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.elements.push(value.serialize(TagSerializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Tag::List(self.elements))
    }
}

impl ser::SerializeTupleVariant for SerializeList {
    type Ok = Tag;
    type Error = SerializeError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.elements.push(value.serialize(TagSerializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Tag::List(self.elements))
    }
}

pub struct SerializeCompound {
    fields: CompoundTag,
}

impl ser::SerializeMap for SerializeCompound {
    type Ok = Tag;
    type Error = SerializeError;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        // store key temporarily? (NBT requires String keys)
        Err(SerializeError(
            "serialize_key not directly supported, use serialize_struct".into(),
        ))
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(SerializeError("serialize_value not supported".into()))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Tag::Compound(self.fields))
    }
}

impl ser::SerializeStruct for SerializeCompound {
    type Ok = Tag;
    type Error = SerializeError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.fields
            .insert(key.to_string(), value.serialize(TagSerializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Tag::Compound(self.fields))
    }
}

impl ser::SerializeStructVariant for SerializeCompound {
    type Ok = Tag;
    type Error = SerializeError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.fields
            .insert(key.to_string(), value.serialize(TagSerializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Tag::Compound(self.fields))
    }
}
