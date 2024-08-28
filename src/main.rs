use anyhow::{Context, Result};
use thames::{Client, OutgoingAccountSummaryTag, ReqAccountSummary};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<()> {
    // Create client and connect to TWS/IB Gateway
    let client = Client::new("127.0.0.1:4002")
        .await
        .context("Failed to create TwsClient")?;
    client.connect().await?;

    client
        .req_account_summary(ReqAccountSummary {
            req_id: 1,
            group: "All".to_string(),
            tags: vec![
                OutgoingAccountSummaryTag::AccountType,
                OutgoingAccountSummaryTag::TotalCashValue,
                OutgoingAccountSummaryTag::NetLiquidation,
            ],
        })
        .await?;

    // Empty loop to keep the program running
    loop {
        sleep(Duration::from_secs(1)).await;
    }
}
