use rust_nbt::{StringTag, Tag, from_tag, to_tag};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
struct Player {
    name: StringTag,
}

fn main() -> Result<(), Box<dyn Error>> {
    let player: Player = Player {
        name: "Zesty Poopoo".into(),
    };
    println!("{:#?}", player);

    let player_nbt: Tag = to_tag(&player)?;
    println!("{:#?}", player_nbt);

    let player_again: Player = from_tag(player_nbt)?;
    println!("{:#?}", player_again);

    Ok(())
}
