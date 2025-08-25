use byteorder::{BigEndian, ByteOrder, LittleEndian};
use rust_nbt::{
    BedrockHeader, ByteArrayTag, ByteTag, CompressionFormat, DoubleTag, FloatTag, IntArrayTag,
    IntTag, ListTag, LongArrayTag, LongTag, ShortTag, StringTag, Tag, decompress, from_tag,
    read_root, to_tag, write_root,
};
use serde::{Deserialize, Serialize};
use std::{fs::read, io::Result};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
struct Bigtest {
    longTest: LongTag,
    shortTest: ShortTag,
    stringTest: StringTag,
    floatTest: FloatTag,
    intTest: IntTag,
    #[serde(rename = "nested compound test")]
    nested_compound_test: NestedCompoundTest,
    #[serde(rename = "listTest (long)")]
    list_test_long: ListTag<LongTag>,
    #[serde(rename = "listTest (compound)")]
    list_test_compound: ListTestCompound,
    byteTest: ByteTag,
    doubleTest: DoubleTag,
    byteArrayTest: ByteArrayTag,
    intArrayTest: IntArrayTag,
    longArrayTest: LongArrayTag,
    escapedString: StringTag,
    escapeSequences: StringTag,
    otherEscape: StringTag,
}

#[derive(Debug, Serialize, Deserialize)]
struct NestedCompoundTest {
    ham: NestedCompoundTestEntry,
    egg: NestedCompoundTestEntry,
}

#[derive(Debug, Serialize, Deserialize)]
struct NestedCompoundTestEntry {
    name: String,
    value: FloatTag,
}

type ListTestCompound = ListTag<ListTestCompoundEntry>;

#[derive(Debug, Serialize, Deserialize)]
struct ListTestCompoundEntry {
    name: StringTag,
    #[serde(rename = "created-on")]
    created_on: LongTag,
}

fn check_symmetry_tagged<E: ByteOrder>(
    file: &str,
    root_name: &str,
    compression: Option<CompressionFormat>,
    header: BedrockHeader,
) -> Result<()> {
    println!("{}", file);

    let raw: Vec<u8> = read(file).unwrap();
    let nbt_bytes: Vec<u8> = match compression {
        Some(compression) => decompress(&raw, compression)?,
        None => raw,
    };
    println!("{:?}", &nbt_bytes[0..10]);

    // Example usage: Pass an NBT file's binary contents as a Vec<u8>
    let nbt_data: Tag = read_root::<E>(&nbt_bytes, &header)?;
    println!("{:?}", nbt_data);

    let recompile: Vec<u8> = write_root::<E>(&nbt_data, root_name, &header)?;
    println!("{:?}", &recompile[0..10]);

    assert_eq!(&nbt_bytes, &recompile);
    println!("Successful r/w!");

    Ok(())
}

fn check_symmetry_struct_validated<E: ByteOrder>(
    file: &str,
    root_name: &str,
    compression: Option<CompressionFormat>,
    header: BedrockHeader,
) -> Result<()> {
    println!("{}", file);

    let raw: Vec<u8> = read(file).unwrap();
    let nbt_bytes: Vec<u8> = match compression {
        Some(compression) => decompress(&raw, compression)?,
        None => raw,
    };
    println!("{:?}", &nbt_bytes[0..10]);

    // Example usage: Pass an NBT file's binary contents as a Vec<u8>
    let nbt_data: Tag = read_root::<E>(&nbt_bytes, &header)?;
    println!("{:?}", nbt_data);

    let struct_data: Bigtest = from_tag(nbt_data)?;
    println!("{:?}", struct_data);

    let retagged_nbt_data: Tag = to_tag(&struct_data)?;
    println!("{:?}", retagged_nbt_data);

    let recompile: Vec<u8> = write_root::<E>(&retagged_nbt_data, root_name, &header)?;
    println!("{:?}", &recompile[0..10]);

    assert_eq!(&nbt_bytes, &recompile);
    println!("Successful r/w!");

    Ok(())
}

// #[test]
fn symmetrical_nbt_be() -> Result<()> {
    check_symmetry_tagged::<BigEndian>(
        "./tests/bigtest.nbt",
        "Level",
        Some(CompressionFormat::Gzip),
        BedrockHeader::Without,
    )
}

#[test]
fn symmetrical_nbt_be_struct_validated() -> Result<()> {
    check_symmetry_struct_validated::<BigEndian>(
        "./tests/bigtest.nbt",
        "Level",
        Some(CompressionFormat::Gzip),
        BedrockHeader::Without,
    )
}

// #[test]
fn symmetrical_nbt_le() -> Result<()> {
    check_symmetry_tagged::<LittleEndian>(
        "./tests/bigtest_little.nbt",
        "Level",
        Some(CompressionFormat::Gzip),
        BedrockHeader::Without,
    )
}

// #[test]
fn symmetrical_nbt_le_bedrock_header() -> Result<()> {
    check_symmetry_tagged::<LittleEndian>("./tests/level.dat", "", None, BedrockHeader::With)
}
