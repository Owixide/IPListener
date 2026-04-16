use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn save_info(datapacket: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("packets.txt")?;

    writeln!(file, "{}", datapacket)?;

    Ok(())
}
