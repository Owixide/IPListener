use pcap::Device;
use std::io;

pub fn device_search() -> Result<Device, Box<dyn std::error::Error>> {
    let devices = Device::list()?;

    for (i, device) in devices.iter().enumerate() {
        let start = device.name.find('{');
        let end = device.name.find('}');

        if let (Some(s), Some(e)) = (start, end) {
            let id = &device.name[s + 1..e];
            println!("Device {}: {}", i + 1, id);
        } else {
            println!("Device {}: {}", i + 1, device.name);
        }
    }

    println!("Input number device ...");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let mut num: usize = input.trim().parse()?;

    num -= 1;

    if let Some(device) = devices.get(num) {
        Ok(device.clone())
    } else {
        Err("The device does not exist".into())
    }
}
