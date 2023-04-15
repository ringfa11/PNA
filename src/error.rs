use thiserror::Error;
/// KvsError kind
#[derive(Error, Debug)]
pub enum KvsError {
    /// i/o error
    #[error("An i/o error occured")]
    Io(std::io::Error),
    /// bson deserialize error
    #[error("A bson deserialize error occured")]
    BsonDe(bson::de::Error),
    /// bson serialize error
    #[error("A bson serialize error occured")]
    BsonSer(bson::ser::Error),
    /// key not found error
    #[error("Key not found")]
    KeyNotFound,
    /// wrong engine
    #[error("Wrong engine")]
    WrongEngine,
    /// from utf8 error
    #[error("From utf8 error")]
    FromUtf8Error(std::string::FromUtf8Error),
    /// error contains a string
    #[error("{0}")]
    StringErr(std::string::String),
    /// unexpected error
    #[error("Unexpeted error")]
    Unexpected,
}
impl Into<std::string::String> for KvsError {
    fn into(self) -> std::string::String {
       format!("{}",self)
    }
}

impl From<std::io::Error> for KvsError {
    fn from(value: std::io::Error) -> Self {
        KvsError::Io(value)
    }
}
impl From<bson::de::Error> for KvsError {
    fn from(value: bson::de::Error) -> Self {
        KvsError::BsonDe(value)
    }
}
impl From<bson::ser::Error> for KvsError {
    fn from(value: bson::ser::Error) -> Self {
        KvsError::BsonSer(value)
    }
}

impl From<std::string::FromUtf8Error> for KvsError {
    fn from(value: std::string::FromUtf8Error) -> Self {
        KvsError::FromUtf8Error(value)
    }
}

impl From<std::string::String> for KvsError {
    fn from(value: std::string::String) -> Self {
        KvsError::StringErr(value)
    }
}

/// Result for KvsError
pub type Result<T> = std::result::Result<T, KvsError>;
