pub mod accounts;
pub mod domain_federation;
pub mod ip_blocks;
pub mod reports;

use crate::MastodonClient;

/// Handler for admin-related API endpoints.
pub struct AdminHandler<'a> {
    client: &'a MastodonClient,
}

impl<'a> AdminHandler<'a> {
    /// Creates a new `AdminHandler` for the given client.
    pub fn new(client: &'a MastodonClient) -> Self {
        Self { client }
    }

    /// Access admin account moderation endpoints.
    pub fn accounts(&self) -> accounts::AdminAccountsHandler<'_> {
        accounts::AdminAccountsHandler::new(self.client)
    }

    /// Access admin report management endpoints.
    pub fn reports(&self) -> reports::AdminReportsHandler<'_> {
        reports::AdminReportsHandler::new(self.client)
    }

    /// Access admin domain federation endpoints.
    pub fn domain_federation(&self) -> domain_federation::AdminDomainFederationHandler<'_> {
        domain_federation::AdminDomainFederationHandler::new(self.client)
    }

    /// Access admin IP block management endpoints.
    pub fn ip_blocks(&self) -> ip_blocks::AdminIpBlocksHandler<'_> {
        ip_blocks::AdminIpBlocksHandler::new(self.client)
    }
}
