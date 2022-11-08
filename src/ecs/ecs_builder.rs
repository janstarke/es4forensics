use std::collections::{HashMap, HashSet};

use anyhow::bail;
use duplicate::duplicate_item;
use serde_json::{Value, json};

use crate::ecs::ecs_object::EcsObject;
use crate::ecs::{Event, Host, log::Log, File};
use crate::timestamp::Timestamp;

pub struct EcsBuilder {
    ts: Timestamp,
    message: Option<String>,
    //labels: HashMap<String, String>,
    tags: HashSet<String>,
    contents: HashMap<&'static str, Value>
}

impl EcsBuilder {

    pub fn with(ts: Timestamp) -> Self {
        Self {
            ts,
            message: None,
            tags: HashSet::default(),
            contents: HashMap::default()
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
        method       ret_type;
    [ with_event ] [ Event<'_> ];
    [ with_host ]  [ Host ];
    [ with_log ]   [ Log ];
    [ with_file ]  [ File ];
    )]
    pub fn method(mut self, ts: ret_type) -> anyhow::Result<Self> {
        if self.contents.contains_key(ts.object_key()) {
            bail!("unambigious key: '{}'", ts.object_key());
        }
        self.contents.insert(ts.object_key(), json!(ts));
        Ok(self)
    }
}

impl From<EcsBuilder> for Value {
    fn from(val: EcsBuilder) -> Value {
        let mut m = HashMap::from([
            (
                "@timestamp",
                Value::Number(val.ts.timestamp_millis().into()),
            ),
            ("ecs", json!({"version": "8.4"})),
        ]);

        if let Some(message) = val.message {
            m.insert("message", json!(message));
        }

        if !val.tags.is_empty() {
            m.insert("tags", json!(val.tags));
        }

        for (key, value) in val.contents.into_iter() {
            m.insert(key, value);
        }
        json!(m)

    }
}
