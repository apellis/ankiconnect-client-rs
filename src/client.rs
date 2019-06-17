extern crate default;
extern crate serde;
extern crate serde_json;

use default::default;
use serde::{Serialize, Deserialize};
use serde_json::json;

use super::error::AnkiConnectError;
use super::api_version::ApiVersion;

// TODO:
// -- API call: multi
// -- API calls: Decks section (11)
// -- API calls: Models section (5)
// -- API calls: Notes section (10)
// -- API calls: Cards section (8)
// -- API calls: Media section (3)
// -- API calls: Graphical section (11)

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

    fn call<'b>(&self, action: &'b str, params: Option<serde_json::Value>)
        -> Result<serde_json::Value, Box<std::error::Error>>
    {
        let request = AnkiConnectRequest {
            action: action,
            version: self.version.to_i64(),
            params: match params {
                Some(json_val) => json_val,
                None => json!(null),
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

    pub fn version(&self) -> Result<i64, Box<std::error::Error>> {
        match self.call("version", None) {
            Ok(json_val) => {
                if let Some(n) = json_val.as_i64() {
                    Ok(n)
                } else {
                    let err = AnkiConnectError {
                        error_msg: "Could not parse i64 from json".to_string()
                    };
                    Err(err.into())
                }
            },
            Err(e) => Err(e)
        }
    }

    pub fn upgrade(&self) -> Result<bool, Box<std::error::Error>> {
        match self.call("upgrade", None) {
            Ok(json_val) => {
                if let Some(b) = json_val.as_bool() {
                    Ok(b)
                } else {
                    let err = AnkiConnectError {
                        error_msg: "Could not parse bool from json".to_string()
                    };
                    Err(err.into())
                }
            },
            Err(e) => Err(e)
        }
    }

    pub fn sync(&self) -> Result<(), Box<std::error::Error>> {
        match self.call("sync", None) {
            Ok(json!(null)) => Ok(()),
            Ok(_) => {
                let err = AnkiConnectError {
                    error_msg: "Received non-null response to sync request".to_string()
                };
                Err(err.into())
            },
            Err(e) => Err(e)
        }
    }

    pub fn load_profile(&self, username: &str) -> Result<bool, Box<std::error::Error>> {
        let params = json!({ "name": username });

        match self.call("loadProfile", Some(params)) {
            Ok(json_val) => {
                if let Some(b) = json_val.as_bool() {
                    Ok(b)
                } else {
                    let err = AnkiConnectError {
                        error_msg: "Could not parse bool from json".to_string()
                    };
                    Err(err.into())
                }
            },
            Err(e) => Err(e)
        }
    }
}
