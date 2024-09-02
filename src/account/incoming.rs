use super::AccountSummaryTag;

#[derive(Debug, Clone, PartialEq)]
pub struct ManagedAccounts {
    pub accounts: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AccountSummary {
    pub req_id: i32,
    pub account: String,
    pub tag: AccountSummaryTag,
    pub value: String,
    pub currency: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AccountSummaryEnd {
    pub req_id: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AccountUpdateTime {
    pub timestamp: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AccountValue {
    pub key: String,
    pub value: String,
    pub currency: Option<String>,
    pub account: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AccountDownloadEnd {
    pub account: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PortfolioValue {
    pub contract_id: String,
    pub symbol: String,
    pub sec_type: String,
    pub last_trade_date_or_contract_month: String,
    pub strike: String,
    pub right: String,
    pub multiplier: String,
    pub primary_exchange: String,
    pub currency: String,
    pub local_symbol: String,
    pub trading_class: String,
    pub position: String,
    pub market_price: String,
    pub market_value: String,
    pub average_cost: String,
    pub unrealized_pnl: String,
    pub realized_pnl: String,
    pub account: String,
}
