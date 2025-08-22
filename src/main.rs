use rust_nbt::{SerializeError, StringTag, Tag, TagSerializer};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Player {
    name: StringTag,
}

fn main() -> Result<(), SerializeError> {
    let player: Player = Player {
        name: "Zesty Poopoo".into(),
    };

    let player_nbt: Tag = player.serialize(TagSerializer)?;

    println!("{:#?}", player);
    println!("{:#?}", player_nbt);

    Ok(())
}
