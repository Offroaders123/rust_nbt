use std::fs;
mod tag;
// use crate::tag::{ Tag, IntTag };
use crate::tag::Tag;

fn main() {
    let file: &str = "./test/hello_world.nbt";
    println!("{}", file);

    let data: Vec<u8> = fs::read(file).unwrap();
    println!("{:?}", &data);

    let haha: Tag = Tag::Byte(25);
    println!("{:?}", haha);

    // let haha2: IntTag = Tag::Int(25);
    // println!("{:?}",haha2);
}
