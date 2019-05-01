#[derive(Debug)]
pub enum SpikeError {
    DatabaseConnectionError(r2d2::Error),
    DatabaseQueryError(diesel::result::Error),
    ObjectNotFound,
}

impl std::fmt::Display for SpikeError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        use SpikeError::*;
        match *self {
            DatabaseConnectionError(ref e) => e.fmt(fmt),
            DatabaseQueryError(ref e) => e.fmt(fmt),
            ObjectNotFound => write!(fmt, "Could not find requested object."),
        }
    }
}

impl std::error::Error for SpikeError {
    fn description(&self) -> &str {
        use SpikeError::*;
        match *self {
            DatabaseConnectionError(ref e) => e.description(),
            DatabaseQueryError(ref e) => e.description(),
            ObjectNotFound => "Could not find requested object.",
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        use SpikeError::*;
        match *self {
            DatabaseConnectionError(ref e) => Some(e),
            DatabaseQueryError(ref e) => Some(e),
            _ => None,
        }
    }
}

impl std::convert::From<r2d2::Error> for SpikeError {
    fn from(error: r2d2::Error) -> Self {
        SpikeError::DatabaseConnectionError(error)
    }
}

impl std::convert::From<diesel::result::Error> for SpikeError {
    fn from(error: diesel::result::Error) -> Self {
        SpikeError::DatabaseQueryError(error)
    }
}
