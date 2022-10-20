use std::collections::HashMap;

use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::{Serialize, Deserialize};
use serde_json::Value;

use crate::{Timestamp, utils::json::add_to_json};

#[derive(Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable")]
pub struct WindowsEvent {
    #[builder(pattern = "owned", setter(prefix="with"))]
    event_record_id: u64,

    #[serde(with="chrono::serde::ts_milliseconds")]
    #[builder(pattern = "owned", setter(prefix="with"))]
    timestamp: DateTime<Utc>,

    #[builder(pattern = "owned", setter(prefix="with"))]
    event_id: u64,
}

impl WindowsEvent {
    pub fn documents(&self) -> impl Iterator<Item=Value> {
        let docs: HashMap<Timestamp, Value> = HashMap::new();
        docs.into_iter().map(|(ts, v)| {
            add_to_json(&v, "|@timestamp|", Value::Number(ts.timestamp_millis().into()))
        })
    }
}