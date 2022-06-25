pub mod docs;

use std::fs::File;

use serde_json;

pub fn parse_forge_commands(filename: &str) -> serde_json::Value {
    let file = File::open(filename).unwrap();
    let json: serde_json::Value =
        serde_json::from_reader(file).unwrap();
    println!("{:?}", json);
    return json;
}
