use pcap::{Activated, Capture};
use crate::save_info::save_info;
use std::error::Error;

pub fn read_packets<T: Activated>(mut packet: Capture<T>) -> Result<(), Box<dyn Error>> {
    while let Ok(packet) = packet.next_packet() {
        let x = format!("{:?}", packet.data);
        save_info(x)?;
        println!("Packet save ... {:?}", packet.data);
    }
    Ok(())
}