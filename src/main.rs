extern crate pcap;
extern crate argparse;

use pcap::{Capture, Device, Active, Savefile, Error};
use amiquip::{Connection, Exchange, Publish, Result as RMQResult};

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

fn main() {
    let target = "wlp3s0";
    let rabbit_url = "amqp://rmq:rmq@127.0.0.1:5672/%2f";

    let devices = Device::list();
    let mut requested_device : Device = Device::lookup().unwrap();
    load_device(&target, &mut requested_device, &devices);

    let mut capture = Capture::from_device(requested_device)
        .unwrap()
        .open()
        .unwrap();

    let mut connection =
        Connection::insecure_open(rabbit_url).unwrap();
    let channel = connection.open_channel(None).unwrap();
    let exchange = Exchange::direct(&channel);

    while let Ok(packet) = capture.next() {
        exchange.publish(Publish::new(&packet, "network_traffic")).unwrap();
        println!("next");
    }

    connection.close();

}
