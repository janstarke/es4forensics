use std::path::PathBuf;

use duplicate::duplicate_item;
use serde::Serialize;

use crate::timestamp::Timestamp;

use super::ecs_object::EcsObject;

#[derive(Serialize)]
pub enum FileType {
    File,
    Dir,
    Symlink,
}

#[derive(Serialize, Default)]
pub struct File {
    mtime: Option<Timestamp>,
    accessed: Option<Timestamp>,
    ctime: Option<Timestamp>,
    created: Option<Timestamp>,
    directory: Option<String>,
    extension: Option<String>,
    gid: u64,
    uid: u64,
    inode: String,
    mode: String,
    name: Option<String>,
    path: Option<String>,
    size: u64,
    target_path: Option<String>,

    #[serde(rename = "type")]
    file_type: Option<FileType>,
}

impl From<String> for File {
    fn from(filename: String) -> Self {
        let buf = PathBuf::from(&filename);
        Self {
            name: buf.file_name().map(|s| s.to_string_lossy().to_string()),
            extension: buf.extension().map(|s| s.to_string_lossy().to_string()),
            directory: buf.parent().map(|s| s.to_string_lossy().to_string()),
            path: Some(filename),
            ..Default::default()
        }
    }
}

impl File {
    #[duplicate_item(
        method            attribute    ret_type;
      [ with_mtime ]       [ mtime ]       [ Timestamp ];
      [ with_accessed ]    [ accessed ]    [ Timestamp ];
      [ with_ctime ]       [ ctime ]       [ Timestamp ];
      [ with_created ]     [ created ]     [ Timestamp ];
      [ with_target_path ] [ target_path ] [ String ];
      [ with_type ]        [ file_type ]   [ FileType ];
   )]
    pub fn method(mut self, ts: Option<ret_type>) -> Self {
        self.attribute = ts;
        self
    }

    #[duplicate_item(
        method            attribute    ret_type;
      [ with_gid ]   [ gid ]   [ u64 ];
      [ with_uid ]   [ uid ]   [ u64 ];
      [ with_inode ] [ inode ] [ String ];
      [ with_mode ]  [ mode ]  [ String ];
      [ with_size ]  [ size ]  [ u64 ];
   )]
    pub fn method(mut self, ts: ret_type) -> Self {
        self.attribute = ts;
        self
    }
}

impl EcsObject for File {
    fn object_key(&self) -> &'static str {
        "file"
    }
}
