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
pub enum AccountSummaryTag {
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
    LookAheadNextChangeTime,
    LookAheadInitMarginReq,
    LookAheadMaintMarginReq,
    LookAheadAvailableFunds,
    LookAheadExcessLiquidity,
    HighestSeverity,
    DayTradesRemaining,
    Leverage,
    Ledger,
    LedgerCurrency(String),
    LedgerAll,
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
