use crate::MastodonClient;
use crate::error::Result;
use crate::models::Filter;
use serde::Serialize;

/// Handler for content filter API endpoints.
pub struct FiltersHandler<'a> {
    client: &'a MastodonClient,
}

/// Parameters for creating a new content filter.
#[derive(Serialize)]
pub struct CreateFilterParams {
    /// The keyword or phrase to filter.
    pub phrase: String,
    /// The contexts in which to apply the filter (e.g., "home", "notifications", "public", "thread").
    pub context: Vec<String>,
    /// Whether the filter should be applied server-side (irreversible) or just hint the client.
    pub irreversible: bool,
    /// Whether to match only whole words.
    pub whole_word: bool,
    /// Number of seconds from now until the filter expires.
    pub expires_in: Option<u32>,
}

impl<'a> FiltersHandler<'a> {
    /// Creates a new `FiltersHandler` for the given client.
    ///
    /// Parameters:
    /// - `client`: The client to use for making requests.
    ///
    /// Returns:
    /// - `FiltersHandler`: The created filters handler.
    pub fn new(client: &'a MastodonClient) -> Self {
        Self { client }
    }

    /// Fetches all content filters for the authenticated user.
    ///
    /// Returns:
    /// - `Result<Vec<Filter>>`: The fetched filters.
    ///
    /// Corresponds to `GET /api/v1/filters`.
    pub async fn list(&self) -> Result<Vec<Filter>> {
        let url = format!("{}/api/v1/filters", self.client.base_url());
        let req = self.client.http_client().get(&url);
        self.client.send(req).await
    }

    /// Creates a new content filter.
    ///
    /// Parameters:
    /// - `params`: The parameters for the filter to create.
    ///
    /// Returns:
    /// - `Result<Filter>`: The created filter.
    ///
    /// Corresponds to `POST /api/v1/filters`.
    pub async fn create(&self, params: &CreateFilterParams) -> Result<Filter> {
        let url = format!("{}/api/v1/filters", self.client.base_url());
        let req = self.client.http_client().post(&url).json(params);
        self.client.send(req).await
    }

    /// Deletes a content filter by its ID.
    ///
    /// Parameters:
    /// - `id`: The ID of the filter to delete.
    ///
    /// Returns:
    /// - `Result<()>`: The deleted filter.
    ///
    /// Corresponds to `DELETE /api/v1/filters/:id`.
    pub async fn delete(&self, id: &str) -> Result<()> {
        let url = format!("{}/api/v1/filters/{}", self.client.base_url(), id);
        let req = self.client.http_client().delete(&url);
        let _: serde_json::Value = self.client.send(req).await?;
        Ok(())
    }
}
