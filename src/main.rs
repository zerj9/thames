// EXAMPLE CODE:
use anyhow::{Context, Result};
#[allow(unused_imports)]
use thames::{
    AccountUpdateTime, Client, MessageHandler, PositionData, ReqAccountSummary, ReqAccountUpdates,
};
use tokio::time::{sleep, Duration};

struct CustomMessageHandler;

impl MessageHandler for CustomMessageHandler {
    fn account_update_time(&self, account_update_time: AccountUpdateTime) -> Result<()> {
        println!(
            "Last time the account was updated: {}",
            account_update_time.timestamp
        );
        Ok(())
    }

    fn position_data(&self, message: PositionData) -> Result<()> {
        let stock = message.contract.clone();
        println!("overriden position_data handler: {stock:?}");
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let custom_handler = CustomMessageHandler;
    // Create client and connect to TWS/IB Gateway
    let client = Client::new_with_handler("127.0.0.1:4002", custom_handler)
        .await
        .context("Failed to create TwsClient")?;
    client.connect().await?;

    // Send request to IBKR API for position data
    client.req_positions().await?;

    // Empty loop to keep the program running
    loop {
        sleep(Duration::from_secs(1)).await;
    }
}
