use rust_nbt::{decompress, read, write, RootTag};
use std::fs;
use std::io::Result;

fn main() -> Result<()> {
    let file: &str = "./test/bigtest.nbt";
    println!("{}", file);

    let nbt_bytes: Vec<u8> =
        decompress(&fs::read(file).unwrap(), rust_nbt::CompressionFormat::Gzip)?;
    println!("{:?}", &nbt_bytes[0..10]);

    // Example usage: Pass an NBT file's binary contents as a Vec<u8>
    let nbt_data: RootTag = read(&nbt_bytes)?;
    println!("{:#?}", nbt_data);

    let recompile: Vec<u8> = write(&nbt_data, "Level")?;
    println!("{:?}", &recompile[0..10]);

    assert_eq!(&nbt_bytes, &recompile);
    println!("Successful r/w!");

    Ok(())
}
