use crate::MastodonClient;
use crate::error::Result;
use crate::models::announcement::{Announcement, AnnouncementReaction};

/// Handler for announcement-related API endpoints.
pub struct AnnouncementsHandler<'a> {
    client: &'a MastodonClient,
}

impl<'a> AnnouncementsHandler<'a> {
    /// Creates a new `AnnouncementsHandler` for the given client.
    pub fn new(client: &'a MastodonClient) -> Self {
        Self { client }
    }

    /// Fetches all active announcements from the server.
    ///
    /// Returns:
    /// - `Result<Vec<Announcement>>`: The announcements.
    ///
    /// Corresponds to `GET /api/v1/announcements`.
    pub async fn list(&self) -> Result<Vec<Announcement>> {
        let url = format!("{}/api/v1/announcements", self.client.base_url());
        let req = self.client.http_client().get(&url);
        self.client.send(req).await
    }

    /// Dismisses an announcement for the authenticated user.
    ///
    /// Parameters:
    /// - `id`: The ID of the announcement to dismiss.
    ///
    /// Returns:
    /// - `Result<()>`: Success if the announcement was dismissed.
    ///
    /// Corresponds to `POST /api/v1/announcements/:id/dismiss`.
    pub async fn dismiss(&self, id: &str) -> Result<()> {
        let url = format!(
            "{}/api/v1/announcements/{}/dismiss",
            self.client.base_url(),
            id
        );
        let req = self.client.http_client().post(&url);
        // Mastodon returns 200 OK with empty body for this endpoint.
        // MastodonClient::send expects T: DeserializeOwned, so we might need a workaround for empty bodies
        // if send is strictly requiring a JSON body.
        // Let's check MastodonClient::send in src/lib.rs.
        self.client.send(req).await
    }

    /// Adds a reaction to an announcement.
    ///
    /// Parameters:
    /// - `id`: The ID of the announcement.
    /// - `name`: The name of the reaction (emoji).
    ///
    /// Returns:
    /// - `Result<AnnouncementReaction>`: The updated reaction.
    ///
    /// Corresponds to `POST /api/v1/announcements/:id/reactions/:name`.
    pub async fn add_reaction(&self, id: &str, name: &str) -> Result<AnnouncementReaction> {
        let url = format!(
            "{}/api/v1/announcements/{}/reactions/{}",
            self.client.base_url(),
            id,
            name
        );
        let req = self.client.http_client().post(&url);
        self.client.send(req).await
    }

    /// Removes a reaction from an announcement.
    ///
    /// Parameters:
    /// - `id`: The ID of the announcement.
    /// - `name`: The name of the reaction (emoji).
    ///
    /// Returns:
    /// - `Result<AnnouncementReaction>`: The updated reaction.
    ///
    /// Corresponds to `DELETE /api/v1/announcements/:id/reactions/:name`.
    pub async fn remove_reaction(&self, id: &str, name: &str) -> Result<AnnouncementReaction> {
        let url = format!(
            "{}/api/v1/announcements/{}/reactions/{}",
            self.client.base_url(),
            id,
            name
        );
        let req = self.client.http_client().delete(&url);
        self.client.send(req).await
    }
}
