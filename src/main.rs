use std::io::Result;

use rust_nbt::{CompoundTag, StringTag};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Player {
    name: StringTag,
}

fn main() -> Result<()> {
    let player: Player = Player {
        name: "Zesty Poopoo".into(),
    };

    let player_nbt: CompoundTag = player.serialize();

    Ok(())
}
