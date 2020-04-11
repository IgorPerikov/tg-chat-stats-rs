use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct History {
    chats: Chats,
}

#[derive(Deserialize, Debug)]
struct Chats {
    list: Vec<Chat>,
}

#[derive(Deserialize, Debug)]
pub struct Chat {
    name: String,
    messages: Vec<Message>,
}

#[derive(Deserialize, Debug)]
pub struct Message {
    from: Option<String>,
    actor: Option<String>,
    text: Value,
}

impl Message {
    pub fn get_author(&self) -> Option<&String> {
        let from_field = self.from.as_ref();
        let actor_field = self.actor.as_ref();
        return from_field.or(actor_field);
    }

    fn get_text(&self) -> String {
        return match &self.text {
            Value::String(s) => s.clone(),
            Value::Array(objects) => {
                let strings: Vec<String> = objects
                    .iter()
                    .map(|value| {
                        return match value {
                            Value::String(s) => s.clone(),
                            Value::Object(object_map) => {
                                object_map["text"].as_str().unwrap().to_string()
                            }
                            _ => panic!("Unexpected Value enum type inside Value::Array"),
                        };
                    })
                    .collect();
                return strings.join("");
            }
            _ => panic!("Unexpected Value enum type"),
        };
    }

    pub fn get_text_length(&self) -> usize {
        self.get_text().chars().count()
    }
}

impl History {
    pub fn get_chats(&self) -> &Vec<Chat> {
        return &self.chats.list;
    }
}

impl Chat {
    pub fn get_name(&self) -> &String {
        return &self.name;
    }

    pub fn get_messages(&self) -> &Vec<Message> {
        return &self.messages;
    }
}

#[cfg(test)]
mod message_get_author_tests {
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
        assert_eq!(from, *message.get_author().unwrap());
    }

    #[test]
    fn message_author_fetched_from_actor() {
        let actor = String::from("actor");
        let message = Message {
            from: None,
            actor: Some(actor.clone()),
            text: ValueString(String::new()),
        };
        assert_eq!(actor, *message.get_author().unwrap());
    }

    #[test]
    fn from_and_actor_absence_should_return_none() {
        let message = Message {
            from: None,
            actor: None,
            text: ValueString(String::new()),
        };
        assert!(message.get_author().is_none());
    }
}

#[cfg(test)]
mod message_get_text_tests {
    use super::*;
    use serde_json::Map;
    use serde_json::Value::Array;
    use serde_json::Value::Object;
    use serde_json::Value::String as ValueString;

    #[test]
    fn single_string_text_should_be_returned() {
        let text = String::from("single-string-text");
        let message = Message {
            from: None,
            actor: None,
            text: ValueString(text.clone()),
        };
        assert_eq!(text, message.get_text());
    }

    #[test]
    fn multipart_text_should_be_concatenated_and_returned_as_whole() {
        let text1 = String::from("a");
        let text2 = String::from("b");
        let text3 = String::from("c");
        let mut object_map = Map::new();
        object_map.insert(String::from("text"), ValueString(text2.clone()));
        let message = Message {
            from: None,
            actor: None,
            text: Array(vec![
                ValueString(text1.clone()),
                Object(object_map),
                ValueString(text3.clone()),
            ]),
        };

        let mut result = String::new();
        result.push_str(&text1);
        result.push_str(&text2);
        result.push_str(&text3);

        assert_eq!(message.get_text(), result);
    }
}

#[cfg(test)]
mod message_get_text_length_tests {
    use super::*;
    use serde_json::Value::String as ValueString;

    #[test]
    fn should_return_correct_length() {
        let text = String::from("hello");
        let message = Message {
            from: None,
            actor: None,
            text: ValueString(text.clone()),
        };
        assert_eq!(5, message.get_text_length());
    }

    #[test]
    fn should_return_text_length_based_on_number_of_chars_not_bytes() {
        let text = String::from("привет");
        let message = Message {
            from: None,
            actor: None,
            text: ValueString(text.clone()),
        };
        assert_eq!(6, message.get_text_length());
    }
}
