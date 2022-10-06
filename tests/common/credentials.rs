use std::env;

pub fn username_from_env() -> String {
    match env::var("E4F_USER") {
        Ok(s) => s,
        Err(_) => "elastic".to_owned()
    }
}

pub fn password_from_env() -> String {
    match env::var("E4F_PASSWORD") {
        Ok(s) => s,
        Err(_) => panic!("You need to set E4F_PASSWORD")
    }
}