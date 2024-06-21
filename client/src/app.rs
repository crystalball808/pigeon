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

    pub fn with_mock() -> App {
        App {
            input_value: String::new(),
            messages: vec![
                Message {
                    author_name: "Alfred".to_owned(),
                    content: "Hi I'm Alfred".to_owned(),
                },
                Message {
                    author_name: "Axel".to_owned(),
                    content: "Hi Alfred".to_owned(),
                },
            ],
        }
    }
}
