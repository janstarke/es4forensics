use num_derive::{FromPrimitive, ToPrimitive};
use serde::Serialize;

/// Source: <https://learn.microsoft.com/de-de/dotnet/api/system.diagnostics.tracing.eventlevel?view=net-6.0>
#[derive(strum_macros::Display, FromPrimitive, ToPrimitive, Serialize)]
#[strum(serialize_all = "lowercase")]
pub enum EventLevel {
    LogAlways = 0,
    Critical = 1,
    Error = 2,
    Warning = 3,
    Information = 4,
    Verbose = 5,
}