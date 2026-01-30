use crate::MastodonClient;
use crate::error::Result;
use crate::models::{FeaturedTag, Tag};

/// Handler for tag-related API endpoints (followed and featured tags).
pub struct TagsHandler<'a> {
    client: &'a MastodonClient,
}

impl<'a> TagsHandler<'a> {
    /// Creates a new `TagsHandler` for the given client.
    pub fn new(client: &'a MastodonClient) -> Self {
        Self { client }
    }

    /// Fetches all tags that the authenticated user is following.
    ///
    /// Returns:
    /// - `Result<Vec<Tag>>`: The followed tags.
    ///
    /// Corresponds to `GET /api/v1/followed_tags`.
    pub async fn list_followed(&self) -> Result<Vec<Tag>> {
        let url = format!("{}/api/v1/followed_tags", self.client.base_url());
        let req = self.client.http_client().get(&url);
        self.client.send(req).await
    }

    /// Follows a tag.
    ///
    /// Parameters:
    /// - `name`: The name of the tag to follow.
    ///
    /// Returns:
    /// - `Result<Tag>`: The followed tag.
    ///
    /// Corresponds to `POST /api/v1/tags/:name/follow`.
    pub async fn follow(&self, name: &str) -> Result<Tag> {
        let url = format!("{}/api/v1/tags/{}/follow", self.client.base_url(), name);
        let req = self.client.http_client().post(&url);
        self.client.send(req).await
    }

    /// Unfollows a tag.
    ///
    /// Parameters:
    /// - `name`: The name of the tag to unfollow.
    ///
    /// Returns:
    /// - `Result<Tag>`: The unfollowed tag.
    ///
    /// Corresponds to `POST /api/v1/tags/{name}/unfollow`.
    pub async fn unfollow(&self, name: &str) -> Result<Tag> {
        let url = format!("{}/api/v1/tags/{}/unfollow", self.client.base_url(), name);
        let req = self.client.http_client().post(&url);
        self.client.send(req).await
    }

    /// Fetches all tags featured on the authenticated user's profile.
    ///
    /// Returns:
    /// - `Result<Vec<FeaturedTag>>`: The featured tags.
    ///
    /// Corresponds to `GET /api/v1/featured_tags`.
    pub async fn list_featured(&self) -> Result<Vec<FeaturedTag>> {
        let url = format!("{}/api/v1/featured_tags", self.client.base_url());
        let req = self.client.http_client().get(&url);
        self.client.send(req).await
    }

    /// Features a tag on the authenticated user's profile.
    ///
    /// Parameters:
    /// - `name`: The name of the tag to feature.
    ///
    /// Returns:
    /// - `Result<FeaturedTag>`: The featured tag.
    ///
    /// Corresponds to `POST /api/v1/featured_tags`.
    pub async fn feature(&self, name: &str) -> Result<FeaturedTag> {
        let url = format!("{}/api/v1/featured_tags", self.client.base_url());
        let req = self.client.http_client().post(&url).form(&[("name", name)]);
        self.client.send(req).await
    }

    /// Unfeatures a tag from the profile.
    ///
    /// Parameters:
    /// - `id`: The ID of the featured tag.
    ///
    /// Returns:
    /// - `Result<()>`: Success if the tag was unfeatured.
    ///
    /// Corresponds to `DELETE /api/v1/featured_tags/:id`.
    pub async fn unfeature(&self, id: &str) -> Result<()> {
        let url = format!("{}/api/v1/featured_tags/{}", self.client.base_url(), id);
        let req = self.client.http_client().delete(&url);
        self.client.send(req).await
    }

    /// Fetches suggestions for tags to feature.
    ///
    /// Returns:
    /// - `Result<Vec<Tag>>`: The suggested tags.
    ///
    /// Corresponds to `GET /api/v1/featured_tags/suggestions`.
    pub async fn featured_suggestions(&self) -> Result<Vec<Tag>> {
        let url = format!(
            "{}/api/v1/featured_tags/suggestions",
            self.client.base_url()
        );
        let req = self.client.http_client().get(&url);
        self.client.send(req).await
    }
}
