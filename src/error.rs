#[derive(Debug, Clone)]
pub struct AnkiConnectError {
    pub error_msg: String,
}

impl std::error::Error for AnkiConnectError {
    fn description(&self) -> &str {
        "error returned by AnkiConnect"
    }

    fn cause(&self) -> Option<&std::error::Error> {
        None
    }
}

impl std::fmt::Display for AnkiConnectError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "AnkiConnect error message: {}", self.error_msg)
    }
}

