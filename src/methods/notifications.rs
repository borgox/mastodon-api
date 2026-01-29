use crate::MastodonClient;
use crate::error::Result;
use crate::models::Notification;

/// Handler for notification-related API endpoints.
pub struct NotificationsHandler<'a> {
    client: &'a MastodonClient,
}

impl<'a> NotificationsHandler<'a> {
    // Creates a new `NotificationsHandler` for the given client.
    //
    // Parameters:
    // - `client`: The client to use for making requests.
    //
    // Returns:
    // - `NotificationsHandler`: The created notifications handler.
    pub fn new(client: &'a MastodonClient) -> Self {
        Self { client }
    }

    /// Fetches all notifications for the authenticated user.
    ///
    /// Returns:
    /// - `Result<Vec<Notification>>`: The fetched notifications.
    ///
    /// Corresponds to `GET /api/v1/notifications`.
    pub async fn list(&self) -> Result<Vec<Notification>> {
        let url = format!("{}/api/v1/notifications", self.client.base_url());
        let req = self.client.http_client().get(&url);
        self.client.send(req).await
    }

    /// Fetches a specific notification by its ID.
    ///
    /// Parameters:
    /// - `id`: The ID of the notification to fetch.
    ///
    /// Returns:
    /// - `Result<Notification>`: The fetched notification.
    ///
    /// Corresponds to `GET /api/v1/notifications/:id`.
    pub async fn get(&self, id: &str) -> Result<Notification> {
        let url = format!("{}/api/v1/notifications/{}", self.client.base_url(), id);
        let req = self.client.http_client().get(&url);
        self.client.send(req).await
    }

    /// Clears all notifications for the authenticated user.
    ///
    /// Returns:
    /// - `Result<()>`: The cleared notifications.
    ///
    /// Corresponds to `POST /api/v1/notifications/clear`.
    pub async fn clear(&self) -> Result<()> {
        let url = format!("{}/api/v1/notifications/clear", self.client.base_url());
        let req = self.client.http_client().post(&url);
        let _: serde_json::Value = self.client.send(req).await?;
        Ok(())
    }

    /// Dismisses a single notification by its ID.
    ///
    /// Parameters:
    /// - `id`: The ID of the notification to dismiss.
    ///
    /// Returns:
    /// - `Result<()>`: The dismissed notification.
    ///
    /// Corresponds to `POST /api/v1/notifications/:id/dismiss`.
    pub async fn dismiss(&self, id: &str) -> Result<()> {
        let url = format!(
            "{}/api/v1/notifications/{}/dismiss",
            self.client.base_url(),
            id
        );
        let req = self.client.http_client().post(&url);
        let _: serde_json::Value = self.client.send(req).await?;
        Ok(())
    }
}
