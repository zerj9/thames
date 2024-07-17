use anyhow::{Context, Result};

use thames::Client;
use thames::OutgoingMsgId;

use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<()> {
    // Create client and connect to TWS/IB Gateway
    let client = Client::new("127.0.0.1:4002")
        .await
        .context("Failed to create TwsClient")?;
    client.connect().await?;

    // Request account summary
    client
        .send_message(vec![
            OutgoingMsgId::ReqAccountSummary.as_ref(),
            "0",
            "4",
            "All",
            "AccountType,NetLiquidation,TotalCashValue",
        ])
        .await?;

    // Empty loop to keep the program running
    loop {
        sleep(Duration::from_secs(1)).await;
    }
}
