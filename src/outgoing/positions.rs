use crate::{Client, OutgoingMsgId};
use anyhow::Result;

impl Client {
    pub async fn req_positions(&self) -> Result<()> {
        self.send_message(vec![OutgoingMsgId::ReqPositions.as_ref(), "1"])
            .await?;
        Ok(())
    }
}
