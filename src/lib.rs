pub mod models;
pub mod db;
pub mod errors;

pub type Result<T, E = errors::Error> = std::result::Result<T, E>;
