use crate::MastodonClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

/// Metadata about a Mastodon instance.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Instance {
    /// The domain name of the instance.
    pub uri: String,
    /// The title of the instance.
    pub title: String,
    /// A shorter description of the instance.
    pub description: String,
    /// The administrative contact email for the instance.
    pub email: String,
    /// The version of Mastodon installed on the instance.
    pub version: String,
}

/// Handler for instance-related API endpoints.
pub struct InstanceHandler<'a> {
    client: &'a MastodonClient,
}

impl<'a> InstanceHandler<'a> {
    /// Creates a new `InstanceHandler` for the given client.
    ///
    /// Parameters:
    /// - `client`: The client to use for making requests.
    ///
    /// Returns:
    /// - `InstanceHandler`: The created instance handler.
    pub fn new(client: &'a MastodonClient) -> Self {
        Self { client }
    }

    /// Fetches information about the Mastodon instance.
    ///
    /// Returns:
    /// - `Result<Instance>`: The fetched instance.
    ///
    /// Corresponds to `GET /api/v1/instance`.
    pub async fn get(&self) -> Result<Instance> {
        let url = format!("{}/api/v1/instance", self.client.base_url());
        let req = self.client.http_client().get(&url);
        self.client.send(req).await
    }
}
