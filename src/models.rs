// src/models.rs

// Define ChatMessage structure
#[derive(Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: String,
    pub room_id: String,
    pub user_id: String,
    pub content: String,
    pub timestamp: String,
}

// Define ChatRoom structure
#[derive(Serialize, Deserialize)]
pub struct ChatRoom {
    pub id: String,
    pub name: String,
    pub description: String,
    pub created_at: String,
}