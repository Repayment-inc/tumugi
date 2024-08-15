
use crate::client::ChatMessage;
use std::collections::VecDeque;

pub struct ConversationBufferWindowMemory {
    messages: VecDeque<ChatMessage>,
    window_size: usize,
}

impl ConversationBufferWindowMemory {
    pub fn new(window_size: usize) -> Self {
        Self {
            messages: VecDeque::new(),
            window_size,
        }
    }

    pub fn add_message(&mut self, message: ChatMessage) {
        if self.messages.len() >= self.window_size {
            self.messages.pop_front();
        }
        self.messages.push_back(message);
    }

    pub fn get_messages(&self) -> Vec<ChatMessage> {
        self.messages.iter().cloned().collect()
    }

    pub fn clear(&mut self) {
        self.messages.clear();
    }

    pub fn len(&self) -> usize {
        self.messages.len()
    }

    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversation_buffer_window_memory() {
        let mut memory = ConversationBufferWindowMemory::new(3);

        memory.add_message(ChatMessage::user("Hello".to_string()));
        memory.add_message(ChatMessage::assistant("Hi there!".to_string()));
        memory.add_message(ChatMessage::user("How are you?".to_string()));

        assert_eq!(memory.len(), 3);

        memory.add_message(ChatMessage::assistant("I'm doing well, thanks!".to_string()));

        assert_eq!(memory.len(), 3);
        let messages = memory.get_messages();
        assert_eq!(messages[0].content, "Hi there!");
        assert_eq!(messages[1].content, "How are you?");
        assert_eq!(messages[2].content, "I'm doing well, thanks!");

        memory.clear();
        assert!(memory.is_empty());
    }
}