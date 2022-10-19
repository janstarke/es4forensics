use std::collections::HashMap;

use bodyfile::Bodyfile3Line;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{ecs::ECS, Timestamp};

use super::MACB;

#[derive(Serialize, Deserialize)]
pub struct PosixFile {
    name: String,
    inode: String,
    uid: u64,
    gid: u64,
    size: u64,
    atime: Option<Timestamp>,
    mtime: Option<Timestamp>,
    ctime: Option<Timestamp>,
    crtime: Option<Timestamp>,
}

impl From<&PosixFile> for Value {
    fn from(val: &PosixFile) -> Self {
        let m: HashMap<&str, Value> = val.into();
        json!(m)
    }
}

impl From<&PosixFile> for HashMap<&str, Value> {
    fn from(val: &PosixFile) -> Self {
        let mut m = HashMap::from([
            ("path", json!(val.name.clone())),
            ("inode", json!(val.inode.clone())),
            ("uid", json!(val.uid)),
            ("gid", json!(val.gid)),
            ("size", json!(val.size)),
        ]);

        val.mtime.as_ref().and_then(|t| m.insert("mtime", t.into()));
        val.atime
            .as_ref()
            .and_then(|t| m.insert("accessed", t.into()));
        val.ctime.as_ref().and_then(|t| m.insert("ctime", t.into()));
        val.crtime
            .as_ref()
            .and_then(|t| m.insert("created", t.into()));
        m
    }
}

impl PosixFile {
    pub fn documents(&self) -> impl Iterator<Item = Value> {
        let mut docs = HashMap::new();
        self.add_document_to(&mut docs, &self.mtime);
        self.add_document_to(&mut docs, &self.atime);
        self.add_document_to(&mut docs, &self.ctime);
        self.add_document_to(&mut docs, &self.crtime);
        docs.into_values()
    }

    pub fn get_inode(&self) -> &str {
        &self.inode
    }

    fn load_timestamp(ts: i64) -> Option<Timestamp> {
        match ts {
            0 | -1 => None,
            _ => Some((ts * 1000).into()),
        }
    }

    fn generate_macb(&self, reference_ts: &Timestamp) -> MACB {
        let mut macb = MACB::default();

        if let Some(t) = self.mtime.as_ref() {
            macb.modified = t == reference_ts;
        }
        if let Some(t) = self.atime.as_ref() {
            macb.accessed = t == reference_ts;
        }
        if let Some(t) = self.ctime.as_ref() {
            macb.changed = t == reference_ts;
        }
        if let Some(t) = self.crtime.as_ref() {
            macb.created = t == reference_ts;
        }

        macb
    }

    fn add_document_to(&self, docs: &mut HashMap<Timestamp, Value>, ts: &Option<Timestamp>) {
        if let Some(t) = ts.as_ref() {
            let macb = self.generate_macb(t);
            docs.insert(
                t.clone(),
                ECS::new(t.clone())
                    .with_file(self)
                    .with_macb(&macb)
                    .with_additional_tag("bodyfile")
                    .with_message(&self.name)
                    .into(),
            );
        }
    }
}

impl From<Bodyfile3Line> for PosixFile {
    fn from(bfline: Bodyfile3Line) -> Self {
        Self {
            name: bfline.get_name().to_string(),
            inode: bfline.get_inode().to_string(),
            uid: bfline.get_uid(),
            gid: bfline.get_gid(),
            size: bfline.get_size(),
            atime: Self::load_timestamp(bfline.get_atime()),
            mtime: Self::load_timestamp(bfline.get_mtime()),
            ctime: Self::load_timestamp(bfline.get_ctime()),
            crtime: Self::load_timestamp(bfline.get_crtime()),
        }
    }
}
