use kalosm::language::*;

#[allow(dead_code)]
pub struct ChatbotV1 {
    model: Llama,
}

impl ChatbotV1 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV1 {
        return ChatbotV1 { model: model };
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, message: String) -> String {
        
        let mut chat_session: Chat<Llama> = self.model 
            .chat() 
            .with_system_prompt("The assistant will act like an angsty teenager"); 

        
        let response = chat_session.add_message(message).await; 
        
        
        match response { 
            Ok(text) => text.to_string(),
            Err(e) => format!("Sorry, I encountered an error: {}", e),
        }
    }
}