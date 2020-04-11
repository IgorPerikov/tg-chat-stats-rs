use serde_json::Value;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Stats {
    chats: Chats,
}

#[derive(Deserialize, Debug)]
struct Chats {
    list: Vec<Chat>,
}

#[derive(Deserialize, Debug)]
struct Chat {
    name: String,
    messages: Vec<Message>,
}

#[derive(Deserialize, Debug)]
struct Message {
    from: Option<String>,
    actor: Option<String>,
    text: Value,
}

impl Message {
    fn get_author(&self) -> &String {
        let from_field = self.from.as_ref();
        let actor_field = self.actor.as_ref();
        return from_field.or(actor_field).unwrap();
    }

    fn get_text(&self) -> String {
        // TODO
        return String::from("123");
    }

    fn get_text_length(&self) -> usize {
        // TODO
        return self.get_text().len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value::String as ValueString;

    #[test]
    fn message_author_fetched_from_from() {
        let from = String::from("from");
        let message = Message {
            from: Some(from.clone()),
            actor: None,
            text: ValueString(String::new()),
        };
        assert_eq!(from, *message.get_author());
    }

    #[test]
    fn message_author_fetched_from_actor() {
        let actor = String::from("actor");
        let message = Message {
            from: None,
            actor: Some(actor.clone()),
            text: ValueString(String::new()),
        };
        assert_eq!(actor, *message.get_author());
    }

    #[test]
    #[should_panic]
    fn absence_of_from_and_actor_should_lead_to_panic() {
        let message = Message {
            from: None,
            actor: None,
            text: ValueString(String::new()),
        };
        message.get_author();
    }
}
