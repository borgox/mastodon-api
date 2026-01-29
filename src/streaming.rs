use crate::error::{MastodonError, Result};
use crate::models::{Notification, Status};
use futures_util::StreamExt;
use serde::Deserialize;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

/// Raw event received from the Mastodon streaming API.
#[derive(Debug, Deserialize)]
#[serde(tag = "event", content = "payload")]
pub enum StreamEvent {
    /// A new status has been posted.
    #[serde(rename = "update")]
    Update(String), // JSON string of Status
    /// A new notification has been received.
    #[serde(rename = "notification")]
    Notification(String), // JSON string of Notification
    /// A status has been deleted.
    #[serde(rename = "delete")]
    Delete(String), // ID of deleted status
    /// Content filters have changed.
    #[serde(rename = "filters_changed")]
    FiltersChanged,
}

/// A high-level, parsed event from the Mastodon stream.
#[derive(Debug)]
pub enum MastodonEvent {
    /// A new status has been posted.
    Update(Box<Status>),
    /// A new notification (mention, follow, etc.) has been received.
    Notification(Box<Notification>),
    /// a status was deleted (contains the ID).
    Delete(String),
    /// Content filters were updated.
    FiltersChanged,
}

/// Client for connecting to Mastodon's real-time streaming API via WebSockets.
pub struct StreamingClient {
    stream_url: String,
    access_token: Option<String>,
}

impl StreamingClient {
    pub fn new(base_url: &str, access_token: Option<String>) -> Self {
        let stream_url = base_url
            .replace("https://", "wss://")
            .replace("http://", "ws://")
            + "/api/v1/streaming";
        Self {
            stream_url,
            access_token,
        }
    }

    /// Subscribes to a specific stream type.
    ///
    /// # Common Stream Types
    /// - `user`: Events related to the authenticated user.
    /// - `public`: All public statuses.
    /// - `direct`: Direct messages/conversations.
    /// - `hashtag`: Statuses containing a specific tag (requires `tag` parameter in URL usually, but this helper is generic).
    ///
    /// Returns a pinned stream of `MastodonEvent`s.
    pub async fn subscribe(
        &self,
        stream_type: &str,
    ) -> Result<std::pin::Pin<Box<dyn futures_util::Stream<Item = Result<MastodonEvent>> + Send>>>
    {
        let mut url = Url::parse(&self.stream_url)?;
        url.query_pairs_mut().append_pair("stream", stream_type);
        if let Some(token) = &self.access_token {
            url.query_pairs_mut().append_pair("access_token", token);
        }

        let (ws_stream, _) = connect_async(url.to_string())
            .await
            .map_err(|e| MastodonError::Custom(e.to_string()))?;
        let (_, read) = ws_stream.split();

        Ok(Box::pin(read.filter_map(|msg| async {
            match msg {
                Ok(Message::Text(text)) => {
                    let event: StreamEvent = match serde_json::from_str(&text) {
                        Ok(e) => e,
                        Err(_) => return None,
                    };

                    match event {
                        StreamEvent::Update(payload) => {
                            let status: Status = serde_json::from_str(&payload).ok()?;
                            Some(Ok(MastodonEvent::Update(Box::new(status))))
                        }
                        StreamEvent::Notification(payload) => {
                            let notification: Notification = serde_json::from_str(&payload).ok()?;
                            Some(Ok(MastodonEvent::Notification(Box::new(notification))))
                        }
                        StreamEvent::Delete(id) => Some(Ok(MastodonEvent::Delete(id))),
                        StreamEvent::FiltersChanged => Some(Ok(MastodonEvent::FiltersChanged)),
                    }
                }
                _ => None,
            }
        })))
    }
}
