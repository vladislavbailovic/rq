#[derive(Debug)]
pub enum Error {
    Dataset(String),
    Lexer(String),
    Parser(String),
    Filter,
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        return Error::Dataset(format!("Unable to parse data because: {}", e));
    }
}

impl From<json::Error> for Error {
    fn from(e: json::Error) -> Self {
        return Error::Dataset(format!("Unable to parse data because: {}", e));
    }
}

impl From<yaml_rust::ScanError> for Error {
    fn from(e: yaml_rust::ScanError) -> Self {
        return Error::Dataset(format!("Unable to parse data because: {}", e));
    }
}
