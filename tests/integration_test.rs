use byteorder::LittleEndian;
use rust_nbt::{decompress, read_root, write_root, BedrockHeader, CompressionFormat, Tag};
use std::{fs::read, io::Result};

#[test]
fn symmetrical_nbt() -> Result<()> {
    let file: &str = "./tests/level.dat";
    println!("{}", file);

    let nbt_bytes: Vec<u8> = read(file).unwrap().to_owned();
    // let nbt_bytes: Vec<u8> = decompress(&read(file).unwrap(), CompressionFormat::Gzip)?;
    println!("{:?}", &nbt_bytes[0..10]);

    // Example usage: Pass an NBT file's binary contents as a Vec<u8>
    let nbt_data: Tag = read_root::<LittleEndian>(&nbt_bytes, rust_nbt::BedrockHeader::With)?;
    println!("{:?}", nbt_data);

    let recompile: Vec<u8> = write_root::<LittleEndian>(&nbt_data, "", BedrockHeader::With)?;
    println!("{:?}", &recompile[0..10]);

    assert_eq!(&nbt_bytes, &recompile);
    println!("Successful r/w!");

    Ok(())
}
