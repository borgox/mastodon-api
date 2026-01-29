use crate::models::{Account, Status};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Conversation {
    pub id: String,
    pub accounts: Vec<Account>,
    pub last_status: Option<Status>,
    pub unread: bool,
}
