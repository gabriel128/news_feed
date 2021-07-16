use std::env;

use amiquip::{Connection, ConsumerMessage, ConsumerOptions};
use amiquip::QueueDeclareOptions;
use news_feeding::db::cluster::CassandraDb;
use tracing::{info, instrument};
use chrono::prelude::*;
use news_feeding::models::article::Article;
use news_feeding::{Result, db::Query};
use news_feeding::errors::Error;

#[tokio::main]
#[instrument]
async fn main() -> Result<()> {
    let _ = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .try_init();

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(Error::StringError("Wrong number of arguments".to_string()))
    }

    let country = &args[1];

    info!("Starting {} consumer", country);

    let mut rmq_conn = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;
    let channel = rmq_conn.open_channel(None)?;

    let exchange = channel
        .exchange_declare(
            amiquip::ExchangeType::Topic,
            "news_feed.country.tag.topic",
            amiquip::ExchangeDeclareOptions::default(),
        )?;


    let queue = channel
        .queue_declare(
            country.to_owned() + ".covid.queue",
            QueueDeclareOptions {
                durable: true,
                ..QueueDeclareOptions::default()
            },
        )?;

    queue.bind(&exchange, country.to_owned() + ".covid", amiquip::FieldTable::default())?;

    let consumer = queue.consume(ConsumerOptions::default())?;
    println!("Waiting for messages. Press Ctrl-C to exit.");

    let mut cass_conn = CassandraDb::new_cluster()?;
    let session = cass_conn.new_session().await?;

    for message in consumer.receiver().iter() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                if let Some(timestamp) = &delivery.properties.timestamp() {
                    println!("Current Timestamp is: {:?}", Utc.timestamp(*timestamp as i64, 0).to_rfc2822());
                }

                let article: Article = bincode::deserialize(&delivery.body[..]).unwrap();
                let query = article.insert_to_cassandra(country.to_string(), "covid".to_string())?;
                let res = session.execute(&query).await?;
                println!("Inerstion result {:?}", res);
                consumer.ack(delivery)?;
            }
            _ => {
                break;
            }
        }
    }

    Ok(())
}
