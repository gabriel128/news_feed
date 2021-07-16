CREATE KEYSPACE news_feeding
WITH REPLICATION = {
   'class': 'SimpleStrategy',
   'replication_factor': 3
};

USE news_feeding;

CREATE TYPE links (
    url text,
    image_url text,
);

CREATE TABLE articles_by_country_tag_date (
       country text,
       tag text,
       published_at text,
       published_at_ord int,
       title text,
       description text,
       links links,
       content text,
       source frozen<map<text, text>>,
       PRIMARY KEY ((country, tag, published_at), published_at_ord, title)
) WITH CLUSTERING ORDER BY (published_at_ord DESC);
