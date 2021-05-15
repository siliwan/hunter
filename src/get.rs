use std::fs::File;
use std::io::Read;

pub fn user_by_id(id: String) -> String {
    return String::from("");
}

pub fn user_by_key(key: String) -> String {
    return String::from("");
}

pub fn config() -> String {
    let mut config = File::open("/etc/orion/config.json").unwrap();
    let mut config_contents = String::from("");
    config.read_to_string(&mut config_contents).unwrap();
    return config_contents;
}
