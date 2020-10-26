extern crate pcap;
extern crate argparse;

use pcap::{Capture, Device, Active, Savefile};

fn load_device<'a>(target:&str, requested_device:&'a mut Device, vector_devices: &'a Vec<Device>){
    for device in vector_devices {
        match device {
            _ => {
                println!("network: {}, {:?}", device.name, device.desc);
                if &*device.name == target {
                    requested_device.name = device.name.clone();
                    requested_device.desc = device.desc.clone();
                }
            }
        }
    }
}

fn get_file_to_save(filename:&str, capture:&Capture<Active>) -> Savefile {
    match capture.savefile(filename) {
        Ok(f) => f,
        Err(_) => {
            println!("error");
            std::process::exit(1)
        }
    }
}

fn main() {
    let target = "wlp3s0";
    let devices = Device::list();
    let mut requested_device : Device = Device::lookup().unwrap();
    match devices {
        Ok(vector_devices) => {
            load_device(&target, &mut requested_device, &vector_devices);
        } Err(_) => {
            println!("Device not found");
            std::process::exit(1)
        }
    }
    println!("\ndetected target device: {}", requested_device.name);

    let mut capture = Capture::from_device(requested_device)
        .unwrap()
        .open()
        .unwrap();

    let mut file = get_file_to_save("./results.pcap", &capture);

    while let Ok(packet) = capture.next() {
        println!("next");
        file.write(&packet);
    }
}
