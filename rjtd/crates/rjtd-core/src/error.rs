use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    InvalidData(String),
    NotFound(String),
    Unsupported(&'static str),
    Io(String),
    ResourceLimit {
        resource: &'static str,
        limit: usize,
        actual: usize,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidData(message) => write!(f, "invalid data: {message}"),
            Self::NotFound(message) => write!(f, "not found: {message}"),
            Self::Unsupported(feature) => write!(f, "unsupported feature: {feature}"),
            Self::Io(message) => write!(f, "i/o error: {message}"),
            Self::ResourceLimit {
                resource,
                limit,
                actual,
            } => write!(
                f,
                "resource limit exceeded: {resource} is {actual}, limit is {limit}"
            ),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error.to_string())
    }
}
