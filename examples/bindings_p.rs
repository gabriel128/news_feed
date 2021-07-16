use amiquip::{AmqpProperties, Connection, ConsumerMessage, ConsumerOptions, ExchangeDeclareOptions, ExchangeType, Publish, QueueDeclareOptions, Result};

fn main() -> Result<()> {
    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;
    let channel = connection.open_channel(None)?;

    let exchange = channel.exchange_declare(
        ExchangeType::Topic,
        "topic_logs1",
        ExchangeDeclareOptions::default(),
    )?;

    let response_queue = channel.queue_declare(
        "",
        QueueDeclareOptions {
            exclusive: true,
            ..QueueDeclareOptions::default()
        },
    )?;

    let correlation_id = format!("{}", "random_uuid");

    exchange.publish(Publish::with_properties(
        b"this is info!",
        "au.info",
        AmqpProperties::default()
            .with_reply_to(response_queue.name().to_string())
            .with_correlation_id(correlation_id.clone())
            .with_delivery_mode(2),
    ))?;

    exchange.publish(Publish::with_properties(
        b"this is warning!",
        "ar.warning",
        AmqpProperties::default()
            .with_reply_to(response_queue.name().to_string())
            .with_correlation_id(correlation_id.clone())
            .with_delivery_mode(2),
    ))?;

    exchange.publish(Publish::with_properties(
        b"this is error!",
        "au",
        AmqpProperties::default()
            .with_reply_to(response_queue.name().to_string())
            .with_correlation_id(correlation_id.clone())
            .with_delivery_mode(2),
    ))?;

    let consumer = response_queue.consume(ConsumerOptions {no_ack: true, ..ConsumerOptions::default()})?;

    for message in consumer.receiver().iter() {
        match message {
                ConsumerMessage::Delivery(delivery) => {
                    if delivery.properties.correlation_id().as_ref() == Some(&correlation_id) {
                        let resp = String::from_utf8_lossy(&delivery.body);
                        println!("Response is: {}", resp);
                    } else {
                        println!("Response with different correlation id {}", correlation_id);
                    }
                }
                other => {
                    println!("Consumer ended: {:?}", other);
                    break;
                }
        }
    }

    connection.close()
}
