use clap::Parser;
use std::net::SocketAddr;
use std::string::String;
use log::{info,error};
use kvs::Server;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[arg(long, default_value = "127.0.0.1:4000")]
    addr: SocketAddr,
    #[arg(long, default_value = "kvs")]
    engine: String,
}

fn main()  {
    pretty_env_logger::init();
    let cli = Cli::parse();
    info!("Version:{}", env!("CARGO_PKG_VERSION"));
    info!("Configuration: [addr:{}, engine:{}]", cli.addr,cli.engine);
    match Server::init(cli.addr, cli.engine){
        Ok(mut server)=>{
    if let Err(error) =  server.run(){
        error!("ooops, some error occured while handling stream:{}",error);
    }
        }
        Err(error)=>{
            panic!("init server failed:{}",error)
        }
    }
}
