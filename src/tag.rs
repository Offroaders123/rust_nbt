use std::collections::HashMap;

/// Represents an NBT tag type.
#[repr(u8)]
#[derive(Debug)]
pub enum Tag {
    End,
    Byte(ByteTag) = 1,
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

pub type ByteTag = i8;
pub type ShortTag = i16;
pub type IntTag = i32;
pub type LongTag = i64;
pub type FloatTag = f32;
pub type DoubleTag = f64;
pub type ByteArrayTag = Vec<u8>; // temp
pub type StringTag = String;
pub type ListTag<T> = Vec<T>;
pub type CompoundTag = HashMap<String, Tag>;
pub type IntArrayTag = Vec<i32>;
pub type LongArrayTag = Vec<i64>;
