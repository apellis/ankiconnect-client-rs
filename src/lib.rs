extern crate default;
extern crate serde;
extern crate serde_json;
extern crate reqwest;

use default::default;
use serde::{Serialize, Deserialize};

const DEFAULT_API_VERSION: i64 = 6;

#[derive(Debug)]
pub enum ApiVersion {
    V(i64),
}

impl ApiVersion {
    fn to_i64(&self) -> i64 {
        match self {
            ApiVersion::V(v) => *v,
        }
    }
}

impl Default for ApiVersion {
    fn default() -> Self { ApiVersion::V(DEFAULT_API_VERSION) }
}

#[derive(Debug, Serialize)]
struct AnkiConnectRequest<'a> {
    action: &'a str,

    version: i64,

    // Anki 2.1.x will return an error if it gets a Null params field when it
    // expects no params for the given action
    #[serde(skip_serializing_if = "serde_json::Value::is_null")]
    params: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct AnkiConnectResponse {
    result: serde_json::Value,

    error: Option<String>,
}

#[derive(Debug, Clone)]
struct AnkiConnectError {
    error_msg: String,
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

#[derive(Debug)]
pub struct AnkiConnectClient<'a> {
    hostname: &'a str,
    port: i64,
    version: ApiVersion,
}

impl<'a> AnkiConnectClient<'a> {
    pub fn new<'b>(hostname: &'b str, port: i64) -> AnkiConnectClient {
        AnkiConnectClient { hostname, port, version: default() }
    }

    pub fn call<'b>(&self, action: &'b str, params: Option<serde_json::Value>)
        -> Result<serde_json::Value, Box<std::error::Error>>
    {
        let request = AnkiConnectRequest {
            action: action,
            version: self.version.to_i64(),
            params: match params {
                Some(json_val) => json_val,
                None => serde_json::Value::Null,
            }
        };

        let client = reqwest::Client::new();
        let response: AnkiConnectResponse = client.post(
                &format!("http://{}:{}", self.hostname, self.port))
            .json(&request)
            .send()?
            .json()?;

        match response.error {
            Some(error_msg) => {
                let err = AnkiConnectError { error_msg };
                Err(err.into())
            },
            None => Ok(response.result)
        }
    }
}
