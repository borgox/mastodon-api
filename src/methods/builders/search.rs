use crate::MastodonClient;
use crate::error::Result;
use crate::models::Search;

/// A builder for search requests.
pub struct SearchBuilder<'a> {
    client: &'a MastodonClient,
    query: String,
    r#type: Option<String>,
    resolve: Option<bool>,
    limit: Option<u32>,
    offset: Option<u32>,
    account_id: Option<String>,
    min_id: Option<String>,
    max_id: Option<String>,
}

impl<'a> SearchBuilder<'a> {
    /// Creates a new `SearchBuilder` with the given query.
    pub fn new(client: &'a MastodonClient, query: &str) -> Self {
        Self {
            client,
            query: query.to_string(),
            r#type: None,
            resolve: None,
            limit: None,
            offset: None,
            account_id: None,
            min_id: None,
            max_id: None,
        }
    }

    /// Sets the type of results to return (accounts, hashtags, statuses).
    pub fn r#type(mut self, value: &str) -> Self {
        self.r#type = Some(value.to_string());
        self
    }

    /// Whether to resolve accounts and hashtags not known to the local instance.
    pub fn resolve(mut self, value: bool) -> Self {
        self.resolve = Some(value);
        self
    }

    /// Maximum number of results to return.
    pub fn limit(mut self, value: u32) -> Self {
        self.limit = Some(value);
        self
    }

    /// Number of results to skip.
    pub fn offset(mut self, value: u32) -> Self {
        self.offset = Some(value);
        self
    }

    /// Search only for statuses by this account.
    pub fn account_id(mut self, value: &str) -> Self {
        self.account_id = Some(value.to_string());
        self
    }

    /// Minimum status ID.
    pub fn min_id(mut self, value: &str) -> Self {
        self.min_id = Some(value.to_string());
        self
    }

    /// Maximum status ID.
    pub fn max_id(mut self, value: &str) -> Self {
        self.max_id = Some(value.to_string());
        self
    }

    /// Executes the search request (v2).
    pub async fn send(self) -> Result<Search> {
        let url = format!("{}/api/v2/search", self.client.base_url());
        let mut req = self
            .client
            .http_client()
            .get(&url)
            .query(&[("q", &self.query)]);

        if let Some(t) = self.r#type {
            req = req.query(&[("type", t)]);
        }
        if let Some(r) = self.resolve {
            req = req.query(&[("resolve", r.to_string())]);
        }
        if let Some(l) = self.limit {
            req = req.query(&[("limit", l.to_string())]);
        }
        if let Some(o) = self.offset {
            req = req.query(&[("offset", o.to_string())]);
        }
        if let Some(a) = self.account_id {
            req = req.query(&[("account_id", a)]);
        }
        if let Some(min) = self.min_id {
            req = req.query(&[("min_id", min)]);
        }
        if let Some(max) = self.max_id {
            req = req.query(&[("max_id", max)]);
        }

        self.client.send(req).await
    }
}
