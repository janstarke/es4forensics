use bodyfile::Bodyfile3Line;
use elastic4forensics::objects::PosixFile;
use serde_json::{json, Value};

#[test]
pub fn test_bodyfile_single() {
    let str_line = "0|/Users/Administrator ($FILE_NAME)|93552-48-2|d/drwxrwxrwx|0|0|92|1577092511|1577092511|1577092511|-1";
    let bf_line = Bodyfile3Line::try_from(str_line).unwrap();
    let pfile = PosixFile::from(bf_line);
    let values: Vec<Value> = pfile.documents().collect();
    let actual = json!(values);
    let expected = json!([{
    "@timestamp":1577092511,
    "ecs":{"version":"1.0.0"},
    "file":{
        "accessed":1577092511,
        "ctime":1577092511,
        "gid":0,
        "inode":"93552-48-2",
        "mtime":1577092511,
        "path":"/Users/Administrator ($FILE_NAME)",
        "size":92,
        "uid":0}
    }]);
    assert_eq!(actual, expected);
}

#[test]
pub fn test_bodyfile_multiple() {
    let str_line = "0|/Users/Administrator ($FILE_NAME)|93552-48-2|d/drwxrwxrwx|0|0|92|1577092511|1577092511|1577092511|1577092512";
    let bf_line = Bodyfile3Line::try_from(str_line).unwrap();
    let pfile = PosixFile::from(bf_line);
    let values: Vec<Value> = pfile.documents().collect();
    let actual = json!(values);
    let expected = json!([{
    "@timestamp":1577092511,
    "ecs":{"version":"1.0.0"},
    "file":{
        "accessed":1577092511,
        "ctime":1577092511,
        "created":1577092512,
        "gid":0,
        "inode":"93552-48-2",
        "mtime":1577092511,
        "path":"/Users/Administrator ($FILE_NAME)",
        "size":92,
        "uid":0}
    },
    {
        "@timestamp":1577092512,
        "ecs":{"version":"1.0.0"},
        "file":{
            "accessed":1577092511,
            "ctime":1577092511,
            "created":1577092512,
            "gid":0,
            "inode":"93552-48-2",
            "mtime":1577092511,
            "path":"/Users/Administrator ($FILE_NAME)",
            "size":92,
            "uid":0}
    }]);
    assert_eq!(actual, expected);
}
