use crate::MastodonClient;
use crate::error::Result;
use crate::models::report::Report;

/// Handler for report-related API endpoints.
pub struct ReportsHandler<'a> {
    client: &'a MastodonClient,
}

impl<'a> ReportsHandler<'a> {
    /// Creates a new `ReportsHandler` for the given client.
    pub fn new(client: &'a MastodonClient) -> Self {
        Self { client }
    }

    /// Fetches all reports made by the authenticated user.
    ///
    /// Returns:
    /// - `Result<Vec<Report>>`: The reports.
    ///
    /// Corresponds to `GET /api/v1/reports`.
    pub async fn list(&self) -> Result<Vec<Report>> {
        let url = format!("{}/api/v1/reports", self.client.base_url());
        let req = self.client.http_client().get(&url);
        self.client.send(req).await
    }

    /// Creates a new report.
    ///
    /// Parameters:
    /// - `account_id`: The ID of the account being reported.
    /// - `status_ids`: IDs of statuses to include in the report.
    /// - `comment`: A comment to include with the report.
    /// - `forward`: Whether to forward the report to the remote instance.
    /// - `category`: The category of the report.
    /// - `rule_ids`: IDs of rules that were violated.
    ///
    /// Returns:
    /// - `Result<Report>`: The created report.
    ///
    /// Corresponds to `POST /api/v1/reports`.
    pub async fn create(
        &self,
        account_id: &str,
        status_ids: Option<&[String]>,
        comment: Option<&str>,
        forward: Option<bool>,
        category: Option<&str>,
        rule_ids: Option<&[u32]>,
    ) -> Result<Report> {
        let url = format!("{}/api/v1/reports", self.client.base_url());
        let mut req = self.client.http_client().post(&url);

        let mut form = vec![("account_id", account_id.to_string())];
        if let Some(ids) = status_ids {
            for id in ids {
                form.push(("status_ids[]", id.clone()));
            }
        }
        if let Some(c) = comment {
            form.push(("comment", c.to_string()));
        }
        if let Some(f) = forward {
            form.push(("forward", f.to_string()));
        }
        if let Some(cat) = category {
            form.push(("category", cat.to_string()));
        }
        if let Some(r_ids) = rule_ids {
            for id in r_ids {
                form.push(("rule_ids[]", id.to_string()));
            }
        }

        req = req.form(&form);
        self.client.send(req).await
    }
}
