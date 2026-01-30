use mastodon_api::MastodonClient;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize admin client
    let client = MastodonClient::new("https://mastodon.social").with_token("ADMIN_TOKEN");

    println!("Starting Auto-Moderation Bot...");

    // 2. Periodically poll for new reports
    let mut interval = tokio::time::interval(Duration::from_secs(60)); // Every minute
    loop {
        interval.tick().await;
        println!("Checking for new reports...");

        let reports = client.admin().reports().list().await?;
        
        // Note: reports is a serde_json::Value here as we used Value for simplicity in some admin methods.
        // In a real bot, we'd parse this into a model.
        if let Some(report_list) = reports.as_array() {
            for report in report_list {
                let id = report["id"].as_str().unwrap_or_default();
                let comment = report["comment"].as_str().unwrap_or_default();
                let category = report["category"].as_str().unwrap_or_default();

                println!("Processing report {} (Category: {})", id, category);

                // Simple logic: if a report contains a specific keyword, resolve it automatically
                // or escalate it (in this example we just resolve for demo purposes).
                if comment.contains("spam") || category == "spam" {
                    println!("Auto-resolving spam report {}...", id);
                    client.admin().reports().resolve(id).await?;
                    
                    // Also maybe silence the offending account
                    if let Some(account_id) = report["target_account"]["id"].as_str() {
                        client.admin().accounts().silence(account_id).await?;
                        println!("Silenced account {}", account_id);
                    }
                }
            }
        }
    }
}
