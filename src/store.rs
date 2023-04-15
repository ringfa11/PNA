use crate::error::{KvsError, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::path::PathBuf;
/// KvStore store k/v pairs in a BTreeMap
pub struct KvStore {
    file: File,
    index: BTreeMap<String, u64>,
    workdir: PathBuf,
    limit: u64,
}

/// KvsEngine
pub struct KvsEngine;

/// Log
#[derive(Serialize, Deserialize)]
pub enum Log {
    /// Set
    Set {
        /// key
        key: String,
        /// value
        value: String,
    },
    ///Remove
    Remove {
        ///key
        key: String,
    },
}

impl KvStore {
    /// open database
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let workdir: PathBuf = path.into();
        let mut fliepath = workdir.clone();
        fliepath.push("data.db");
        let mut file = File::options()
            .read(true)
            .write(true)
            .create(true)
            .open(&fliepath)?;
        let mut index: BTreeMap<String, u64> = BTreeMap::new();
        file.rewind()?;
        loop {
            let offset = file.stream_position()?;
            let result: bson::de::Result<Log> = bson::from_reader(&file);
            if let Ok(cmd) = result {
                match cmd {
                    Log::Set { key, .. } => {
                        index.insert(key, offset);
                    }
                    Log::Remove { key } => {
                        index.remove(&key);
                    }
                }
            } else {
                break;
            }
        }
        let limit = 1024;
        return Ok(KvStore {
            file,
            index,
            workdir,
            limit,
        });
    }
    /// set k/v
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let cmd = Log::Set {
            key: key.clone(),
            value,
        };
        let bytes = bson::to_vec(&cmd)?;
        let offset = self.file.seek(SeekFrom::End(0))?;
        self.file.write(&bytes)?;
        self.index.insert(key.clone(), offset);
        self.file.rewind()?;
        let filesize = self.file.seek(SeekFrom::End(0))?;
        if filesize > self.limit {
            self.compact()?;
            self.limit = filesize * 2;
        }
        Ok(())
    }
    /// get value by key
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        if !self.index.contains_key(&key) {
            std::fs::copy(self.workdir.join("data.db"), "mydump")?;
            let mut mem: BTreeMap<String, String> = BTreeMap::new();
            self.file.rewind()?;
            loop {
                let result: bson::de::Result<Log> = bson::from_reader(&self.file);
                if let Ok(cmd) = result {
                    match cmd {
                        Log::Set { key, value } => {
                            mem.insert(key, value);
                        }
                        Log::Remove { key } => {
                            mem.remove(&key);
                        }
                    }
                } else {
                    break;
                }
            }
            let mut mymem = File::options().write(true).create(true).open("mymem")?;
            for (key, val) in &mem {
                write!(mymem, "{},{}\n", key, val)?;
            }
            return Ok(None);
        }
        let offset = self.index.get(&key).unwrap().clone();
        self.file.seek(SeekFrom::Start(offset))?;
        if let Log::Set { value, .. } = bson::from_reader(&self.file)? {
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }
    /// remove k/v pair
    pub fn remove(&mut self, key: String) -> Result<()> {
        if !self.index.contains_key(&key) {
            return Err(KvsError::KeyNotFound);
        }
        let cmd = Log::Remove { key: key.clone() };
        let bytes = bson::to_vec(&cmd)?;
        let offset = self.file.seek(SeekFrom::End(0))?;
        self.file.write(&bytes)?;
        self.index.insert(key, offset);
        Ok(())
    }

    fn compact(&mut self) -> Result<()> {
        let mut mem: BTreeMap<String, String> = BTreeMap::new();
        self.file.rewind()?;
        loop {
            let result: bson::de::Result<Log> = bson::from_reader(&self.file);
            if let Ok(cmd) = result {
                match cmd {
                    Log::Set { key, value } => {
                        mem.insert(key, value);
                    }
                    Log::Remove { key } => {
                        mem.remove(&key);
                    }
                }
            } else {
                break;
            }
        }
        let mut compact = self.workdir.clone();
        compact.push(".compact.db");
        let mut file = File::options()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(&compact)?;
        let mut index: BTreeMap<String, u64> = BTreeMap::new();
        for item in mem {
            let (key, value) = item;
            let bytes = bson::to_vec(&Log::Set {
                key: key.clone(),
                value,
            })?;
            let offset = file.seek(SeekFrom::Current(0))?;
            file.write(&bytes)?;
            index.insert(key, offset);
        }
        self.file = file;
        self.index = index;
        let mut datafile = self.workdir.clone();
        datafile.push("data.db");
        std::fs::remove_file(&datafile)?;
        std::fs::rename(compact, datafile)?;
        Ok(())
    }
}
