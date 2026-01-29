use crate::MastodonClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

/// Handler for application-related API endpoints.
pub struct AppsHandler<'a> {
    client: &'a MastodonClient,
}

/// Parameters for registering a new application.
#[derive(Serialize)]
pub struct RegisterAppParams {
    /// The name of your application.
    pub client_name: String,
    /// Where to redirect the user after authorization. Use `urn:ietf:wg:oauth:2.0:oob` for local apps.
    pub redirect_uris: String,
    /// Space-separated list of scopes (e.g., "read write follow").
    pub scopes: String,
    /// The website of your application.
    pub website: Option<String>,
}

/// Response from a successful application registration.
#[derive(Deserialize)]
pub struct AppRegistration {
    pub id: String,
    pub name: String,
    pub website: Option<String>,
    pub redirect_uri: String,
    pub client_id: String,
    pub client_secret: String,
    pub vapid_key: Option<String>,
}

impl<'a> AppsHandler<'a> {
    pub fn new(client: &'a MastodonClient) -> Self {
        Self { client }
    }

    /// Registers a new application with the Mastodon instance.
    ///
    /// Parameters:
    /// - `params`: The parameters for the application to register.
    ///
    /// Returns:
    /// - `Result<AppRegistration>`: The registered application.
    ///
    /// Corresponds to `POST /api/v1/apps`.
    pub async fn register(&self, params: &RegisterAppParams) -> Result<AppRegistration> {
        let url = format!("{}/api/v1/apps", self.client.base_url());
        let req = self.client.http_client().post(&url).json(params);
        self.client.send(req).await
    }
}
