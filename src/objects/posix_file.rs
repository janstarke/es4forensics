use std::collections::HashMap;

use bodyfile::Bodyfile3Line;
use serde::{Serialize, Deserialize};
use serde_json::{Value, json};

use crate::{Timestamp, utils::json::add_to_json, ecs::{File, ECS}};

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

impl From<&PosixFile> for File {
    fn from(me: &PosixFile) -> Self {
        let mut obj = Self::new(me.name.clone())
            .with_inode(me.inode.clone())
            .with_uid(me.uid)
            .with_gid(me.gid)
            .with_size(me.size)
        ;
        
        if let Some(t) = me.atime.as_ref() { obj = obj.with_accessed(t.clone()); }
        if let Some(t) = me.mtime.as_ref() { obj = obj.with_mtime(t.clone()); }
        if let Some(t) = me.ctime.as_ref() { obj = obj.with_ctime(t.clone()); }
        if let Some(t) = me.crtime.as_ref() { obj = obj.with_created(t.clone()); }
        obj
    }
}

impl PosixFile {
    fn load_timestamp(ts: i64) -> Option<Timestamp> {
        match ts {
            0 | -1 => None,
            _ => Some(ts.into())
        }
    }

    pub fn documents(&self) -> impl Iterator<Item=Value> {
        let mut docs = HashMap::new();

        if let Some(atime) = self.atime.as_ref() {
            docs.insert(atime.clone(), ECS::new(atime.clone()).with_file(self.into()).into());
        }

        if let Some(mtime) = self.mtime.as_ref() {
            if ! docs.contains_key(mtime) {
                docs.insert(mtime.clone(), ECS::new(mtime.clone()).with_file(self.into()).into());
            }
        }

        if let Some(ctime) = self.ctime.as_ref() {
            if ! docs.contains_key(ctime) {
                docs.insert(ctime.clone(), ECS::new(ctime.clone()).with_file(self.into()).into());
            }
        }

        if let Some(crtime) = self.crtime.as_ref() {
            if ! docs.contains_key(crtime) {
                docs.insert(crtime.clone(), ECS::new(crtime.clone()).with_file(self.into()).into());
            }
        }

        docs.into_values()
    }

    pub fn get_inode(&self) -> &str { &self.inode }
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