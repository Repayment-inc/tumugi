use std::collections::VecDeque;
use crate::client::ChatMessage;

/// 会話履歴を管理し、システムメッセージを別に保持する構造体
pub struct ConversationBufferWindowMemory {
    messages: VecDeque<ChatMessage>,
    system_message: Option<ChatMessage>,
    window_size: usize,
}

impl ConversationBufferWindowMemory {
    /// 新しいConversationBufferWindowMemoryインスタンスを作成する
    pub fn new(window_size: usize) -> Self {
        Self {
            messages: VecDeque::with_capacity(window_size),
            system_message: None,
            window_size,
        }
    }

    /// メッセージを追加する。システムメッセージの場合は別に保存する
    pub fn add_message(&mut self, message: ChatMessage) {
        match message.role.as_str() {
            "system" => self.system_message = Some(message),
            _ => {
                if self.messages.len() >= self.window_size {
                    self.messages.pop_front();
                }
                self.messages.push_back(message);
            }
        }
    }

    /// 全てのメッセージを取得する。システムメッセージが存在する場合は先頭に追加する
    pub fn get_messages(&self) -> Vec<ChatMessage> {
        let mut result = Vec::with_capacity(self.messages.len() + self.system_message.is_some() as usize);
        if let Some(sys_msg) = &self.system_message {
            result.push(sys_msg.clone());
        }
        result.extend(self.messages.iter().cloned());
        result
    }

    /// システムメッセージを除く全てのメッセージをクリアする
    pub fn clear(&mut self) {
        self.messages.clear();
    }

    /// メッセージの総数を返す（システムメッセージを含む）
    pub fn len(&self) -> usize {
        self.messages.len() + self.system_message.is_some() as usize
    }

    /// メモリが空かどうかを返す（システムメッセージも考慮する）
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty() && self.system_message.is_none()
    }

    /// システムメッセージを設定する
    pub fn set_system_message(&mut self, message: ChatMessage) {
        assert_eq!(message.role, "system", "Message must have 'system' role");
        self.system_message = Some(message);
    }

    /// システムメッセージを取得する
    pub fn get_system_message(&self) -> Option<&ChatMessage> {
        self.system_message.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversation_buffer_window_memory() {
        let mut memory = ConversationBufferWindowMemory::new(3);

        memory.set_system_message(ChatMessage::system("System message".to_string()));
        memory.add_message(ChatMessage::user("User 1".to_string()));
        memory.add_message(ChatMessage::assistant("Assistant 1".to_string()));
        memory.add_message(ChatMessage::user("User 2".to_string()));
        memory.add_message(ChatMessage::assistant("Assistant 2".to_string()));

        let messages = memory.get_messages();
        assert_eq!(messages.len(), 4);  // システムメッセージ + 3つのメッセージ
        assert_eq!(messages[0].content, "System message");
        assert_eq!(messages[1].content, "Assistant 1");
        assert_eq!(messages[2].content, "User 2");
        assert_eq!(messages[3].content, "Assistant 2");

        assert_eq!(memory.len(), 4);
        assert!(!memory.is_empty());

        memory.clear();
        assert_eq!(memory.len(), 1);  // システムメッセージは残る
        assert!(!memory.is_empty());  // システムメッセージがあるので空ではない

        assert!(memory.get_system_message().is_some());
    }

    #[test]
    fn test_conversation_buffer_window_memory_without_system_msg() {
        let mut memory = ConversationBufferWindowMemory::new(3);

        memory.add_message(ChatMessage::user("User 1".to_string()));
        memory.add_message(ChatMessage::assistant("Assistant 1".to_string()));
        memory.add_message(ChatMessage::user("User 2".to_string()));
        memory.add_message(ChatMessage::assistant("Assistant 2".to_string()));

        let messages = memory.get_messages();
        assert_eq!(messages.len(), 3);  // 3つのメッセージ
        assert_eq!(messages[0].content, "Assistant 1");
        assert_eq!(messages[1].content, "User 2");
        assert_eq!(messages[2].content, "Assistant 2");

        assert_eq!(memory.len(), 3);
        assert!(!memory.is_empty());

        memory.clear();
        assert_eq!(memory.len(), 0);  // メモリは空になる
        assert!(memory.is_empty());  // システムメッセージがあるので空ではない

        assert!(memory.get_system_message().is_none());
    }
}