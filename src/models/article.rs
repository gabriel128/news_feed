use cassandra_cpp::{Statement, stmt};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::{Result, db::Query};

#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    source: HashMap<String, Option<String>>,
    title: String,
    url: Option<String>,
    description: Option<String>,
    #[serde(rename = "publishedAt")]
    published_at: Option<String>,
    content: Option<String>,
}

impl Query for Article {
    fn insert_to_cassandra(&self, country: String, tag: String) -> Result<Statement> {
        let mut insert_data = stmt!(&insert_query());
        insert_data.bind_string_by_name("country", &country)?;
        insert_data.bind_string_by_name("tag", &tag)?;

        let published_at = self
            .published_at
            .clone()
            .map(|time| time.parse::<DateTime<Utc>>().unwrap())
            .unwrap_or_else(|| Utc::now());


        println!("Timestamp is {:?}", published_at);

        insert_data.bind_string_by_name("published_at", &published_at.format("%d-%m-%Y").to_string())?;
        insert_data.bind_int32_by_name("published_at_ord", published_at.timestamp() as i32)?;
        insert_data.bind_string_by_name("title", &self.title)?;
        insert_data.bind_string_by_name("description", self.description.as_ref().unwrap_or(&"".to_string()))?;
        insert_data.bind_string_by_name("content", self.content.as_ref().unwrap_or(&"".to_string()))?;

        Ok(insert_data)
    }

}

fn insert_query() -> String {
    "INSERT INTO news_feeding.articles_by_country_tag_date \
     (country, tag, published_at, published_at_ord, title, description, content) \
     VALUES (?, ?, ?, ?, ?, ?, ?)".to_string()
}
