pub mod objects;

use std::collections::{HashMap, HashSet};

use duplicate::duplicate_item;
use serde_json::{json, Value};

use crate::timestamp::Timestamp;

use self::objects::{PosixFile, WindowsEvent, Macb};

//mod file;
//pub use file::*;

pub trait ECSFields {
    fn into(&self) -> Value;
}

pub struct Ecs<'a> {
    ts: Timestamp,
    message: Option<String>,
    //labels: HashMap<String, String>,
    tags: HashSet<String>,
    file: Option<&'a PosixFile>,
    windows_event: Option<&'a WindowsEvent<'a>>,
    macb: Option<&'a Macb>,
}

impl<'a> Ecs<'a> {
    pub fn new(ts: Timestamp) -> Self {
        Self {
            ts,
            message: None,
            //labels: HashMap::new(),
            tags: HashSet::new(),
            file: None,
            windows_event: None,
            macb: None,
        }
    }

    pub fn with_additional_tag(mut self, tag: &str) -> Self {
        self.tags.insert(tag.to_owned());
        self
    }

    pub fn with_message(mut self, message: &str) -> Self {
        self.message = Some(message.to_owned());
        self
    }

    #[duplicate_item(
        method            attribute    ret_type;
      [ with_file ] [ file ] [ PosixFile ];
      [ with_windows_event ] [ windows_event ] [ WindowsEvent ];
      [ with_macb ] [ macb ] [ Macb ];
    )]
    pub fn method(mut self, ts: &'a ret_type) -> Self {
        self.attribute = Some(ts);
        self
    }
}

impl From<Ecs<'_>> for Value {
    fn from(val: Ecs) -> Value {
        let mut m = HashMap::from([
            ("@timestamp", Value::Number(val.ts.timestamp_millis().into())),
            ("ecs", json!({"version": "8.4"}))
        ]);

        if let Some(message) = val.message {
            m.insert("message", json!(message));
        }

        if ! val.tags.is_empty() {
            m.insert("tags", json!(val.tags));
        }

        if let Some(file) = val.file.as_ref() {
            let mut file_map: HashMap<&str, Value> = (*file).into();
            if let Some(macb) = val.macb {
                let macb_short: String = macb.into();
                let macb_long: Vec<&str> = macb.into();
                file_map.insert("macb_short", json!(macb_short));
                file_map.insert("macb_long", json!(macb_long));
            }

            m.insert("file", json!(file_map));
        }

        if let Some(event) = val.windows_event {
            m.extend(event);
        }

        json!(m)
    }
}

