# MastDoon API Wrapper 🦀

A comprehensive, asynchronous, and industrial-grade Rust wrapper for the Mastodon API. Designed for building powerful bots and automations with ease.

## Features

- **Full API Coverage**: ~150 endpoints categorized into logical handlers.
- **WebSocket Streaming**: Real-time event handling for responsive bots.
- **Automatic Pagination**: Hassle-free navigation of timelines and lists.
- **Robust Models**: Exhaustive type-safe representations of all Mastodon entities.
- **Async/Await**: Built on `tokio` and `reqwest`.

## Quick Start

### Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
mastdoon-api = { git = "https://github.com/borgox/mastdoon-api" }
tokio = { version = "1.0", features = ["full"] }
```

### Basic Example

```rust
use mastdoon_api::MastodonClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MastodonClient::new("https://mastodon.social") // this can be any url of a mastodon 
        .with_token("YOUR_ACCESS_TOKEN");

    // Post a status
    client.statuses().create_simple("Hello world!").await?;

    Ok(())
}
```

## Modules

| Handler | Description |
| :--- | :--- |
| `accounts()` | Lookup, follow, block, and manage profiles. |
| `statuses()` | Post, reply, edit, and interact with posts. |
| `timelines()` | Fetch home, public, tag, and list timelines. |
| `streaming()` | Real-time WebSocket connection for "user", "public", etc. |
| `media()` | Upload and manage attachments. |
| `notifications()` | List and manage notifications. |
| `filters()` | Manage keyword filters. |
| `lists()` | Create and manage account lists. |
| `apps()` | Register applications and handle OAuth. |

## Paging

For endpoints that return lists, you can use the automatic paging support:

```rust
let mut paged = client.timelines().public_paged().await?;
while let Some(page) = paged.next_page().await? {
    for status in page {
        println!("{}", status.content);
    }
}
```

## Documentation

The most comprehensive and up-to-date documentation is generated directly from the source code. You can view it by running:
```powershell
cargo doc --open
```
This will open a searchable API reference in your browser, including detailed descriptions of all methods, models, and core concepts.

## Examples

Check the `examples/` directory for more advanced use cases:
- `basic_posting.rs`: Standard interactions.
- `streaming_bot.rs`: Real-time reactive bot.

## License

MIT
