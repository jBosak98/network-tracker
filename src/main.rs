use pcap::{Capture, Device, Error as PcapError};
use amiquip::{ExchangeDeclareOptions, Error, Connection, Publish, QueueDeclareOptions, ExchangeType, FieldTable, Exchange, Channel};

fn load_device<'a>(target:&str, requested_device:&'a mut Device, devices: &'a Result<Vec<Device>,PcapError>){
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
fn create_exchange<'a>(
    channel:&'a Channel,
    queue_name: &str,
    exchange_name:&str,
    routing_key:&str
) -> Exchange<'a> {

    let queue= channel.queue_declare(
        queue_name,
        QueueDeclareOptions::default())
        .unwrap();
    let exchange = channel.exchange_declare(
        ExchangeType::Direct,
        exchange_name,
        ExchangeDeclareOptions::default()
    ).unwrap();

    channel.queue_bind(
        queue.name(),
        exchange.name(),
        routing_key,
        FieldTable::new()
    ).unwrap();
    println!("channel id: {}", channel.channel_id());
    exchange
}
fn main() -> Result<(), Error> {
    let target = "wlp3s0";
    let rabbit_url = "amqp://rmq:rmq@127.0.0.1:5672/%2f";
    let routing_key ="network_traffic";
    let queue_name = "hello_queue";
    let exchange_name = "hello_exchange";

    let devices = Device::list();
    let mut requested_device: Device = Device::lookup().unwrap();
    load_device(&target, &mut requested_device, &devices);

    let mut capture = Capture::from_device(requested_device)
        .unwrap()
        .open()
        .unwrap();
    let mut connection =
        Connection::insecure_open(rabbit_url).unwrap();
    let channel = connection.open_channel(None).unwrap();

    let exchange = create_exchange(
        &channel,
        queue_name,
        exchange_name,
        routing_key
    );

    while let Ok(packet) = capture.next() {
        let message = Publish::new(
            &*packet,
            routing_key
        );
        exchange.publish(message).unwrap();
    }

    connection.close()
}
