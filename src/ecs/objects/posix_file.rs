use std::collections::{HashMap, hash_map};
use anyhow::Result;

use bodyfile::Bodyfile3Line;
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{ecs::Ecs, timestamp::Timestamp};

use super::{Macb, ElasticObject};

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

impl ElasticObject for PosixFile {
    type DocsIter = hash_map::IntoValues<Timestamp, Value>;
    fn documents(&self) -> Self::DocsIter {
        let mut docs = HashMap::new();
        self.add_document_to(&mut docs, &self.mtime);
        self.add_document_to(&mut docs, &self.atime);
        self.add_document_to(&mut docs, &self.ctime);
        self.add_document_to(&mut docs, &self.crtime);
        docs.into_values()
    }
}

impl PosixFile {
    pub fn get_inode(&self) -> &str {
        &self.inode
    }

    fn load_timestamp(ts: i64, tz: &Tz) -> Result<Option<Timestamp>> {
        match ts {
            -1 => Ok(None),
            _ => Ok(Some((ts * 1000, tz).try_into()?)),
        }
    }

    fn generate_macb(&self, reference_ts: &Timestamp) -> Macb {
        let mut macb = Macb::default();

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
                Ecs::new(t.clone())
                    .with_file(self)
                    .with_macb(&macb)
                    .with_additional_tag("bodyfile")
                    .with_message(&self.name)
                    .into(),
            );
        }
    }
}

impl TryFrom<(Bodyfile3Line, &Tz)> for PosixFile {
    type Error = anyhow::Error;
    fn try_from((bfline, src_tz): (Bodyfile3Line, &Tz)) -> Result<Self> {
        Ok(Self {
            name: bfline.get_name().to_string(),
            inode: bfline.get_inode().to_string(),
            uid: bfline.get_uid(),
            gid: bfline.get_gid(),
            size: bfline.get_size(),
            atime: Self::load_timestamp(bfline.get_atime(), src_tz)?,
            mtime: Self::load_timestamp(bfline.get_mtime(), src_tz)?,
            ctime: Self::load_timestamp(bfline.get_ctime(), src_tz)?,
            crtime: Self::load_timestamp(bfline.get_crtime(), src_tz)?,
        })
    }
}

impl TryFrom<(&Bodyfile3Line, &Tz)> for PosixFile {
    type Error = anyhow::Error;
    fn try_from((bfline, src_tz): (&Bodyfile3Line, &Tz)) -> Result<Self> {
        Ok(Self {
            name: bfline.get_name().to_string(),
            inode: bfline.get_inode().to_string(),
            uid: bfline.get_uid(),
            gid: bfline.get_gid(),
            size: bfline.get_size(),
            atime: Self::load_timestamp(bfline.get_atime(), src_tz)?,
            mtime: Self::load_timestamp(bfline.get_mtime(), src_tz)?,
            ctime: Self::load_timestamp(bfline.get_ctime(), src_tz)?,
            crtime: Self::load_timestamp(bfline.get_crtime(), src_tz)?,
        })
    }
}