use std::env::VarError;

use reqwest::header::InvalidHeaderValue;
use tokio::task::JoinError;

#[derive(Debug)]
pub enum Error {
    RequestError(reqwest::Error),
    RequestHeaderError(InvalidHeaderValue),
    RabbitMQError(amiquip::Error),
    ThreadJoinError(JoinError),
    StringError(String),
    EnvError(VarError),
    CassandraError(cassandra_cpp::Error)
}

impl From<VarError> for Error {
    fn from(e: VarError) -> Self {
        Error::EnvError(e)
    }
}

impl From<InvalidHeaderValue> for Error {
    fn from(e: InvalidHeaderValue) -> Self {
        Error::RequestHeaderError(e)
    }
}

impl From<cassandra_cpp::Error> for Error {
    fn from(e: cassandra_cpp::Error) -> Self {
        Error::CassandraError(e)
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::RequestError(e)
    }
}

impl From<amiquip::Error> for Error {
    fn from(e: amiquip::Error) -> Self {
        Error::RabbitMQError(e)
    }
}

impl From<JoinError> for Error {
    fn from(e: JoinError) -> Self {
        Error::ThreadJoinError(e)
    }
}
