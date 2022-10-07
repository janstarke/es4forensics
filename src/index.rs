use chrono::{TimeZone, DateTime};
use chrono_tz::Tz;
use elasticsearch::Elasticsearch;

pub struct Index {
    name: String,
    client: Elasticsearch,
}

impl Index {
    pub fn new(name: String, client: Elasticsearch) -> Self {
        Self {
            name,
            client
        }
    }
}