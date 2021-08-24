use avif_parse::read_avif;
use avif_serialize::serialize;
use std::{
    convert::TryInto,
    error::Error,
    io::{stdin, stdout, Cursor, Read},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input_bytes = Vec::new();
    stdin().read_to_end(&mut input_bytes)?;

    // Hacky way to find the ispe box in the AVIF metadata, just look for its
    // name, then 8 bytes after it is the image width, and 12 bytes after the height.
    let ispe_pos = input_bytes
        .windows(4)
        .position(|data| data == b"ispe")
        .ok_or("Could not find ispe box to grab image dimensions.")?;

    let parsed_input_data = read_avif(&mut Cursor::new(&input_bytes))?;

    serialize(
        &mut stdout(),
        &parsed_input_data.primary_item,
        parsed_input_data.alpha_item.as_deref(),
        u32::from_be_bytes(input_bytes[ispe_pos + 8..ispe_pos + 12].try_into()?),
        u32::from_be_bytes(input_bytes[ispe_pos + 12..ispe_pos + 16].try_into()?),
        8,
    )?;

    Ok(())
}