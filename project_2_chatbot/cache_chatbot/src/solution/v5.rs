use kalosm::language::*;
use file_chatbot::solution::file_library;

use crate::solution::Cache;

pub struct ChatbotV5 {
    model: Llama,
    cache: Cache<Chat<Llama>>,
}

impl ChatbotV5 {
    pub fn new(model: Llama) -> ChatbotV5 {
        return ChatbotV5 {
            model: model,
            cache: Cache::new(3),
        };
    }

    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
    let filename = format!("{}.txt", username);

    if self.cache.get_chat(&username).is_none() {
        println!("chat_with_user: {username} is not in the cache!");

        let new_chat = match file_library::load_chat_session_from_file(&filename) {
            Some(session) => {
                self.model.chat().with_session(session)
            }
            None => {
                self.model
                    .chat()
                    .with_system_prompt("The assistant will act like an angsty teenager")
            }
        };

        self.cache.insert_chat(username.clone(), new_chat);
    } else {
        println!("chat_with_user: {username} is in the cache! Nice!");
    }

    let chat_session = self.cache.get_chat(&username).unwrap();

    match chat_session.add_message(message).await {
        Ok(text) => {
            let session = chat_session.session().unwrap();
            file_library::save_chat_session_to_file(&filename, &session);
            text.to_string()
        }
        Err(e) => format!("I encountered an error: {}", e),
    }
}

    pub fn get_history(&mut self, username: String) -> Vec<String> {
    let filename = &format!("{}.txt", username);
    let cached_chat = self.cache.get_chat(&username);

    match cached_chat {
        None => {
            println!("get_history: {username} is not in the cache!");

            let new_chat = match file_library::load_chat_session_from_file(filename) {
                Some(session) => {
                    self.model.chat().with_session(session)
                }
                None => {
                    self.model
                        .chat()
                        .with_system_prompt("The assistant will act like an angsty teenager")
                }
            };

            self.cache.insert_chat(username.clone(), new_chat);

            let chat_session = self.cache.get_chat(&username).unwrap();
            let session = chat_session.session().unwrap();
            let history = session.history();

            let mut messages: Vec<String> = Vec::new();

            for msg in history {
                if msg.role() != MessageType::SystemPrompt {
                    messages.push(msg.content().to_string());
                }
            }

            return messages;
        }

        Some(chat_session) => {
            println!("get_history: {username} is in the cache! Nice!");

            let session = chat_session.session().unwrap();
            let history = session.history();

            let mut messages: Vec<String> = Vec::new();

            for msg in history {
                if msg.role() != MessageType::SystemPrompt {
                    messages.push(msg.content().to_string());
                }
            }

            return messages;
        }
    }
}
}