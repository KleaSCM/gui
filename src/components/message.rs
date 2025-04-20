#[derive(Debug, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
    pub timestamp: chrono::DateTime<chrono::Local>,
    pub status: MessageStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MessageStatus {
    Sent,
    Delivered,
}

impl Default for Message {
    fn default() -> Self {
        Self {
            role: String::new(),
            content: String::new(),
            timestamp: chrono::Local::now(),
            status: MessageStatus::Sent,
        }
    }
} 