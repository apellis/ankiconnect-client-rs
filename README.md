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

println!("host API version: {}", client.version()?);
```

### Get deck names

```
// create a client as above

for deck_name in client.deck_names()? {
    println!("found deck with name: {}", deck_name);
}
```

## License

This project is licensed under the [MIT License](https://opensource.org/licenses/MIT). See the `LICENSE` file.
