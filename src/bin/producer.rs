use amiquip::Channel;
use amiquip::{AmqpProperties, Connection, Publish};
use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use chrono::prelude::*;
use news_feeding::models::article::Article;
use news_feeding::Result;

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    status: String,
    #[serde(rename = "totalResults")]
    total_results: u32,
    articles: Vec<Article>,
}

#[tokio::main]
#[instrument]
async fn main() -> Result<()> {
    let _ = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .try_init();

    info!("Starting producer");

    let mut rmq_conn: Connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;

    let mut headers = HeaderMap::new();
    let api_key = &std::env::var("API_KEY")?;
    headers.insert(
        "X-Api-Key",
        HeaderValue::from_str(api_key)?
    );

    let client = Client::builder().default_headers(headers).build()?;

    let countries = vec!["au", "ar"];
    let tags = vec!["covid"];
    let mut handles = vec![];

    for country in countries {
        for tag in &tags {
            let new_client = client.clone();
            let channel = rmq_conn.open_channel(None)?;

            let handle = tokio::spawn(send_request(new_client, country, tag, channel));
            handles.push(handle);
        }
    }

    for handle in handles {
        handle.await??;
    }

    rmq_conn.close()?;

    Ok(())
}

async fn send_request(client: Client, country: &str, tag: &str, channel: Channel) -> Result<()> {
    let res: Response = client
        .get(
            "https://newsapi.org/v2/top-headlines?q=".to_string()
                + tag
                + &"&country=".to_string()
                + country,
        )
        .send()
        .await?
        .json()
        .await?;

    let exchange = channel
        .exchange_declare(
            amiquip::ExchangeType::Topic,
            "news_feed.country.tag.topic",
            amiquip::ExchangeDeclareOptions::default(),
        )?;

    info!("Sending messages");

    for article in res.articles {
        let now: DateTime<Utc> =  Utc::now();

        exchange
            .publish(Publish::with_properties(
                &bincode::serialize(&article).unwrap(),
                country.to_owned() + "." + tag,
                AmqpProperties::default()
                    .with_timestamp(now.timestamp() as u64)
                    .with_delivery_mode(2),
            ))?;
    }
    Ok(())
}
