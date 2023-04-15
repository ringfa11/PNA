use clap::{Parser, Subcommand};
use kvs::{Client, Result};
use std::net::SocketAddr;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    subcommand: SubCommand,
}

#[derive(Subcommand)]
enum SubCommand {
    /// set a key/value pair
    Set {
        /// A string key
        key: String,
        /// A string value
        value: String,
        /// Target address
        #[arg(long, default_value = "127.0.0.1:4000")]
        addr: SocketAddr,
    },
    /// get a value by key
    Get {
        /// A string key
        key: String,
        /// Target address
        #[arg(long, default_value = "127.0.0.1:4000")]
        addr: SocketAddr,
    },
    /// remove a key/value pair
    Rm {
        /// A string key
        key: String,
        /// Target address
        #[arg(long, default_value = "127.0.0.1:4000")]
        addr: SocketAddr,
    },
}

fn run_command(command: SubCommand) -> Result<()> {
    match command {
        SubCommand::Set { key, value, addr } => {
            let mut client = Client::connect(&addr)?;
            client.set(key, value)
        }
        SubCommand::Get { key, addr } => {
            let mut client = Client::connect(&addr)?;
            client.get(key)
        }
        SubCommand::Rm { key, addr } => {
            let mut client = Client::connect(&addr)?;
            client.remove(key)
        }
    }
}

fn main() {
    let cli = Cli::parse();
    match run_command(cli.subcommand) {
        Ok(_) => {}
        Err(error) => {
            panic!("{}", error);
        }
    }
}
