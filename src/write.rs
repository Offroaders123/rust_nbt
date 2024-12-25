use crate::Tag;
use std::io::Result;

pub fn write(data: Tag) -> Result<Vec<u8>> {
    Ok(vec![0, 1, 2, 3, 4, 5])
}
