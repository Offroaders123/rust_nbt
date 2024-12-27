use rust_nbt::{decompress, read, write, ListTag, Tag};
use std::fs;
use std::io::Result;

fn main() -> Result<()> {
    let file: &str = "./test/bigtest.nbt";
    println!("{}", file);

    let nbt_bytes: Vec<u8> =
        decompress(&fs::read(file).unwrap(), rust_nbt::CompressionFormat::Gzip)?;
    println!("{:?}", &nbt_bytes);

    let list: ListTag<Tag> = vec![Tag::Byte(5)];
    println!("{:?}", list);

    // Example usage: Pass an NBT file's binary contents as a Vec<u8>
    let nbt_data: Tag = read(nbt_bytes)?;
    println!("{:#?}", nbt_data);

    let recompile: Vec<u8> = write(&nbt_data, "Level")?;
    println!("{:?}", &recompile);

    assert_eq!(&nbt_bytes, &recompile);
    Ok(())
}
