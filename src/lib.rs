//! simple crate for kvs
mod client;
mod error;
mod protocol;
mod server;
mod store;
pub use client::Client;
pub use error::{KvsError, Result};
pub use protocol::{GetResponse, RemoveResponse, Request, SetResponse};
pub use server::Server;
pub use store::{KvStore, KvsEngine};
