use crate::models::{Account, Status};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Notification {
    pub id: String,
    pub r#type: String,
    pub created_at: String,
    pub account: Account,
    pub status: Option<Status>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Tag {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Mention {
    pub id: String,
    pub username: String,
    pub url: String,
    pub acct: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Relationship {
    pub id: String,
    pub following: bool,
    pub followed_by: bool,
    pub blocking: bool,
    pub blocked_by: bool,
    pub muting: bool,
    pub muting_notifications: bool,
    pub requested: bool,
    pub domain_blocking: bool,
    pub endorsed: bool,
    pub note: String,
}
