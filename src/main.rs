use std::fs;
use serde_json::Value;
use tg_chat_stats::Stats;

fn main() {
    let stats_file_path = "example.json"; // TODO: get as cli argument
    let example_json = fs::read_to_string(stats_file_path).unwrap();
    let stats: Stats = serde_json::from_str(&example_json).unwrap();
    println!("{:?}", stats); // TODO

}
