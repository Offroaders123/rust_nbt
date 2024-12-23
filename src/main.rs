use std::fs;
use rust_nbt::tag::{ListTag, Tag};

fn main() -> () {
    let file: &str = "./test/hello_world.nbt";
    println!("{}", file);

    let data: Vec<u8> = fs::read(file).unwrap();
    println!("{:?}", &data);

    let mut list: ListTag<Tag> = Vec::new();
    list.push(Tag::Byte(5));
    println!("{:?}", list);
}
