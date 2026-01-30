use crate::MastodonClient;
use crate::error::Result;
use crate::models::{Account, Activity, Instance, Rule};

/// Handler for instance-related API endpoints.
pub struct InstanceHandler<'a> {
    client: &'a MastodonClient,
}

impl<'a> InstanceHandler<'a> {
    /// Creates a new `InstanceHandler` for the given client.
    pub fn new(client: &'a MastodonClient) -> Self {
        Self { client }
    }

    /// Fetches metadata about the Mastodon instance.
    ///
    /// Returns:
    /// - `Result<Instance>`: The instance metadata.
    ///
    /// Corresponds to `GET /api/v1/instance`.
    pub async fn get(&self) -> Result<Instance> {
        let url = format!("{}/api/v1/instance", self.client.base_url());
        let req = self.client.http_client().get(&url);
        self.client.send(req).await
    }

    /// Fetches the list of domains this instance is aware of.
    ///
    /// Returns:
    /// - `Result<Vec<String>>`: A list of domain names.
    ///
    /// Corresponds to `GET /api/v1/instance/peers`.
    pub async fn peers(&self) -> Result<Vec<String>> {
        let url = format!("{}/api/v1/instance/peers", self.client.base_url());
        let req = self.client.http_client().get(&url);
        self.client.send(req).await
    }

    /// Fetches weekly usage statistics for the instance.
    ///
    /// Returns:
    /// - `Result<Vec<Activity>>`: A list of weekly activity statistics.
    ///
    /// Corresponds to `GET /api/v1/instance/activity`.
    pub async fn activity(&self) -> Result<Vec<Activity>> {
        let url = format!("{}/api/v1/instance/activity", self.client.base_url());
        let req = self.client.http_client().get(&url);
        self.client.send(req).await
    }

    /// Fetches the formal rules established by the server.
    ///
    /// Returns:
    /// - `Result<Vec<Rule>>`: A list of instance rules.
    ///
    /// Corresponds to `GET /api/v1/instance/rules`.
    pub async fn rules(&self) -> Result<Vec<Rule>> {
        let url = format!("{}/api/v1/instance/rules", self.client.base_url());
        let req = self.client.http_client().get(&url);
        self.client.send(req).await
    }

    /// Fetches the instance-level user directory.
    ///
    /// Returns:
    /// - `Result<Vec<Account>>`: A list of accounts in the directory.
    ///
    /// Corresponds to `GET /api/v1/directories`.
    pub async fn directories(&self) -> Result<Vec<Account>> {
        let url = format!("{}/api/v1/directories", self.client.base_url());
        let req = self.client.http_client().get(&url);
        self.client.send(req).await
    }
}
