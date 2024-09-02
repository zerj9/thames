use crate::MessageProcessor;
use crate::{
    incoming::{AccountSummary, AccountSummaryTag},
    AccountDownloadEnd, AccountSummaryEnd, AccountUpdateTime, AccountValue, ManagedAccounts,
    PortfolioValue,
};
use anyhow::Result;

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
    pub(crate) fn parse_managed_accounts(parts: &[&str]) -> Result<ManagedAccounts> {
        Ok(ManagedAccounts {
            accounts: parts[1].to_string(),
        })
    }

    pub(crate) fn parse_account_summary(parts: &[&str]) -> Result<AccountSummary> {
        Ok(AccountSummary {
            req_id: parts[1].parse()?,
            account: parts[2].to_string(),
            tag: parts[3].into(),
            value: parts[4].to_string(),
            currency: parts.get(5).map(|&s| s.to_string()),
        })
    }

    pub(crate) fn parse_account_summary_end(parts: &[&str]) -> Result<AccountSummaryEnd> {
        Ok(AccountSummaryEnd {
            req_id: parts[1].parse()?,
        })
    }

    pub(crate) fn parse_account_update_time(parts: &[&str]) -> Result<AccountUpdateTime> {
        Ok(AccountUpdateTime {
            timestamp: parts[1].to_string(),
        })
    }

    pub(crate) fn parse_account_value(parts: &[&str]) -> Result<AccountValue> {
        Ok(AccountValue {
            key: parts[1].to_string(),
            value: parts[2].to_string(),
            currency: if parts[3].is_empty() {
                None
            } else {
                Some(parts[3].to_string())
            },
            account: parts[4].to_string(),
        })
    }

    pub(crate) fn parse_account_download_end(parts: &[&str]) -> Result<AccountDownloadEnd> {
        Ok(AccountDownloadEnd {
            account: parts[1].to_string(),
        })
    }

    pub(crate) fn parse_portfolio_update_value(parts: &[&str]) -> Result<PortfolioValue> {
        Ok(PortfolioValue {
            contract_id: parts[1].to_string(),
            symbol: parts[2].to_string(),
            sec_type: parts[3].to_string(),
            last_trade_date_or_contract_month: parts[4].to_string(),
            strike: parts[5].to_string(),
            right: parts[6].to_string(),
            multiplier: parts[7].to_string(),
            primary_exchange: parts[8].to_string(),
            currency: parts[9].to_string(),
            local_symbol: parts[10].to_string(),
            trading_class: parts[11].to_string(),
            position: parts[12].to_string(),
            market_price: parts[13].to_string(),
            market_value: parts[14].to_string(),
            average_cost: parts[15].to_string(),
            unrealized_pnl: parts[16].to_string(),
            realized_pnl: parts[17].to_string(),
            account: parts[18].to_string(),
        })
    }
}
