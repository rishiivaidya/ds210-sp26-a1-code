use kalosm::language::*;
use std::collections::HashMap; //we need a hashmap to store username -> chat session

#[allow(dead_code)]
pub struct ChatbotV3 {
    model: Llama,
    chats: HashMap<String, Chat<Llama>> //stores one chat session per user
}

impl ChatbotV3 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV3 {
        //creates new chatbotV3 instance, stores Llama model, inits empty hashmap
        return ChatbotV3 {
            model: model,
            chats: HashMap::new(),
            // Make sure you initialize your struct members here
        };
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        
        if !self.chats.contains_key(&username) { //check if user already has chat session
            let chat_session = self //create new chat session
            .model
            .chat()
            .with_system_prompt("The assistant will act like an angsty teenager");

        self.chats.insert(username.clone(), chat_session); //store in hashmap
        }
        
        let chat = self.chats.get_mut(&username).unwrap(); //retrieve existing chat session
        let response = chat.add_message(message).await; //send to LLM

        match response { //same code from V1
            Ok(text) => return text.to_string(),
            Err(e) => return format!("Sorry, I encountered an error: {}", e),
        }
    }

    #[allow(dead_code)]
    pub fn get_history(&self, username: String) -> Vec<String> {
        
        if let Some(chat) = self.chats.get(&username) { //check if chat exists

            let session = chat.session().unwrap(); //get chat session

            let history = session.history(); //get msg history

            let mut messages: Vec<String> = Vec::new(); //create vec to store message contents as Strings

            for msg in history {
                if msg.role() != MessageType::SystemPrompt { //fix for error that swapped user and
                    //chatbot????
                    messages.push(msg.content().to_string()); //store text contents of each msg
                }
                
            }

            return messages;

        }

        return Vec::new(); //if no chat history, empty vec

    }
}