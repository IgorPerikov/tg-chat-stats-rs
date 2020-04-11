use serde_json::Value;
use std::fs;
use tg_chat_stats::History;

fn main() {
    let stats_file_path = "example.json"; // TODO: get as cli argument
    let example_json = fs::read_to_string(stats_file_path).unwrap();
    let history: History = serde_json::from_str(&example_json).unwrap();
    println!("{:?}", history); // TODO
}
