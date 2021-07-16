use amiquip::{Connection, Exchange, QueueDeclareOptions, Publish, Result, AmqpProperties};

fn main() -> Result<()> {
    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;
    let channel = connection.open_channel(None)?;
    channel.queue_declare("task_queue", QueueDeclareOptions { durable: true, ..QueueDeclareOptions::default() })?;

    let exchange = Exchange::direct(&channel);
    exchange.publish(Publish::with_properties(b"hey!", "task_queue", AmqpProperties::default().with_delivery_mode(2)))?;

    connection.close()
}
