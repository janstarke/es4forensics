use std::collections::HashMap;
use duplicate::duplicate_item;

use serde_json::{Value, json};

use crate::Timestamp;

#[derive(Default)]
pub struct File {
    path: String,
    accessed: Option<Timestamp>,
    created: Option<Timestamp>,
    ctime: Option<Timestamp>,
    mtime: Option<Timestamp>,
    inode: Option<String>,
    uid: Option<u64>,
    gid: Option<u64>,
    size: Option<u64>,
}

impl From<&File> for Value {
    fn from(val: &File) -> Value {
        let mut m = HashMap::new();
        m.insert("path", Value::String(val.path.clone()));
        val.accessed.as_ref().and_then(|t| m.insert("accessed", t.into()));
        val.created.as_ref().and_then(|t| m.insert("created", t.into()));
        val.ctime.as_ref().and_then(|t| m.insert("ctime", t.into()));
        val.mtime.as_ref().and_then(|t| m.insert("mtime", t.into()));

        val.inode.as_ref().and_then(|t| m.insert("inode", json!(t)));
        val.uid.as_ref().and_then(|t| m.insert("uid", json!(t)));
        val.gid.as_ref().and_then(|t| m.insert("gid", json!(t)));
        val.size.as_ref().and_then(|t| m.insert("size", json!(t)));

        json!(m)
    }
}

impl File {
    pub fn new(path: String) -> Self {
        let mut me = Self::default();
        me.path = path;
        me
    }

    #[duplicate_item(
          method            attribute    ret_type;
        [ with_accessed ] [ accessed ] [ Timestamp ];
        [ with_created ]  [ created ]  [ Timestamp ];
        [ with_ctime ]    [ ctime ]    [ Timestamp ];
        [ with_mtime ]    [ mtime ]    [ Timestamp ];
        [ with_inode ]    [ inode ]    [ String ];
        [ with_uid ]      [ uid ]      [ u64 ];
        [ with_gid ]      [ gid ]      [ u64 ];
        [ with_size ]     [ size ]     [ u64 ];
    )]
    pub fn method(mut self, ts: ret_type) -> Self {
        self.attribute = Some(ts);
        self
    }
}