extern crate pcap;
extern crate argparse;

use pcap::{Capture, Device, Active, Savefile, Error};

fn load_device<'a>(target:&str, requested_device:&'a mut Device, devices: &'a Result<Vec<Device>,Error>){
    match devices {
        Ok(vector_devices) => {
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
        } Err(_) => {
            println!("Device not found");
            std::process::exit(1)
        }
    }
    println!("\ndetected target device: {}", requested_device.name);
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
    load_device(&target, &mut requested_device, &devices);

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
