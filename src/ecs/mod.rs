use std::collections::{HashMap, HashSet};

use duplicate::duplicate_item;
use serde_json::{json, Value};

use crate::{Timestamp, objects::{PosixFile, MACB}};

//mod file;
//pub use file::*;

pub trait ECSFields {
    fn into(&self) -> Value;
}

pub struct ECS<'a> {
    ts: Timestamp,
    message: Option<String>,
    labels: HashMap<String, String>,
    tags: HashSet<String>,
    file: Option<&'a PosixFile>,
    macb: Option<&'a MACB>,
}

impl<'a> ECS<'a> {
    pub fn new(ts: Timestamp) -> Self {
        Self {
            ts,
            message: None,
            labels: HashMap::new(),
            tags: HashSet::new(),
            file: None,
            macb: None,
        }
    }

    pub fn with_additional_tag(mut self, tag: &str) -> Self {
        self.tags.insert(tag.to_owned());
        self
    }

    #[duplicate_item(
        method            attribute    ret_type;
      [ with_file ] [ file ] [ PosixFile ];
      [ with_macb ] [ macb ] [ MACB ];
    )]
    pub fn method(mut self, ts: &'a ret_type) -> Self {
        self.attribute = Some(ts);
        self
    }
}

impl From<ECS<'_>> for Value {
    fn from(val: ECS) -> Value {
        let mut m = HashMap::new();
        m.insert(
            "timestamp",
            Value::Number(val.ts.timestamp_millis().into()),
        );
        m.insert("ecs", json!({"version": "1.0.0"}));

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

        json!(m)
    }
}

