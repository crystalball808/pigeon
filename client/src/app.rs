#[derive(Debug)]
pub struct Message {
    pub author_name: String,
    pub content: String,
}

pub struct App {
    pub input_value: String,
    pub messages: Vec<Message>,
}

impl App {
    pub fn new() -> App {
        App {
            input_value: String::new(),
            messages: Vec::new(),
        }
    }

    pub fn extract_message(&mut self) -> Option<Message> {
        if self.input_value.is_empty() == false {
            self.input_value.push_str("\n");
            let message = Message {
                author_name: "Client".to_owned(),
                content: self.input_value.clone(),
            };
            self.input_value.clear();

            return Some(message);
        } else {
            return None;
        }
    }
}
