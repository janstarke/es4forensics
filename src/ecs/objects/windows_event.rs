use std::collections::HashMap;

use anyhow::bail;
use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::{json, Value};
use num_derive::{FromPrimitive, ToPrimitive};    
use num_traits::{FromPrimitive, ToPrimitive};

use crate::{ecs::Ecs, timestamp::Timestamp};

pub struct WindowsEvent<'a> {
    event_record_id: u64,

    timestamp: DateTime<Utc>,

    event_id: u64,
    level: EventLevel,
    computer: &'a Value,

    provider_name: &'a Value,
    channel_name: &'a Value,
    activity_id: Option<&'a Value>,
    custom_data: HashMap<&'a String, &'a Value>,
}

/// Source: <https://learn.microsoft.com/de-de/dotnet/api/system.diagnostics.tracing.eventlevel?view=net-6.0>
#[derive(FromPrimitive, ToPrimitive, Serialize)]
pub enum EventLevel {
    LogAlways = 0,
    Critical = 1,
    Error = 2,
    Warning = 3,
    Information = 4,
    Verbose = 5,
}

impl TryFrom<&Value> for EventLevel {
    type Error = anyhow::Error;

    fn try_from(value: &Value) -> Result<Self, <EventLevel as TryFrom<&Value>>::Error> {
        match value.as_u64() {
            Some(v) => {
                match EventLevel::from_u64(v) {
                    Some(el) => Ok(el),
                    None => bail!("invalid log level: {}", v),
                }
            },
            None => bail!("unable to convert event level '{:?}' to a number", value)
        }
    }
}

impl<'a> WindowsEvent<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        event_record_id: u64,
        timestamp: DateTime<Utc>,
        event_id: u64,
        level: EventLevel,
        computer: &'a Value,
        provider_name: &'a Value,
        channel_name: &'a Value,
        activity_id: Option<&'a Value>,
        custom_data: HashMap<&'a String, &'a Value>,
    ) -> Self {
        Self {
            event_record_id,
            timestamp,
            event_id,
            level,
            computer,
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

impl IntoIterator for &WindowsEvent<'_> {
    type Item = (&'static str, Value);

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut values = Vec::new();
        let event = json!({
            "code": self.event_id,
            "sequence": self.event_record_id,
            "module": self.channel_name,
            "provider": self.provider_name,
            "custom_data": self.custom_data,
            "activity": self.activity_id,
            "kind": "event",
            "severity": self.level.to_u8().unwrap(),
        });
        let host = json!({
            "name": self.computer,
        });
        let log = json!({
            "syslog": {
                "severity": {
                    "code": self.level.to_u8().unwrap(),
                    "name": self.level
                }
            }
        });

        values.push(("event", event));
        values.push(("host", host));
        values.push(("log", log));
        values.into_iter()
    }
}