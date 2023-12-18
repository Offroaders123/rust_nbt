use std::fs;

fn main() {
    let file: &str = "./test/hello_world.nbt";
    println!("{}",file);

    let data: Vec<u8> = fs::read(file).unwrap();
    println!("{:?}",&data);
}