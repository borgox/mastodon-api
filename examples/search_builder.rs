use mastodon_api::MastodonClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MastodonClient::new("https://mastodon.social").with_token("YOUR_TOKEN");

    println!("Searching for accounts matching 'rust'...");

    let search_results = client
        .search()
        .builder("rust")
        .r#type("accounts")
        .limit(5)
        .resolve(true)
        .send()
        .await?;

    for account in search_results.accounts {
        println!("Found: {} (@{})", account.display_name, account.acct);
    }

    Ok(())
}
