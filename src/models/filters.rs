use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Filter {
    pub id: String,
    pub phrase: String,
    pub context: Vec<String>,
    pub expires_at: Option<String>,
    pub irreversible: bool,
    pub whole_word: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct List {
    pub id: String,
    pub title: String,
    pub replies_policy: String,
}
