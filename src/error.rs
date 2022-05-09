#[derive(Debug)]
pub enum Error {
    ParseData(String),
    ParseExpression(String),
    FilterError,
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        return Error::ParseData(format!("Unable to parse data because: {}", e));
    }
}

impl From<json::Error> for Error {
    fn from(e: json::Error) -> Self {
        return Error::ParseData(format!("Unable to parse data because: {}", e));
    }
}


