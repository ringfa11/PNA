use crate::{KvStore, Request, RemoveResponse,GetResponse,SetResponse,Result,KvsError};
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::fs::File;
use std::io::prelude::*;
use log::{info,debug,error};
struct Config {
    addr: SocketAddr,
    engine: String,
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{addr: {},engine: {}}}", self.addr, self.engine)
    }
}
pub struct Server {
    config: Config,
    store: KvStore,
}

impl Server {
    pub fn init(addr:SocketAddr,engine: String) -> Result<Self> {
        let  store = KvStore::open(std::env::current_dir()?)?;
        Self::check_engine(&engine)?;
        Ok(Server{config:Config { addr,engine },store})
    }

    pub fn run(&mut self) -> Result<()> {
        let listener = TcpListener::bind(self.config.addr)?;
        // accept connections and process them serially
        for stream in listener.incoming() {
            self.handle_client(stream?)?;
        }
        Ok(())
    }

    fn handle_client(&mut self, stream: TcpStream)->Result<()> {
        info!("new connection established.");
        let request =bson::from_reader::<&TcpStream, Request>(&stream)?;
        self.do_request(stream,request)?;
        Ok(())
    }

    fn check_engine(engine: &String) -> Result<()> {
        let mut file: File = File::options()
            .read(true)
            .write(true)
            .create(true)
            .open("data.type")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        if contents.len() != 0 && &contents != engine {
            error!("wrong engine!");
            return Err(KvsError::WrongEngine);
        }
        if contents.len() == 0 {
            file.write_all(engine.clone().as_bytes())?;
        }
        Ok(())
    }

    fn do_request(&mut self, mut stream: TcpStream,request: Request) ->Result<()> {
        match request {
            Request::Set { key, value } => {
                debug!("set command");
                match self.store.set(key, value){
                    Ok(_)=>{
                        stream.write(bson::to_vec(&SetResponse::Ok(()))?.as_slice())?;
                    }
                    Err(error)=>{
                        error!("set failed: {}",error);
                        stream.write(bson::to_vec(&SetResponse::Err(error.into()))?.as_slice())?;
                    }
                }
            }
            Request::Get { key } => {
                debug!("get command");
                match self.store.get(key){
                    Ok(value)=>{
                        stream.write(bson::to_vec(&GetResponse::Ok(value))?.as_slice())?;
                    }
                    Err(error)=>{
                        error!("write failed: {}",error);
                        stream.write(bson::to_vec(&SetResponse::Err(error.into()))?.as_slice())?;
                    }
                }
            }
            Request::Remove { key } => {
                debug!("remove command");
                match self.store.remove(key){
                    Ok(_)=>{
                        stream.write(bson::to_vec(&RemoveResponse::Ok(()))?.as_slice())?;
                    }
                    Err(error)=>{
                        error!("remove failed: {}",error);
                        stream.write(bson::to_vec(&RemoveResponse::Err(error.into()))?.as_slice())?;
                    }
                }
            }
        };
        Ok(())
    }
}
