use std::collections::HashMap;

use duplicate::duplicate_item;
use serde_json::{json, Value};

use crate::Timestamp;

mod file;
pub use file::*;

pub trait ECSFields {
    fn into(&self) -> Value;
}

pub struct ECS {
    ts: Timestamp,
    message: Option<String>,
    labels: HashMap<String, String>,
    tags: Vec<String>,
    file: Option<File>,
}

impl ECS {
    pub fn new(ts: Timestamp) -> Self {
        Self {
            ts,
            message: None,
            labels: HashMap::new(),
            tags: Vec::new(),
            file: None,
        }
    }

    #[duplicate_item(
        method            attribute    ret_type;
      [ with_file ] [ file ] [ File ];
    )]
    pub fn method(mut self, ts: ret_type) -> Self {
        self.attribute = Some(ts);
        self
    }
}

impl From<ECS> for Value {
    fn from(val: ECS) -> Value {
        let mut m = HashMap::new();
        m.insert(
            "@timestamp",
            Value::Number(val.ts.timestamp_millis().into()),
        );
        m.insert("ecs", json!({"version": "1.0.0"}));

        val.file.as_ref().and_then(|f| m.insert("file", f.into()));

        json!(m)
    }
}

