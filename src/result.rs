use std::io;

type StdResult<T, E> = std::result::Result<T, E>;
pub type Result<T> = StdResult<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("i/o error: {0}")]
    Io(#[from] io::Error),
    #[error("connection error: {0}")]
    Connection(#[from] ConnectionError),
    #[error("protocol error: {0}")]
    Protocol(#[from] ProtocolError),
}

#[derive(thiserror::Error, Debug, PartialEq, Eq, Clone)]
pub enum ConnectionError {
    #[error("connection reset by peer")]
    PeerReset,
}

#[derive(thiserror::Error, Debug, PartialEq, Eq, Clone)]
pub enum ProtocolError {
    #[error("incomplete")]
    Incomplete,
}
