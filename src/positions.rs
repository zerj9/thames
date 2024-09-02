use crate::{Client, Contract, OutgoingMsgId};
use anyhow::Result;

#[derive(Debug, Clone, PartialEq)]
pub struct PositionData {
    pub account: String,
    pub contract: Contract,
    pub position: f64,
    pub avg_cost: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PositionEnd {}

impl Client {
    pub async fn req_positions(&self) -> Result<()> {
        self.send_message(vec![OutgoingMsgId::ReqPositions.as_ref(), "1"])
            .await?;
        Ok(())
    }
}
