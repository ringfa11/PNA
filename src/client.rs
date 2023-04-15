use crate::{GetResponse, RemoveResponse, Request, Result, SetResponse};
use std::io::prelude::*;
use std::net::{SocketAddr, TcpStream};

pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn connect(addr: &SocketAddr) -> Result<Self> {
        Ok(Client {
            stream: TcpStream::connect(addr)?,
        })
    }
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.stream
            .write(bson::to_vec(&Request::Set { key, value })?.as_slice())?;
        match bson::from_reader::<&TcpStream, SetResponse>(&self.stream)? {
            SetResponse::Ok(_) => Ok(()),
            SetResponse::Err(error) => Err(error.into()),
        }
    }
    pub fn get(&mut self, key: String) -> Result<()> {
        self.stream
            .write(bson::to_vec(&Request::Get { key })?.as_slice())?;
        match bson::from_reader::<&TcpStream, GetResponse>(&self.stream)? {
            GetResponse::Ok(value) => {
                if value == None {
                    println!("Key not found");
                } else {
                    println!("{}", value.unwrap());
                }
                Ok(())
            }
            GetResponse::Err(error) => Err(error.into()),
        }
    }
    pub fn remove(&mut self, key: String) -> Result<()> {
        self.stream
            .write(bson::to_vec(&Request::Remove { key })?.as_slice())?;
        match bson::from_reader::<&TcpStream, RemoveResponse>(&self.stream)? {
            RemoveResponse::Ok(_) => Ok(()),
            RemoveResponse::Err(error) => Err(error.into()),
        }
    }
}
