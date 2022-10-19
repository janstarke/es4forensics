use serde::{Serialize, Deserialize};
use serde_json::Value;

use crate::ecs::objects::*;


#[derive(Serialize, Deserialize)]
pub enum EvidenceObjectV1 {
    PosixFile (PosixFile),
    NtfsFile (NtfsFile),
    RegistryKey (RegistryKey),
    WindowsEvent (WindowsEvent),
    SimpleEvent (SimpleEvent),
    ADObject (ADObject)
}

impl EvidenceObjectV1 {
    pub fn documents(&self) -> Box<dyn Iterator<Item=Value>> {
        match self {
            EvidenceObjectV1::PosixFile(o) => Box::new(o.documents()),
            EvidenceObjectV1::NtfsFile(o) => Box::new(o.documents()),
            EvidenceObjectV1::RegistryKey(o) => Box::new(o.documents()),
            EvidenceObjectV1::WindowsEvent(o) => Box::new(o.documents()),
            EvidenceObjectV1::SimpleEvent(o) => Box::new(o.documents()),
            EvidenceObjectV1::ADObject(o) => Box::new(o.documents()),
        }
    }
}