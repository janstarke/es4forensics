use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::hash::Hash;

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct Timestamp {
    ts: i64,
    tz: String,
}

impl Hash for Timestamp {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.ts.hash(state);
        self.tz.hash(state);
    }
}

impl From<DateTime<Tz>> for Timestamp {
    fn from(d: DateTime<Tz>) -> Self {
        let ts = d.with_timezone(&Utc);
        Self {
            ts: ts.timestamp_millis(),
            tz: d.timezone().to_string(),
        }
    }
}

impl From<i64> for Timestamp {
    fn from(ts: i64) -> Self {
        Self {
            ts,
            tz: Utc.to_string(),
        }
    }
}

impl From<&Timestamp> for Value {
    fn from(ts: &Timestamp) -> Self {
        json!(ts.ts)
    }
}

impl Timestamp {
    pub fn timestamp_millis(&self) -> i64 {
        self.ts
    }
    pub fn original_tz(&self) -> &str {
        &self.tz
    }
}
