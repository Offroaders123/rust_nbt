use byteorder::{BigEndian, ByteOrder, LittleEndian};
use rust_nbt::{BedrockHeader, CompressionFormat, Tag, decompress, read_root, write_root};
use std::{fs::read, io::Result};

fn check_symmetry<E: ByteOrder>(
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

#[test]
fn symmetrical_nbt_be() -> Result<()> {
    check_symmetry::<BigEndian>(
        "./tests/bigtest.nbt",
        "Level",
        Some(CompressionFormat::Gzip),
        BedrockHeader::Without,
    )
}

#[test]
fn symmetrical_nbt_le() -> Result<()> {
    check_symmetry::<LittleEndian>(
        "./tests/bigtest_little.nbt",
        "Level",
        Some(CompressionFormat::Gzip),
        BedrockHeader::Without,
    )
}

#[test]
fn symmetrical_nbt_le_bedrock_header() -> Result<()> {
    check_symmetry::<LittleEndian>("./tests/level.dat", "", None, BedrockHeader::With)
}
