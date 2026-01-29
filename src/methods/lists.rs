use crate::MastodonClient;
use crate::error::Result;
use crate::models::List;

/// Handler for list-related API endpoints.
pub struct ListsHandler<'a> {
    client: &'a MastodonClient,
}

impl<'a> ListsHandler<'a> {
    /// Creates a new `ListsHandler` for the given client.
    ///
    /// Parameters:
    /// - `client`: The client to use for making requests.
    ///
    /// Returns:
    /// - `ListsHandler`: The created lists handler.
    pub fn new(client: &'a MastodonClient) -> Self {
        Self { client }
    }

    /// Fetches all lists for the authenticated user.
    ///
    /// Returns:
    /// - `Result<Vec<List>>`: The fetched lists.
    ///
    /// Corresponds to `GET /api/v1/lists`.
    pub async fn list(&self) -> Result<Vec<List>> {
        let url = format!("{}/api/v1/lists", self.client.base_url());
        let req = self.client.http_client().get(&url);
        self.client.send(req).await
    }

    /// Fetches a specific list by its ID.
    ///
    /// Parameters:
    /// - `id`: The ID of the list to fetch.
    ///
    /// Returns:
    /// - `Result<List>`: The fetched list.
    ///
    /// Corresponds to `GET /api/v1/lists/:id`.
    pub async fn get(&self, id: &str) -> Result<List> {
        let url = format!("{}/api/v1/lists/{}", self.client.base_url(), id);
        let req = self.client.http_client().get(&url);
        self.client.send(req).await
    }

    /// Creates a new list with the given title.
    ///
    /// Parameters:
    /// - `title`: The title of the list to create.
    ///
    /// Returns:
    /// - `Result<List>`: The created list.
    ///
    /// Corresponds to `POST /api/v1/lists`.
    pub async fn create(&self, title: &str) -> Result<List> {
        let url = format!("{}/api/v1/lists", self.client.base_url());
        let req = self
            .client
            .http_client()
            .post(&url)
            .json(&serde_json::json!({ "title": title }));
        self.client.send(req).await
    }

    /// Deletes a list by its ID.
    ///
    /// Parameters:
    /// - `id`: The ID of the list to delete.
    ///
    /// Returns:
    /// - `Result<()>`: The deleted list.
    ///
    /// Corresponds to `DELETE /api/v1/lists/:id`.
    pub async fn delete(&self, id: &str) -> Result<()> {
        let url = format!("{}/api/v1/lists/{}", self.client.base_url(), id);
        let req = self.client.http_client().delete(&url);
        let _: serde_json::Value = self.client.send(req).await?;
        Ok(())
    }

    /// Adds accounts to a list.
    ///
    /// Parameters:
    /// - `list_id`: The ID of the list to add accounts to.
    /// - `account_ids`: The IDs of the accounts to add to the list.
    ///
    /// Returns:
    /// - `Result<()>`: The added accounts.
    ///
    /// Corresponds to `POST /api/v1/lists/:id/accounts`.
    pub async fn add_accounts(&self, list_id: &str, account_ids: &[String]) -> Result<()> {
        let url = format!(
            "{}/api/v1/lists/{}/accounts",
            self.client.base_url(),
            list_id
        );
        let req = self
            .client
            .http_client()
            .post(&url)
            .json(&serde_json::json!({ "account_ids": account_ids }));
        let _: serde_json::Value = self.client.send(req).await?;
        Ok(())
    }

    /// Removes accounts from a list.
    ///
    /// Parameters:
    /// - `list_id`: The ID of the list to remove accounts from.
    /// - `account_ids`: The IDs of the accounts to remove from the list.
    ///
    /// Returns:
    /// - `Result<()>`: The removed accounts.
    ///
    /// Corresponds to `DELETE /api/v1/lists/:id/accounts`.
    pub async fn remove_accounts(&self, list_id: &str, account_ids: &[String]) -> Result<()> {
        let url = format!(
            "{}/api/v1/lists/{}/accounts",
            self.client.base_url(),
            list_id
        );
        let req = self
            .client
            .http_client()
            .delete(&url)
            .json(&serde_json::json!({ "account_ids": account_ids }));
        let _: serde_json::Value = self.client.send(req).await?;
        Ok(())
    }
}
