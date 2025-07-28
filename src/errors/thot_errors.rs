use std::error::Error;
use std::sync::{RwLockReadGuard, RwLockWriteGuard, TryLockError};
use std::{fmt, io};

#[derive(Debug, Clone)]
pub enum ThothErrors {
    LockError(String),
    Io(String),
}

impl fmt::Display for ThothErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ThothErrors::LockError(msg) => write!(f, "Lock error: {}", msg),
            ThothErrors::Io(msg) => write!(f, "Io error: {}", msg),
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
        ThothErrors::Io(err.to_string())
    }
}
