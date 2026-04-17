use kalosm::language::*;
use crate::solution::file_library;
use std::collections::HashMap;

pub struct ChatbotV4 {
    model: Llama,
    user_sessions: HashMap<String, Chat<Llama>>,
}

impl ChatbotV4 {
    pub fn new(model: Llama) -> ChatbotV4 {
        ChatbotV4 {
            model,
            user_sessions: HashMap::new(),
        }
    }

    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        let filename = format!("{}.txt", username);

        let chat_session = if let Some(session) = self.user_sessions.get_mut(&username) {
            session
        } else {
            match file_library::load_chat_session_from_file(&filename) {
                Some(loaded_session) => {
                    let chat = self.model
                        .chat()
                        .with_session(loaded_session);
                    self.user_sessions.insert(username.clone(), chat);
                    self.user_sessions.get_mut(&username).unwrap()
                }
                None => {
                    let new_session = self.model
                        .chat()
                        .with_system_prompt("The assistant will act like a pirate");
                    self.user_sessions.insert(username.clone(), new_session);
                    self.user_sessions.get_mut(&username).unwrap()
                }
            }
        };
        
        match chat_session.add_message(message).await {
            Ok(text) => {
                let response = text.to_string();
                if let Err(e) = self.save_chat_session_to_file(&username) {
                    eprintln!("Error saving session for {}: {}", username, e);
                }
                response
            }
            Err(e) => format!("Sorry, I encountered an error: {}", e),
        }
    }

    pub fn save_chat_session_to_file(&self, username: &str) -> Result<(), Box<dyn std::error::Error + '_>> {
        let filename = format!("{}.txt", username);
        
        if let Some(chat) = self.user_sessions.get(username) {
            let session_data = chat.session()?;
            file_library::save_chat_session_to_file(&filename, &session_data)?;
        }
        
        Ok(())
    }

    pub fn get_history(&self, username: String) -> Vec<String> {
    let filename = format!("{}.txt", username);

    match file_library::load_chat_session_from_file(&filename) {
        None => Vec::new(),
        Some(session) => {
            let mut messages: Vec<String> = Vec::new();

            for msg in session.history() {
                if msg.role() != MessageType::SystemPrompt {
                    messages.push(msg.content().to_string());
                }
            }

            messages
        }
    }
}
}

