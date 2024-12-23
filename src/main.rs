use rust_nbt::tag::{read_nbt_file, ListTag, Tag};
use std::fs;
use std::io::Result;

fn main() -> Result<()> {
    let file: &str = "./test/hello_world.nbt";
    println!("{}", file);

    let nbt_bytes: Vec<u8> = fs::read(file).unwrap();
    println!("{:?}", &nbt_bytes);

    let list: ListTag<Tag> = vec![Tag::Byte(5)];
    println!("{:?}", list);

    // Example usage: Pass an NBT file's binary contents as a Vec<u8>
    let nbt_data: Tag = read_nbt_file(nbt_bytes)?;
    println!("{:#?}", nbt_data);
    Ok(())
}
