#[derive(Debug)]
pub enum SpikeError {
    DatabaseConnectionError(r2d2::Error),
    DatabaseQueryError(diesel::result::Error),
    InvalidId,
}

impl std::fmt::Display for SpikeError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        use SpikeError::*;
        match *self {
            DatabaseConnectionError(ref e) => e.fmt(fmt),
            DatabaseQueryError(ref e) => e.fmt(fmt),
            InvalidId => write!(fmt, "The specified Id is invalid"),
        }
    }
}

impl std::error::Error for SpikeError {
    fn description(&self) -> &str {
        use SpikeError::*;
        match *self {
            DatabaseConnectionError(ref e) => e.description(),
            DatabaseQueryError(ref e) => e.description(),
            InvalidId => "Specified ID is invalid",
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
