use byteorder::{BigEndian, LittleEndian};
use rust_nbt::{BedrockHeader, CompressionFormat, Tag, decompress, read_root, write_root};
use std::{fs::read, io::Result};

#[test]
fn symmetrical_nbt_be() -> Result<()> {
    let file: &str = "./tests/bigtest.nbt";
    println!("{}", file);

    let nbt_bytes: Vec<u8> = decompress(&read(file).unwrap(), CompressionFormat::Gzip)?;
    println!("{:?}", &nbt_bytes[0..10]);

    // Example usage: Pass an NBT file's binary contents as a Vec<u8>
    let nbt_data: Tag = read_root::<BigEndian>(&nbt_bytes, BedrockHeader::Without)?;
    println!("{:?}", nbt_data);

    let recompile: Vec<u8> = write_root::<BigEndian>(&nbt_data, "Level", BedrockHeader::Without)?;
    println!("{:?}", &recompile[0..10]);

    assert_eq!(&nbt_bytes, &recompile);
    println!("Successful r/w!");

    Ok(())
}

#[test]
fn symmetrical_nbt_le() -> Result<()> {
    let file: &str = "./tests/bigtest_little.nbt";
    println!("{}", file);

    let nbt_bytes: Vec<u8> = decompress(&read(file).unwrap(), CompressionFormat::Gzip)?;
    println!("{:?}", &nbt_bytes[0..10]);

    // Example usage: Pass an NBT file's binary contents as a Vec<u8>
    let nbt_data: Tag = read_root::<LittleEndian>(&nbt_bytes, BedrockHeader::Without)?;
    println!("{:?}", nbt_data);

    let recompile: Vec<u8> =
        write_root::<LittleEndian>(&nbt_data, "Level", BedrockHeader::Without)?;
    println!("{:?}", &recompile[0..10]);

    assert_eq!(&nbt_bytes, &recompile);
    println!("Successful r/w!");

    Ok(())
}

#[test]
fn symmetrical_nbt_le_bedrock_header() -> Result<()> {
    let file: &str = "./tests/level.dat";
    println!("{}", file);

    let nbt_bytes: Vec<u8> = read(file).unwrap().to_owned();
    // let nbt_bytes: Vec<u8> = decompress(&read(file).unwrap(), CompressionFormat::Gzip)?;
    println!("{:?}", &nbt_bytes[0..10]);

    // Example usage: Pass an NBT file's binary contents as a Vec<u8>
    let nbt_data: Tag = read_root::<LittleEndian>(&nbt_bytes, BedrockHeader::With)?;
    println!("{:?}", nbt_data);

    let recompile: Vec<u8> = write_root::<LittleEndian>(&nbt_data, "", BedrockHeader::With)?;
    println!("{:?}", &recompile[0..10]);

    assert_eq!(&nbt_bytes, &recompile);
    println!("Successful r/w!");

    Ok(())
}
