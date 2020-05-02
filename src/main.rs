use itertools::Itertools;
use std::fs;
use tg_chat_stats::{History, Chat};
use crate::util::count_letters_by_actor;

mod util;

// TODO: print all chat names in a separate cli command
// TODO: support chats exclusion/inclusion
fn main() {
    let stats_file_path = "result.json"; // TODO: cli argument
    let example_json = fs::read_to_string(stats_file_path).unwrap();
    let history: History = serde_json::from_str(&example_json).unwrap();
    history
        .get_chats()
        .iter()
        .for_each(|chat| analyze_chat(chat));
}

fn analyze_chat(chat: &Chat) {
    let actor_to_letters = count_letters_by_actor(chat);
    println!("Chat: {}", chat.get_name());
    actor_to_letters
        .iter()
        .sorted_by(|a, b| Ord::cmp((*a).1, (*b).1))
        .rev()
        .take(20) // TODO: cli parameter
        .enumerate()
        .for_each(|a| println!("№{}: {}, letters: {}", a.0 + 1, (a.1).0, (a.1).1));
    println!("-------");
}
