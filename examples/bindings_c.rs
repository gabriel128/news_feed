use amiquip::{AmqpProperties, Connection, ConsumerMessage, ConsumerOptions, Exchange, ExchangeDeclareOptions, ExchangeType, FieldTable, Publish, QueueDeclareOptions, Result};

fn main() -> Result<()> {
    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;
    let channel = connection.open_channel(None)?;
    let exchange = channel.exchange_declare(ExchangeType::Topic, "topic_logs1", ExchangeDeclareOptions::default())?;

    let queue = channel.queue_declare("", QueueDeclareOptions { exclusive: true, durable: false, ..QueueDeclareOptions::default() })?;
    println!("created exclusive queue {}", queue.name());

    queue.bind(&exchange, "au.#", FieldTable::default())?;
    // queue.bind(&exchange, "warning", FieldTable::default())?;

    let consumer = queue.consume(ConsumerOptions::default())?;
    println!("Waiting for messages. Press Ctrl-C to exit.");

    for message in consumer.receiver().iter() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let res = String::from_utf8_lossy(&delivery.body);
                println!("Received: {:?}", res);

                let (reply_to, corr_id) = match (
                    delivery.properties.reply_to(),
                    delivery.properties.correlation_id(),
                ) {
                    (Some(r), Some(c)) => (r.clone(), c.clone()),
                    _ => {
                        println!("received delivery without reply_to or correlation_id");
                        consumer.ack(delivery)?;
                        continue;
                    }
                };

                let resp_exchange = Exchange::direct(&channel);

                resp_exchange.publish(Publish::with_properties(
                    b"got you homie",
                    reply_to,
                    AmqpProperties::default().with_correlation_id(corr_id),
                ))?;

                consumer.ack(delivery)?;
            }
            _ => {
                break;
            }
        }
    }

    connection.close()
}
