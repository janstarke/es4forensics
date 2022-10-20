[![Crates.io](https://img.shields.io/crates/v/es4forensics)](https://crates.io/crates/es4forensics)
[![docs.rs](https://img.shields.io/docsrs/es4forensics)](https://docs.rs/crate/es4forensics)
![Crates.io](https://img.shields.io/crates/l/es4forensics)
![Crates.io (latest)](https://img.shields.io/crates/dv/es4forensics)

# es4forensics

This crates provides structs and functions to insert timeline data into
an elasticsearch index.

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
    .build();
```
After doing this, you can easily add documents to the index using [`Index::add_timeline_object`]

## Adding documents to elasticsearch

For example, consider we have a line from a bodyfile. We need to convert this
into a [`ecs::objects::PosixFile`]-Object, which can then be added to an Index:

```rust
use bodyfile::Bodyfile3Line;
use es4forensics::objects::PosixFile;

let str_line = "0|/Users/Administrator ($FILE_NAME)|93552-48-2|d/drwxrwxrwx|0|0|92|1577092511|1577092511|1577092511|-1";
let bf_line = Bodyfile3Line::try_from(str_line).unwrap();

index.add_timeline_object(PosixFile::from(bf_line));
```

## Exporting documents in JSON format

Sometimes you might want to simply export your documents, instead of directly importing them into
elasticsearch.

Keep in mind that one bodyfile line might contain multiple different timestamps (up to four),
which yields up to four elasticsearch documents. Therefore, [`ecs::objects::ElasticObject::documents()`] returns an
iterator over [`serde_json::Value`]

```rust
use bodyfile::Bodyfile3Line;
use es4forensics::objects::PosixFile;
use es4forensics::objects::ElasticObject;

let str_line = "0|/Users/Administrator ($FILE_NAME)|93552-48-2|d/drwxrwxrwx|0|0|92|1577092511|1577092511|1577092511|-1";
let bf_line = Bodyfile3Line::try_from(str_line).unwrap();

for json_value in PosixFile::from(bf_line).documents() {
    println!("{json_value}");
}
```

License: GPL-3.0
