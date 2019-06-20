extern crate default;
extern crate serde;
extern crate serde_json;

use default::default;
use std::fmt::Debug;
use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;
use serde_json::json;
use std::collections::HashMap;

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

    fn call<'b, ResultT>(&self, action: &'b str, params: Option<serde_json::Value>)
            -> Result<ResultT, Box<std::error::Error>>
            where ResultT: Debug + DeserializeOwned
    {
        let request = AnkiConnectRequest {
            action: action,
            version: self.version.to_i64(),
            params: match params {
                Some(json_val) => json_val,
                None => json!(null),
            }
        };

        #[derive(Debug, Deserialize)]
        struct Response<ResultT> {
            result: ResultT,
            error: Option<String>,
        }

        let client = reqwest::Client::new();
        let response: Response<ResultT> = client.post(
                &format!("http://{}:{}", self.hostname, self.port))
            .json(&request)
            .send()?
            .json()?;

        if let Some(error_msg) = response.error {
            Err(AnkiConnectError { error_msg }.into())
        } else {
            Ok(response.result)
        }
    }

    // ======================================================================
    // ========== Miscellaneous =============================================
    // ======================================================================

    pub fn version(&self) -> Result<i64, Box<std::error::Error>> {
        Ok(self.call::<i64>("version", None)?)
    }

    pub fn upgrade(&self) -> Result<bool, Box<std::error::Error>> {
        Ok(self.call::<bool>("upgrade", None)?)
    }

    pub fn sync(&self) -> Result<(), Box<std::error::Error>> {
        Ok(self.call::<()>("sync", None)?)
    }

    pub fn load_profile(&self, username: &str) -> Result<bool, Box<std::error::Error>> {
        let params = json!({ "name": username });
        Ok(self.call::<bool>("loadProfile", Some(params))?)
    }

    // TODO
    // -- multi

    // ======================================================================
    // ========== Decks =====================================================
    // ======================================================================

    pub fn deck_names(&self) -> Result<Vec<String>, Box<std::error::Error>> {
        Ok(self.call::<Vec<String>>("deckNames", None)?)
    }

    pub fn deck_names_and_ids(&self) -> Result<HashMap<String, i64>, Box<std::error::Error>> {
        Ok(self.call::<HashMap<String, i64>>("deckNamesAndIds", None)?)
    }

    // TODO
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
        Ok(self.call::<Vec<String>>("modelNames", None)?)
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
    // Warning 2: Return type is Option<bool> rather than bool for symmetry with unsuspend;
    // see warning 2 on that method.
    pub fn suspend(&self, cards: &Vec<i64>) -> Result<Option<bool>, Box<std::error::Error>> {
        let params = json!({ "cards": &cards });
        Ok(self.call::<Option<bool>>("suspend", Some(params))?)
    }

    // Warning: Anki 2.1.x will give an error ("NoneType is not iterable") if you provide
    // invalide card ids
    // Warning 2: Return type is Option<bool> rather than bool because in some cases, Anki 2.1.x
    // will return null for both result and error -- according to current (2019-06-20) API docs,
    // this appears to be out of compliance with the API spec. But if were to just let this error
    // out when Anki 2.1.x gives the null-null response, this would happen too often for this
    // method to be useful.
    pub fn unsuspend(&self, cards: &Vec<i64>) -> Result<Option<bool>, Box<std::error::Error>> {
        let params = json!({ "cards": &cards });
        Ok(self.call::<Option<bool>>("unsuspend", Some(params))?)
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
        Ok(self.call::<()>("deleteMediaFile", Some(params))?)
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
