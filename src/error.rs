use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
pub enum IronTunnelError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    
    #[error("Connection error: {0}")]
    Connection(String),
    
    #[error("Protocol error: {0}")]
    Protocol(String),
}

pub type Result<T> = std::result::Result<T, IronTunnelError>; 