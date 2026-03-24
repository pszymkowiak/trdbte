pub mod types;
pub mod storage;
pub mod sql;
pub mod executor;

use std::path::Path;

use types::Value;

/// The main database handle.
pub struct Database {
    _path: std::path::PathBuf,
}

/// A result set returned by a query.
pub struct QueryResult {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<Value>>,
}

impl Database {
    /// Open or create a database at the given path.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, crate::Error> {
        Ok(Self {
            _path: path.as_ref().to_path_buf(),
        })
    }

    /// Execute a statement that does not return rows (CREATE, INSERT, UPDATE, DELETE).
    pub fn execute(&self, _sql: &str) -> Result<usize, crate::Error> {
        Err(crate::Error::NotImplemented)
    }

    /// Execute a query that returns rows (SELECT).
    pub fn query(&self, _sql: &str) -> Result<QueryResult, crate::Error> {
        Err(crate::Error::NotImplemented)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("not implemented")]
    NotImplemented,
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
