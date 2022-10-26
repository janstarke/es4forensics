use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde_json::{json, Value};

use crate::{ecs::Ecs, timestamp::Timestamp};

pub struct WindowsEvent<'a> {
    event_record_id: u64,

    timestamp: DateTime<Utc>,

    event_id: u64,

    provider_name: &'a Value,
    channel_name: &'a Value,
    activity_id: Option<&'a Value>,
    custom_data: HashMap<&'a String, &'a Value>,
}

impl<'a> WindowsEvent<'a> {
    pub fn new(
        event_record_id: u64,
        timestamp: DateTime<Utc>,
        event_id: u64,
        provider_name: &'a Value,
        channel_name: &'a Value,
        activity_id: Option<&'a Value>,
        custom_data: HashMap<&'a String, &'a Value>,
    ) -> Self {
        Self {
            event_record_id,
            timestamp,
            event_id,
            provider_name,
            channel_name,
            activity_id,
            custom_data
        }
    }
    pub fn documents(&self) -> impl Iterator<Item = (Timestamp, Value)> {
        let mut docs: Vec<(Timestamp, Value)> = Vec::new();
        let ecs = Ecs::new(self.timestamp.into()).with_windows_event(self);
        docs.push((self.timestamp.into(), ecs.into()));
        docs.into_iter()
    }
}

impl From<&WindowsEvent<'_>> for Value {
    fn from(me: &WindowsEvent) -> Self {
        json!({
            "code": me.event_id,
            "sequence": me.event_record_id,
            "module": me.channel_name,
            "provider": me.provider_name,
            "custom_data": me.custom_data,
            "activity": me.activity_id,
        })
    }
}
