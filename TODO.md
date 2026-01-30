# TODO: Full API Coverage Checklist [COMPLETED]

The following endpoints and features are now implemented in the `mastodon-api` wrapper.

## 1. High Priority (Social & Interaction)
- [x] **Follow Requests**: Manage incoming follow requests (`GET /api/v1/follow_requests`, `POST /api/v1/follow_requests/:id/authorize`).
- [x] **Announcements**: Fetch and interact with instance-wide announcements (`GET /api/v1/announcements`, `POST /api/v1/announcements/:id/dismiss`).
- [x] **Suggestions**: Fetch suggested accounts to follow (`GET /api/v1/suggestions`).
- [x] **Followed Tags**: Manage tags the account resides in (`GET /api/v1/followed_tags`).
- [x] **Featured Tags**: Manage tags featured on the profile.
- [x] **Markers**: Sync reading position across devices (`GET /api/v1/markers`, `POST /api/v1/markers`).

## 2. Medium Priority (Utility & Metadata)
- [x] **Endorsements**: Profiles you've chosen to feature on your own profile.
- [x] **Reports**: Reporting statuses or accounts for rules violations.
- [x] **Domain Blocks**: Authenticated user-level domain blocks.
- [x] **Preferences**: Fetching user-specific account preferences.
- [x] **Push Notifications**: Managing Web Push API subscriptions.

## 3. Instance & Discoverability
- [x] **Directories**: Instance-level user directory.
- [x] **Instance Peers**: List of domains this instance is aware of.
- [x] **Instance Activity**: Weekly usage statistics for the instance.
- [x] **Instance Rules**: Formal rules established by the server.

## 4. Admin API (Advanced)
- [x] **Account Moderation**: Actioning, silencing, and suspending accounts.
- [x] **Report Management**: Handling incoming reports.
- [x] **Domain Federation**: Managing allowed/blocked domains at the server level.
- [x] **IP Blocks**: Managing firewall-level blocks.

## 5. Technical Improvements
- [x] **Robust Rate Limiting**: Client now respects rate limit headers.
- [x] **Automatic Retries**: Exponential backoff on 429 and 5xx errors.
- [x] **Builder Pattern for Search/Lists**: Ergonomic builder pattern for complex requests.
