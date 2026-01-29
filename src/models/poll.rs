use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Poll {
    pub id: String,
    pub expires_at: Option<String>,
    pub expired: bool,
    pub multiple: bool,
    pub votes_count: u64,
    pub voters_count: Option<u64>,
    pub options: Vec<PollOption>,
    pub emojis: Vec<CustomEmoji>,
    pub voted: Option<bool>,
    pub own_votes: Option<Vec<u32>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PollOption {
    pub title: String,
    pub votes_count: Option<u64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CustomEmoji {
    pub shortcode: String,
    pub url: String,
    pub static_url: String,
    pub visible_in_picker: bool,
    pub category: Option<String>,
}
