use crate::models::Account;
use serde::{Deserialize, Serialize};

/// Represents a suggested account to follow.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Suggestion {
    /// The reason why this account is being suggested.
    pub source: String,
    /// The account being suggested.
    pub account: Account,
}
