use std::collections::HashMap;
use amiquip::{Connection, QueueDeclareOptions, ConsumerMessage, ConsumerOptions, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Article {
    source: HashMap<String, Option<String>>,
    title: String,
    url: Option<String>,
    description: Option<String>,
    content: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    status: String,
    #[serde(rename = "totalResults")]
    total_results: u32,
    articles: Vec<Article>,
}

fn main() -> Result<()> {
    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;
    let channel = connection.open_channel(None)?;
    let queue = channel.queue_declare("task_queue", QueueDeclareOptions { durable: true, ..QueueDeclareOptions::default() })?;
    channel.qos(0, 1, false)?;

    let consumer = queue.consume(ConsumerOptions::default())?;
    println!("Waiting for messages. Press Ctrl-C to exit.");

    for message in consumer.receiver().iter() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                // let res = String::from_utf8_lossy(&delivery.body);
                let res: Response = bincode::deserialize(&delivery.body[..]).unwrap();
                println!("Received: {:?}", res);
                consumer.ack(delivery)?;
            }
            _ => {
                break;
            }
        }
    }

    connection.close()
}
