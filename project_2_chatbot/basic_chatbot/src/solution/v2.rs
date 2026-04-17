use kalosm::language::*;

#[allow(dead_code)]
pub struct ChatbotV2 {
    model: Llama,                    // Store model to create sessions 
    
    chat_session: Option<Chat<Llama>>, // Store session to maintain history // option is used because we don't have a session until the first message is received }
    // the difference between this and v1 is that we store the session as part of the struct, so we can maintain history across multiple messages. In v1, we created a new session for each message, so there was no history.
    
}
impl ChatbotV2 { // impl block to define methods for ChatbotV2
    #[allow(dead_code)]  
    pub fn new(model: Llama) -> ChatbotV2 { // Create a new chatbot instance with the given model //  return a new instance of ChatbotV2 with the model and no session
        return ChatbotV2 { 
            model: model,              // Store model 
            chat_session: None,         // Session starts as None, will be created when the first message is received
        };
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, message: String) -> String { // Check if we already have a chat session, if not, create one
        
        if self.chat_session.is_none() { // If there is no existing chat session, create a new one and stores it in the struct for future use
            self.chat_session = Some( //some is used to create an Option that contains a value, here that is the new chat session. This allows us to maintain the chat session across multiple messages.
                self.model// Use the stored model to create a new chat session
                    .chat() // Start a new chat session
                    .with_system_prompt("The assistant will act like a angry customer") // Set a system prompt for the chat session
            );
        }
        
        // since we have already checked that the chat session is not None, we can safely unwrap it to get a mutable reference to the chat session and add messages to it. This allows us to maintain the history of the conversation across multiple messages.
        let chat_session = self.chat_session.as_mut().unwrap(); 
            
            match chat_session.add_message(message).await {
            Ok(text) => text.to_string(),
            Err(e) => format!("Sorry, I encountered an error: {}", e), 
        }
    }
}
