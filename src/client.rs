extern crate default;
extern crate serde;
extern crate serde_json;

use default::default;
use serde::{Serialize, Deserialize};
use serde_json::json;

use super::error::AnkiConnectError;
use super::api_version::ApiVersion;

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

    // ======================================================================
    // ========== Miscellaneous =============================================
    // ======================================================================

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

    // TODO
    // -- multi

    // ======================================================================
    // ========== Decks =====================================================
    // ======================================================================

    pub fn deck_names(&self) -> Result<Vec<String>, Box<std::error::Error>> {
        match self.call("deckNames", None) {
            Ok(json_val) => {
                if let Some(ref v) = json_val.as_array() {
                    Ok(v
                       .iter()
                       .filter_map(|s| s.as_str())
                       .map(|s| s.to_string())
                       .collect()
                    )
                } else {
                    let err = AnkiConnectError {
                        error_msg: "Could not parse vector of strings from json".to_string()
                    };
                    Err(err.into())
                }
            },
            Err(e) => Err(e)
        }
    }

    // TODO
    //   -- deckNamesAndIds
    //   -- getDecks
    //   -- createDeck
    //   -- changeDeck
    //   -- deleteDecks
    //   -- getDeckConfig
    //   -- saveDeckConfig
    //   -- setDeckConfigId
    //   -- cloneDeckConfigId
    //   -- removeDeckConfigId

    // ======================================================================
    // ========== Models ====================================================
    // ======================================================================

    pub fn model_names(&self) -> Result<Vec<String>, Box<std::error::Error>> {
        match self.call("modelNames", None) {
            Ok(json_val) => {
                if let Some(ref v) = json_val.as_array() {
                    Ok(v
                       .iter()
                       .filter_map(|s| s.as_str())
                       .map(|s| s.to_string())
                       .collect()
                    )
                } else {
                    let err = AnkiConnectError {
                        error_msg: "Could not parse vector of strings from json".to_string()
                    };
                    Err(err.into())
                }
            },
            Err(e) => Err(e)
        }
    }

    // TODO
    //   -- modelNamesAndIds
    //   -- modelFieldNames
    //   -- modelFieldsOnTemplates
    //   -- createModel

    // ======================================================================
    // ========== Notes =====================================================
    // ======================================================================

    // TODO
    //   -- addNote
    //   -- addNotes
    //   -- canAddNotes
    //   -- updateNoteFields
    //   -- addTags
    //   -- removeTags
    //   -- getTags
    //   -- findNotes
    //   -- notesInfo
    //   -- deleteNotes

    // ======================================================================
    // ========== Cards =====================================================
    // ======================================================================

    // Warning: Anki 2.1.x will give an error ("NoneType is not iterable") if you provide
    // invalide card ids
    pub fn suspend(&self, cards: &Vec<i64>) -> Result<bool, Box<std::error::Error>> {
        let params = json!({ "cards": &cards });

        match self.call("suspend", Some(params)) {
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

    // Warning: Anki 2.1.x will give an error ("NoneType is not iterable") if you provide
    // invalide card ids
    pub fn unsuspend(&self, cards: &Vec<i64>) -> Result<bool, Box<std::error::Error>> {
        let params = json!({ "cards": &cards });

        match self.call("unsuspend", Some(params)) {
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

    // TODO
    //   -- areSuspended
    //   -- areDue
    //   -- getIntervals
    //   -- findCards
    //   -- cardsToNotes
    //   -- cardsInfo

    // ======================================================================
    // ========== Media =====================================================
    // ======================================================================

    pub fn delete_media_file(&self, filename: &str) -> Result<(), Box<std::error::Error>> {
        let params = json!({ "filename": &filename });

        match self.call("deleteMediaFile", Some(params)) {
            Ok(json!(null)) => Ok(()),
            Ok(_) => {
                let err = AnkiConnectError {
                    error_msg: "Received non-null response to deleteMediaFile request".to_string()
                };
                Err(err.into())
            },
            Err(e) => Err(e)
        }
    }

    // TODO
    //   -- storeMediaFile, "from file" and "from b64"
    //   -- retrieveMediaFile, "from file" and "from b64"

    // ======================================================================
    // ========== Graphical =================================================
    // ======================================================================

    // TODO
    //   -- guiBrowse
    //   -- guiAddCards
    //   -- guiCurrentCard
    //   -- guiStartCardTimer
    //   -- guiShowQuestion
    //   -- guiShowAnswer
    //   -- guiAnswerCard
    //   -- guiDeckOverview
    //   -- guiDeckBrowser
    //   -- guiDeckReview
    //   -- guiExitAnki
}
