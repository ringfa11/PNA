use failure::Fail;
/// KvsError kind
#[derive(Fail, Debug)]
pub enum KvsError {
    /// i/o error
    #[fail(display = "An i/o error occured")]
    Io(std::io::Error),
    /// bson deserialize error
    #[fail(display = "A bson deserialize error occured")]
    BsonDe(bson::de::Error),
    /// bson serialize error
    #[fail(display = "A bson serialize error occured")]
    BsonSer(bson::ser::Error),
    /// key not found error
    #[fail(display = "Key not found")]
    KeyNotFound,
    /// unexpected error
    #[fail(display = "Unexpeted error")]
    Unexpected,
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

/// Result for KvsError
pub type Result<T> = std::result::Result<T, KvsError>;
