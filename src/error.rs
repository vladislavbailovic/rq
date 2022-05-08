#[derive(Debug)]
pub enum Error {
    ParseError(String),
    FilterError,
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        return Error::ParseError(format!("Unable to parse data because: {}", e));
    }
}

impl From<json::Error> for Error {
    fn from(e: json::Error) -> Self {
        return Error::ParseError(format!("Unable to parse data because: {}", e));
    }
}


