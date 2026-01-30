use crate::models::Account;
use serde::{Deserialize, Serialize};

/// Represents a report of a status or account for rule violations.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Report {
    /// The ID of the report.
    pub id: String,
    /// Whether action has been taken by moderators.
    pub action_taken: bool,
    /// When the action was taken (ISO 8601).
    pub action_taken_at: Option<String>,
    /// The category of the report (e.g., spam, violation).
    pub category: String,
    /// The comment provided by the reporter.
    pub comment: String,
    /// Whether the report was forwarded to the remote instance.
    pub forwarded: bool,
    /// When the report was created (ISO 8601).
    pub created_at: String,
    /// IDs of statuses that were reported.
    pub status_ids: Option<Vec<String>>,
    /// IDs of rules that were violated.
    pub rule_ids: Option<Vec<String>>,
    /// The account that was reported.
    pub target_account: Account,
}
