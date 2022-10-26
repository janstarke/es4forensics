use chrono::{DateTime, Utc, TimeZone, NaiveDateTime, LocalResult};
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::hash::Hash;
use anyhow::{anyhow, Result};

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Hash, PartialOrd, Ord)]
pub struct Timestamp {
    ts: i64,
}

impl From<DateTime<Tz>> for Timestamp {
    fn from(d: DateTime<Tz>) -> Self {
        let ts = d.with_timezone(&Utc);
        log::error!("converting {} to {}", d, ts.timestamp_millis());
        Self {
            ts: ts.timestamp_millis(),
        }
    }
}


impl TryFrom<(i64, &Tz)> for Timestamp {
    type Error = anyhow::Error;

    fn try_from((unix_ts, src_tz): (i64, &Tz)) -> Result<Self, Self::Error> {
        let ts = match src_tz.from_local_datetime(&NaiveDateTime::from_timestamp(unix_ts, 0)) {
            LocalResult::None => {
                return Err(anyhow!("INVALID DATETIME"));
            }
            LocalResult::Single(t) => t,
            LocalResult::Ambiguous(t1, _t2) => t1,
        };
        Ok(
            Self {
                ts: ts.timestamp_millis(),
            }
        )
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
}
