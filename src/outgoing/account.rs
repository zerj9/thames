use crate::{Client, OutgoingMsgId};
use anyhow::Result;
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, PartialEq, Eq, Hash, EnumString, Display)]
#[strum(serialize_all = "PascalCase")]
pub enum OutgoingAccountSummaryTag {
    AccountType,
    NetLiquidation,
    TotalCashValue,
    SettledCash,
    AccruedCash,
    BuyingPower,
    EquityWithLoanValue,
    PreviousEquityWithLoanValue,
    GrossPositionValue,
    RegTEquity,
    RegTMargin,
    SMA,
    InitMarginReq,
    MaintMarginReq,
    AvailableFunds,
    ExcessLiquidity,
    Cushion,
    FullInitMarginReq,
    FullMaintMarginReq,
    FullAvailableFunds,
    FullExcessLiquidity,
    LookAheadNextChange,
    LookAheadInitMarginReq,
    LookAheadMaintMarginReq,
    LookAheadAvailableFunds,
    LookAheadExcessLiquidity,
    HighestSeverity,
    DayTradesRemaining,
    Leverage,
    Ledger,
    LedgerCurrency,
    LedgerAll,
}

#[derive(EnumString, Display)]
#[strum(serialize_all = "lowercase")]
pub enum BoolStr {
    #[strum(serialize = "0")]
    False,
    #[strum(serialize = "1")]
    True,
}

#[derive(Debug)]
pub struct ReqAccountSummary {
    pub req_id: u64,
    pub group: String,
    pub tags: Vec<OutgoingAccountSummaryTag>,
}

impl ReqAccountUpdates {
    pub fn subscribe_str(&self) -> &'static str {
        if self.subscribe {
            "1"
        } else {
            "0"
        }
    }
}

pub struct ReqAccountUpdates {
    pub subscribe: bool,
    pub acct_code: String,
}

impl Client {
    // https://ibkrcampus.com/campus/ibkr-api-page/twsapi-doc/#requesting-account-summary
    pub async fn req_account_summary(&self, input: ReqAccountSummary) -> Result<()> {
        let req_id = input.req_id;
        let group = input.group;
        let tags = input.tags;
        self.send_message(vec![
            OutgoingMsgId::ReqAccountSummary.as_ref(),
            "0", // TODO: message version
            &req_id.to_string(),
            &group,
            &tags
                .iter()
                .map(|tag| tag.to_string())
                .collect::<Vec<String>>()
                .join(","),
        ])
        .await?;
        Ok(())
    }

    // https://www.interactivebrokers.com/campus/ibkr-api-page/twsapi-doc/#cancel-account-summary
    pub async fn cancel_account_summary(&self, req_id: u64) -> Result<()> {
        self.send_message(vec![
            OutgoingMsgId::CancelAccountSummary.as_ref(),
            "1", // TODO: message version
            &req_id.to_string(),
        ])
        .await?;
        Ok(())
    }

    pub async fn req_account_updates(&self, input: ReqAccountUpdates) -> Result<()> {
        self.send_message(vec![
            OutgoingMsgId::ReqAcctData.as_ref(),
            "2", // TODO: message version
            if input.subscribe { "1" } else { "0" },
            &input.acct_code,
        ])
        .await?;
        Ok(())
    }
}
