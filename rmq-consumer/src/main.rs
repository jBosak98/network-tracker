use amiquip::{
    Connection,
    ConsumerMessage,
    ExchangeType,
    ExchangeDeclareOptions,
    QueueDeclareOptions,
    FieldTable,
    ConsumerOptions
};

fn main(){
    let rabbit_url = "amqp://rmq:rmq@127.0.0.1:5672/%2f";
    let routing_key ="network_traffic";
    let queue_name = "hello_queue";
    let exchange_name = "hello_exchange";

    let mut connection = Connection::insecure_open(rabbit_url).unwrap();
    let channel = connection.open_channel(Some(1)).unwrap();
    let exchange = channel.exchange_declare(
        ExchangeType::Direct,
        exchange_name,
        ExchangeDeclareOptions::default()
    ).unwrap();
    let queue = channel.queue_declare(
        queue_name,
        QueueDeclareOptions::default()
    ).unwrap();
    channel.queue_bind(
        queue.name(),
        exchange.name(),
        routing_key,
        FieldTable::new()
    ).unwrap();
    let consumer = queue.consume(ConsumerOptions::default()).unwrap();
    for message in consumer.receiver() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                println!("{:?}", delivery.body);

            },
            _ => {}
        }
    }

}
