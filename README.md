# Mastodon API Wrapper

A comprehensive, asynchronous Rust wrapper for the Mastodon API, designed for advanced bot development.

## 🚀 Features

- **Asynchronous**: Built on `tokio` and `reqwest`.
- **Comprehensive Docs**: Integrated documentation explaining Mastodon concepts.
- **Streaming Support**: Real-time event subscription via WebSockets.
- **Paging Support**: Easy navigation through paginated API results.
- **Strongly Typed**: Models for all core Mastodon entities.

## 📚 Documentation

The documentation has been migrated to Rustdoc for the best developer experience.

- **Conceptual Guide**: Read the main crate documentation in `src/lib.rs` for an overview of Timelines, Statuses, and Streaming.
- **API Reference**: Generated documentation is available in the `docs/` directory.

To generate the documentation locally, run:
```bash
cargo doc --no-deps --open
```
## 📝 Installation

Add the following git dependency to your `Cargo.toml`:
```toml
[dependencies]
mastodon_api = { git = "https://github.com/borgox/mastodon-api.git" }
```
> or once released on crates.io:
```toml
[dependencies]
mastodon_api = "0.1.0" # or the latest version
```
## 🛠️ Quick Start

```rust
use mastodon_api::MastodonClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MastodonClient::new("https://mastodon.social") // this can be any mastodon instance url
        .with_token("your_access_token");

    // Fetch account details
    let me = client.accounts().verify_credentials().await?;
    println!("Logged in as: {}", me.display_name);

    // Post a status
    client.statuses().create_simple("Hello from Mastodon!").await?;

    Ok(())
}
```

## 📋 Roadmap & Contributing

Check [TODO.md](TODO.md) for planned features and missing API coverage. Contributions are welcome!

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
