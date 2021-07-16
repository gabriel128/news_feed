use cassandra_cpp::Statement;

pub mod cluster;

pub trait Query {
   fn insert_to_cassandra(&self, country: String, tag: String) -> crate::Result<Statement>;
}
