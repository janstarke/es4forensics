[![Crates.io](https://img.shields.io/crates/v/es4forensics)](https://crates.io/crates/es4forensics)
[![docs.rs](https://img.shields.io/docsrs/es4forensics)](https://docs.rs/crate/es4forensics)
![Crates.io](https://img.shields.io/crates/l/es4forensics)
![Crates.io (latest)](https://img.shields.io/crates/dv/es4forensics)

# es4forensics

This crates provides structs and functions to insert timeline data into
an elasticsearch index.

# CLI Usage

```
Usage: es4forensics [OPTIONS] --index <INDEX_NAME> --password <PASSWORD> <COMMAND>

Commands:
  create-index  
  import        
  help          Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose...           More output per occurrence
  -q, --quiet...             Less output per occurrence
      --strict               strict mode: do not only warn, but abort if an error occurs
  -I, --index <INDEX_NAME>   name of the elasticsearch index
  -H, --host <HOST>          server name or IP address of elasticsearch server [default: localhost]
  -P, --port <PORT>          API port number of elasticsearch server [default: 9200]
      --proto <PROTOCOL>     protocol to be used to connect to elasticsearch [default: https] [possible values: http, https]
  -k, --insecure             omit certificate validation
  -U, --username <USERNAME>  username for elasticsearch server [default: elastic]
  -W, --password <PASSWORD>  password for authenticating at elasticsearch
  -h, --help                 Print help information
  -V, --version              Print version information
```

## Creating Indices
```rust
use es4forensics::IndexBuilder;
use es4forensics::WithHost;
use elasticsearch::auth::Credentials;

let username = "elastic";
let password = "elastic";
let credentials = Credentials::Basic(username.to_string(), password.to_string());
let mut index = IndexBuilder::with_name("elastic4forensics_test".to_string())
    .with_host("127.0.0.1")
    .with_port(9200)
    .without_certificate_validation()
    .with_credentials(credentials)
    .create_index().await;
```
After doing this, you can easily add documents to the index using [`Index::add_timeline_object`]

## Adding documents to elasticsearch

For example, consider we have a line from a bodyfile. We need to convert this
into a [`ecs::objects::PosixFile`]-Object, which can then be added to an Index:

```rust
use es4forensics::objects::PosixFile;

let str_line = "0|/Users/Administrator ($FILE_NAME)|93552-48-2|d/drwxrwxrwx|0|0|92|1577092511|1577092511|1577092511|-1";
let posix_file: PosixFile = str_line.try_into().unwrap();

index.add_timeline_object(posix_file);
```

## Exporting documents in JSON format

Sometimes you might want to simply export your documents, instead of directly importing them into
elasticsearch.

Keep in mind that one bodyfile line might contain multiple different timestamps (up to four),
which yields up to four elasticsearch documents. Therefore, [`ecs::objects::ElasticObject::documents()`] returns an
iterator over [`serde_json::Value`]

```rust
use es4forensics::objects::PosixFile;
use es4forensics::Timestamp;
use crate::es4forensics::TimelineObject;
use serde_json::Value;

let str_line = "0|/Users/Administrator ($FILE_NAME)|93552-48-2|d/drwxrwxrwx|0|0|92|1577092511|1577092511|1577092511|-1";
let posix_file: PosixFile = str_line.try_into().unwrap();

for json_value in posix_file.into_values() {
    println!("{json_value}");
}
```

License: GPL-3.0
