use clap::{Args, Parser, Subcommand};
use failure::{format_err, Error};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// set a key/value pair
    Set(SetArgs),
    /// get a value by key
    Get(GetArgs),
    /// remove a key/value pair
    Rm(RemoveArgs),
}

#[derive(Args)]
struct SetArgs {
    key: String,
    value: String,
}

#[derive(Args)]
struct GetArgs {
    key: String,
}
#[derive(Args)]
struct RemoveArgs {
    key: String,
}

fn main() -> kvs::Result<()> {
    let cli = Cli::parse();
    let mut store = kvs::KvStore::open(std::env::current_dir()?)?;
    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    let result = match &cli.command {
        Commands::Set(args) => store.set(args.key.clone(), args.value.clone()),
        Commands::Get(args) => {
            let option = store.get(args.key.clone())?;
            if let Some(val) = option {
                println!("{}", val);
            } else {
                println!("Key not found");
            }
            Ok(())
        }
        Commands::Rm(args) => {
            if let Err(e) = store.remove(args.key.clone()) {
                println!("{}", e);
                return Err(e);
            }
            Ok(())
        }
    };
    result
}
