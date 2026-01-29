use crate::MastodonClient;
use crate::error::Result;
use crate::models::Conversation;

/// Handler for direct conversation (DM) API endpoints.
pub struct ConversationsHandler<'a> {
    client: &'a MastodonClient,
}

impl<'a> ConversationsHandler<'a> {
    /// Creates a new `ConversationsHandler` for the given client.
    ///
    /// Parameters:
    /// - `client`: The client to use for making requests.
    ///
    /// Returns:
    /// - `ConversationsHandler`: The created conversations handler.
    pub fn new(client: &'a MastodonClient) -> Self {
        Self { client }
    }

    /// Fetches a list of the authenticated user's direct conversations.
    ///
    /// Returns:
    /// - `Result<Vec<Conversation>>`: The fetched conversations.
    ///
    /// Corresponds to `GET /api/v1/conversations`.
    pub async fn list(&self) -> Result<Vec<Conversation>> {
        let url = format!("{}/api/v1/conversations", self.client.base_url());
        let req = self.client.http_client().get(&url);
        self.client.send(req).await
    }

    /// Deletes a conversation by its ID.
    ///
    /// Parameters:
    /// - `id`: The ID of the conversation to delete.
    ///
    /// Returns:
    /// - `Result<()>`: The deleted conversation.
    ///
    /// Corresponds to `DELETE /api/v1/conversations/:id`.
    pub async fn delete(&self, id: &str) -> Result<()> {
        let url = format!("{}/api/v1/conversations/{}", self.client.base_url(), id);
        let req = self.client.http_client().delete(&url);
        let _: serde_json::Value = self.client.send(req).await?;
        Ok(())
    }

    /// Marks a conversation as read.
    ///
    /// Parameters:
    /// - `id`: The ID of the conversation to mark as read.
    ///
    /// Returns:
    /// - `Result<Conversation>`: The marked conversation.
    ///
    /// Corresponds to `POST /api/v1/conversations/:id/read`.
    pub async fn mark_as_read(&self, id: &str) -> Result<Conversation> {
        let url = format!(
            "{}/api/v1/conversations/{}/read",
            self.client.base_url(),
            id
        );
        let req = self.client.http_client().post(&url);
        self.client.send(req).await
    }
}
