use kalosm::language::*; // Import everything from kalosm

#[allow(dead_code)]
pub struct ChatbotV1 {
    model: Llama, 
}

impl ChatbotV1 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV1 {
        return ChatbotV1 { 
            model: model 
        };
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, message: String) -> String {
        
        
        let mut chat_session: Chat<Llama> = self.model
            .chat()
            .with_system_prompt("The assistant will act like a friendly alien");
        
        
        let response = chat_session.add_message(message).await; // response is a Result<String, Error>
        
        
        match response { 
            Ok(text) => text.to_string(), //to_string() is used to convert &str to String //text is the response from the model
            Err(e) => format!("Sorry, I encountered an error: {}", e), //err is the error that occurred during the chat session //format! is used to create a formatted string with the error message
        }
    }
}