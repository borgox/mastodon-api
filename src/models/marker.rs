use serde::{Deserialize, Serialize};

/// Represents a marker for a reading position in a timeline.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Marker {
    /// The ID of the last read status in the timeline.
    pub last_read_id: String,
    /// The version of the marker (incremental).
    pub version: u32,
    /// When the marker was last updated (ISO 8601).
    pub updated_at: String,
}
