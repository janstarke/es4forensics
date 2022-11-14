use assert_json_diff::assert_json_eq;
use bodyfile::Bodyfile3Line;
use es4forensics::{objects::PosixFile, TimelineObject};
use serde_json::{json, Value};

#[test]
pub fn test_bodyfile_single() {
    let str_line = "0|/Users/Administrator ($FILE_NAME)|93552-48-2|d/drwxrwxrwx|0|0|92|1577092511|1577092511|1577092511|-1";
    let bf_line = Bodyfile3Line::try_from(str_line).unwrap();
    let pfile = PosixFile::try_from((bf_line, &chrono_tz::UTC)).unwrap();
    let values: Vec<Value> = pfile.into_values().collect();
    let actual = json!(values);
    let expected = json!([{
    "@timestamp":1577092511000_u64,
    "ecs":{"version":"8.4"},
    "message": "/Users/Administrator ($FILE_NAME)",
    "tags": ["bodyfile"],
    "file":{
        "accessed":1577092511000_u64,
        "ctime":1577092511000_u64,
        "gid":0,
        "inode":"93552-48-2",
        "mtime":1577092511000_u64,
        "path":"/Users/Administrator ($FILE_NAME)",
        "name":"Administrator ($FILE_NAME)",
        "directory":"/Users",
        "mode": "",
        "size":92,
        "uid":0,
        //"macb_long": ["modified", "accessed", "changed"],
        //"macb_short": "mac."
        }
    }]);
    assert_json_eq!(actual, expected);
}

//#[test]
pub fn test_bodyfile_multiple() {
    let str_line = "0|/Users/Administrator ($FILE_NAME)|93552-48-2|d/drwxrwxrwx|0|0|92|1577092511|1577092511|1577092511|1577092512";
    let bf_line = Bodyfile3Line::try_from(str_line).unwrap();
    let pfile = PosixFile::try_from((bf_line, &chrono_tz::UTC)).unwrap();
    let values: Vec<Value> = pfile.into_values().collect();
    let actual = json!(values);
    let expected = json!([{
    "@timestamp":1577092511000_u64,
    "ecs":{"version":"8.4"},
    "file":{
        "accessed":1577092511000_u64,
        "ctime":1577092511000_u64,
        "created":1577092512000_u64,
        "gid":0,
        "inode":"93552-48-2",
        "mtime":1577092511000_u64,
        "path":"/Users/Administrator ($FILE_NAME)",
        "size":92,
        "uid":0}
    },
    {
        "@timestamp":1577092512000_u64,
        "ecs":{"version":"1.0.0"},
        "file":{
            "accessed":1577092511000_u64,
            "ctime":1577092511000_u64,
            "created":1577092512000_u64,
            "gid":0,
            "inode":"93552-48-2",
            "mtime":1577092511000_u64,
            "path":"/Users/Administrator ($FILE_NAME)",
            "size":92,
            "uid":0}
    }]);
    assert_json_eq!(actual, expected);
}
