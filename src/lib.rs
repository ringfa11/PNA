//! simple crate for kvs
#![deny(missing_docs)]
mod error;
mod store;
pub use error::{KvsError, Result};
pub use store::KvStore;
