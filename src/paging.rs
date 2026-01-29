use crate::MastodonClient;
use crate::error::Result;
use serde::de::DeserializeOwned;

/// A request that supports automatic pagination via the `Link` header.
///
/// This is used to fetch multiple pages of results (e.g., historical timelines)
/// by following the "next" link provided by the server.
pub struct PagedRequest<'a, T> {
    client: &'a MastodonClient,
    next_url: Option<String>,
    _marker: std::marker::PhantomData<T>,
}

impl<'a, T: DeserializeOwned> PagedRequest<'a, T> {
    /// Creates a new `PagedRequest` starting at the given URL.
    pub fn new(client: &'a MastodonClient, initial_url: String) -> Self {
        Self {
            client,
            next_url: Some(initial_url),
            _marker: std::marker::PhantomData,
        }
    }

    /// Fetches the next page of results.
    ///
    /// Returns `Ok(Some(Vec<T>))` if a page was successfully fetched,
    /// or `Ok(None)` if there are no more pages.
    pub async fn next_page(&mut self) -> Result<Option<Vec<T>>> {
        let url = match &self.next_url {
            Some(u) => u,
            None => return Ok(None),
        };

        let response = self.client.http_client().get(url);
        let builder = response;

        let mut req_builder = builder;
        if let Some(token) = self.client.access_token() {
            req_builder = req_builder.bearer_auth(token);
        }

        let resp = req_builder.send().await?;

        if !resp.status().is_success() {
            return Err(crate::error::MastodonError::ApiError {
                status: resp.status(),
                message: resp.text().await.unwrap_or_default(),
            });
        }

        // Parse Link header
        self.next_url = parse_link_header(resp.headers().get("Link"));

        Ok(Some(resp.json().await?))
    }
}

/// Helper to parse the `Link` header into a URL string for the next page.
fn parse_link_header(header: Option<&reqwest::header::HeaderValue>) -> Option<String> {
    let header_str = header?.to_str().ok()?;
    // Example: <url>; rel="next", <url>; rel="prev"
    for part in header_str.split(',') {
        if part.contains("rel=\"next\"") {
            let start = part.find('<')? + 1;
            let end = part.find('>')?;
            return Some(part[start..end].to_string());
        }
    }
    None
}
