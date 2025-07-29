use std::error::Error;
use std::sync::{RwLockReadGuard, RwLockWriteGuard, TryLockError};
use std::{fmt, io};

use libp2p::TransportError;

#[derive(Debug, Clone)]
pub enum ThothErrors {
    LockError(String),
    IoError(String),
    Tonic(String),
    P2PError(String),
    DbError(String),
    SendChError(String),
}

impl fmt::Display for ThothErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ThothErrors::LockError(msg) => write!(f, "Lock error: {}", msg),
            ThothErrors::IoError(msg) => write!(f, "Io error: {}", msg),
            ThothErrors::Tonic(msg) => write!(f, "gRPC Tonic error: {}", msg),
            ThothErrors::P2PError(msg) => write!(f, "P2P error: {}", msg),
            ThothErrors::DbError(msg) => write!(f, "Sqlite Database error: {}", msg),
            ThothErrors::SendChError(msg) => write!(f, "Sending Channel error: {}", msg),
        }
    }
}

impl Error for ThothErrors {}

impl<T> From<TryLockError<RwLockWriteGuard<'_, T>>> for ThothErrors {
    fn from(err: TryLockError<RwLockWriteGuard<'_, T>>) -> Self {
        match err {
            TryLockError::Poisoned(e) => ThothErrors::LockError(format!("Lock poisoned: {:?}", e)),
            TryLockError::WouldBlock => {
                ThothErrors::LockError("Failed to acquire write lock: Would block".to_string())
            }
        }
    }
}

impl<T> From<TryLockError<RwLockReadGuard<'_, T>>> for ThothErrors {
    fn from(err: TryLockError<RwLockReadGuard<'_, T>>) -> Self {
        match err {
            TryLockError::Poisoned(e) => ThothErrors::LockError(format!("Lock poisoned: {:?}", e)),
            TryLockError::WouldBlock => {
                ThothErrors::LockError("Failed to acquire read lock: Would block".to_string())
            }
        }
    }
}

impl From<io::Error> for ThothErrors {
    fn from(err: io::Error) -> Self {
        ThothErrors::IoError(err.to_string())
    }
}

impl From<tonic::transport::Error> for ThothErrors {
    fn from(err: tonic::transport::Error) -> Self {
        ThothErrors::IoError(err.to_string())
    }
}

impl From<libp2p::multiaddr::Error> for ThothErrors {
    fn from(err: libp2p::multiaddr::Error) -> Self {
        ThothErrors::P2PError(err.to_string())
    }
}

impl From<TransportError<std::io::Error>> for ThothErrors {
    fn from(err: TransportError<std::io::Error>) -> Self {
        ThothErrors::P2PError(err.to_string())
    }
}
impl From<sea_orm::DbErr> for ThothErrors {
    fn from(err: sea_orm::DbErr) -> Self {
        ThothErrors::DbError(err.to_string())
    }
}

impl<T> From<tokio::sync::mpsc::error::SendError<T>> for ThothErrors {
    fn from(err: tokio::sync::mpsc::error::SendError<T>) -> Self {
        ThothErrors::SendChError(err.to_string())
    }
}
