use std::collections::HashMap;
use tg_chat_stats::Chat;

pub fn count_letters_by_actor(chat: &Chat) -> HashMap<&str, usize> {
    let mut actor_to_letters = HashMap::new();
    chat.get_messages()
        .iter()
        .filter(|message| message.not_an_action())
        .filter(|message| message.get_author().is_some())
        .filter(|message| !message.get_text().starts_with("#botd"))
        .for_each(|message| {
            let x = actor_to_letters
                .entry(message.get_author().unwrap().as_str())
                .or_insert(0);
            *x += message.get_text_length();
        });
    return actor_to_letters;
}
