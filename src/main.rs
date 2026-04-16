mod capturing_packets;
mod give_device;
mod save_info;
use pcap;
use std::io;
use crate::capturing_packets::read_packets;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device = give_device::device_search()?;
    let capture = pcap::Capture::from_device(device.clone())?
        .immediate_mode(true)
        .open()?;

    println!("{:?}", device.name);

    read_packets(capture)?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(())
}
