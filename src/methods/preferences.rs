use crate::MastodonClient;
use crate::error::Result;
use crate::models::preferences::Preferences;

/// Handler for preference-related API endpoints.
pub struct PreferencesHandler<'a> {
    client: &'a MastodonClient,
}

impl<'a> PreferencesHandler<'a> {
    /// Creates a new `PreferencesHandler` for the given client.
    pub fn new(client: &'a MastodonClient) -> Self {
        Self { client }
    }

    /// Fetches the authenticated user's preferences.
    ///
    /// Returns:
    /// - `Result<Preferences>`: The user's preferences.
    ///
    /// Corresponds to `GET /api/v1/preferences`.
    pub async fn get(&self) -> Result<Preferences> {
        let url = format!("{}/api/v1/preferences", self.client.base_url());
        let req = self.client.http_client().get(&url);
        self.client.send(req).await
    }
}
