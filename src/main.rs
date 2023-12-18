// cargo watch -c -q -x "run -q"

use std::fs::read;

fn main() {
    let file: &str = "../NBTify/test/nbt/hello_world.nbt";
    println!("{}",file);

    let data: Vec<u8> = read(file).unwrap();
    println!("{:?}",&data);
}

// https://kerkour.com/rust-functional-programming
// https://dev.to/jorgecastro/hot-reload-in-rust-with-cargo-watch-5bon
// https://doc.rust-lang.org/cargo/commands/cargo-run.html
// https://stackoverflow.com/questions/31192956/whats-the-de-facto-way-of-reading-and-writing-files-in-rust-1-x
// https://stackoverflow.com/questions/28123923/how-do-i-print-a-vector-of-u8-as-a-string
// https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/std/path/struct.Path.html