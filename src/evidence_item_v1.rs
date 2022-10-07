use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{EvidenceObjectV1, Timestamp, utils::json::add_to_json};

#[derive(Serialize, Deserialize)]
pub struct EvidenceItemV1 {
    version: u16,
    host: Option<String>,
    object: EvidenceObjectV1,
}

impl From<EvidenceObjectV1> for EvidenceItemV1 {
    fn from(object: EvidenceObjectV1) -> Self {
        Self {
            version: 1,
            host: None,
            object,
        }
    }
}

impl EvidenceItemV1 {
    pub fn with_host(mut self, host: String) -> Self {
        self.host = Some(host);
        self
    }

    pub fn documents(&self) -> impl Iterator<Item = Value> + '_ {
        self.object.documents().map(|doc| match self.host.as_ref() {
            None => doc,
            Some(host) => add_to_json(&doc, "host", Value::String(host.clone()))
        })
    }
}
