use std::io::{Error, ErrorKind, Result};

use rust_nbt::{StringTag, Tag, TagSerializer};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Player {
    name: StringTag,
}

fn main() -> Result<()> {
    let player: Player = Player {
        name: "Zesty Poopoo".into(),
    };

    let player_nbt: Tag = player
        .serialize(TagSerializer)
        .map_err(|e| Error::new(ErrorKind::InvalidData, format!("{e}")))?;

    println!("{:#?}", player);
    println!("{:#?}", player_nbt);

    Ok(())
}
