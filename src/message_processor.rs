use crate::incoming::{AccountSummary, AccountSummaryTag};
use anyhow::Result;
pub(crate) struct MessageProcessor;

impl From<&str> for AccountSummaryTag {
    fn from(s: &str) -> Self {
        match s {
            "AccountType" => AccountSummaryTag::AccountType,
            "NetLiquidation" => AccountSummaryTag::NetLiquidation,
            "TotalCashValue" => AccountSummaryTag::TotalCashValue,
            "SettledCash" => AccountSummaryTag::SettledCash,
            "AccruedCash" => AccountSummaryTag::AccruedCash,
            "BuyingPower" => AccountSummaryTag::BuyingPower,
            "EquityWithLoanValue" => AccountSummaryTag::EquityWithLoanValue,
            "PreviousEquityWithLoanValue" => AccountSummaryTag::PreviousEquityWithLoanValue,
            "GrossPositionValue" => AccountSummaryTag::GrossPositionValue,
            "RegTEquity" => AccountSummaryTag::RegTEquity,
            "RegTMargin" => AccountSummaryTag::RegTMargin,
            "SMA" => AccountSummaryTag::SMA,
            "InitMarginReq" => AccountSummaryTag::InitMarginReq,
            "MaintMarginReq" => AccountSummaryTag::MaintMarginReq,
            "AvailableFunds" => AccountSummaryTag::AvailableFunds,
            "ExcessLiquidity" => AccountSummaryTag::ExcessLiquidity,
            "Cushion" => AccountSummaryTag::Cushion,
            "FullInitMarginReq" => AccountSummaryTag::FullInitMarginReq,
            "FullMaintMarginReq" => AccountSummaryTag::FullMaintMarginReq,
            "FullAvailableFunds" => AccountSummaryTag::FullAvailableFunds,
            "FullExcessLiquidity" => AccountSummaryTag::FullExcessLiquidity,
            "LookAheadNextChangeTime" => AccountSummaryTag::LookAheadNextChangeTime,
            "LookAheadInitMarginReq" => AccountSummaryTag::LookAheadInitMarginReq,
            "LookAheadMaintMarginReq" => AccountSummaryTag::LookAheadMaintMarginReq,
            "LookAheadAvailableFunds" => AccountSummaryTag::LookAheadAvailableFunds,
            "LookAheadExcessLiquidity" => AccountSummaryTag::LookAheadExcessLiquidity,
            "HighestSeverity" => AccountSummaryTag::HighestSeverity,
            "DayTradesRemaining" => AccountSummaryTag::DayTradesRemaining,
            "Leverage" => AccountSummaryTag::Leverage,
            "Ledger" => AccountSummaryTag::Ledger,
            s if s.starts_with("LedgerCurrency:") => AccountSummaryTag::LedgerCurrency(
                s.strip_prefix("LedgerCurrency:").unwrap().to_string(),
            ),
            "LedgerAll" => AccountSummaryTag::LedgerAll,
            _ => panic!("Unknown AccountSummaryTag: {}", s),
        }
    }
}

impl MessageProcessor {
    pub(crate) fn parse_account_summary(parts: &[&str]) -> Result<AccountSummary> {
        Ok(AccountSummary {
            req_id: parts[1].parse()?,
            account: parts[2].to_string(),
            tag: parts[3].into(),
            value: parts[4].to_string(),
            currency: parts.get(5).map(|&s| s.to_string()),
        })
    }
}
