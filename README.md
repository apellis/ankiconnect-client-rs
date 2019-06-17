# ankiconnect-client

A Rust client for [AnkiConnect](https://foosoft.net/projects/anki-connect/). The currently supported API version is 6.

Requests and responses use `serde_json::Value` structs, so the `serde_json` crate is a requirement.

## Examples

### Create a client

```
extern crate ankiconnect_client;
use ankiconnect_client::AnkiConnectClient;

let client = AnkiConnectClient::new("localhost", 8765);
```

### Get API version

```
// create a client as above

let result = client.call("version", None)?;
if let Some(n) = result.as_i64() {
    println!("The AnkiConnect server is using API version {}");
} else {
    println!("Got unexpected response: {}", result);
}
```

### Get deck names

```
// create a client as above

let result = client.call("deckNames", None)?;
if let serde_json::Value::Array(v) = result {
    for elt in v {
        if let Some(s) = elt.as_str() {
            println!("Found deck: {}", s);
        } else {
            println!("Could not parse deck: {}", s);
        }
    }
} else {
    println!("Got unexpected response: {}", result);
}
```

### Sync with AnkiWeb service

```
// create a client as above

let result = client.call("sync", None)?;
println!("Synced!");  // if there is an error, we won't make it to this line
```

## License

This project is licensed under the [MIT License](https://opensource.org/licenses/MIT). See the `LICENSE` file.
