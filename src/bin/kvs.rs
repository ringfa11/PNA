use clap::{Args, Parser, Subcommand};

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

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Set(args) => {
            panic!("unimplemented")
        }
        Commands::Get(args) => {
            panic!("unimplemented")
        }
        Commands::Rm(args) => {
            panic!("unimplemented")
        }
    }
}
