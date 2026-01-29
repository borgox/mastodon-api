# TODO: Full API Coverage Checklist

The following endpoints and features are currently missing from the `mastdoon-api` wrapper. These are prioritized by their usefulness for bot developers.

## 1. High Priority (Social & Interaction)
- [ ] **Follow Requests**: Manage incoming follow requests (`GET /api/v1/follow_requests`, `POST /api/v1/follow_requests/:id/authorize`).
- [ ] **Announcements**: Fetch and interact with instance-wide announcements (`GET /api/v1/announcements`, `POST /api/v1/announcements/:id/dismiss`).
- [ ] **Suggestions**: Fetch suggested accounts to follow (`GET /api/v1/suggestions`).
- [ ] **Followed Tags**: Manage tags the account resides in (`GET /api/v1/followed_tags`).
- [ ] **Featured Tags**: Manage tags featured on the profile.
- [ ] **Markers**: Sync reading position across devices (`GET /api/v1/markers`, `POST /api/v1/markers`).

## 2. Medium Priority (Utility & Metadata)
- [ ] **Endorsements**: Profiles you've chosen to feature on your own profile.
- [ ] **Reports**: Reporting statuses or accounts for rules violations.
- [ ] **Domain Blocks**: Authenticated user-level domain blocks.
- [ ] **Preferences**: Fetching user-specific account preferences.
- [ ] **Push Notifications**: Managing Web Push API subscriptions (different from the standard notification list).

## 3. Instance & Discoverability
- [ ] **Directories**: Instance-level user directory.
- [ ] **Instance Peers**: List of domains this instance is aware of.
- [ ] **Instance Activity**: Weekly usage statistics for the instance.
- [ ] **Instance Rules**: Formal rules established by the server.

## 4. Admin API (Advanced)
> These require special admin-level scopes and are usually only used for moderation bots.
- [ ] **Account Moderation**: Actioning, silencing, and suspending accounts.
- [ ] **Report Management**: Handling incoming reports.
- [ ] **Domain Federation**: Managing allowed/blocked domains at the server level.
- [ ] **IP Blocks**: Managing firewall-level blocks.

## 5. Technical Improvements
- [ ] **Robust Rate Limiting**: Implementing a more proactive leaky-bucket rate limiter.
- [ ] **Automatic Retries**: Retrying idempotent requests on 5xx or network errors.
- [ ] **Builder Pattern for Search/Lists**: Moving away from complex parameter structs to a more ergonomic builder pattern.
