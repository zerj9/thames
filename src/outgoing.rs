//use serde::{Deserialize, Serialize};
use anyhow::Result;
use strum_macros::{Display, EnumString};

use crate::Client;

#[allow(dead_code)] // dirty?
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum OutgoingMsgId {
    ReqMktData = 1,
    CancelMktData = 2,
    PlaceOrder = 3,
    CancelOrder = 4,
    ReqOpenOrders = 5,
    ReqAcctData = 6,
    ReqExecutions = 7,
    ReqIds = 8,
    ReqContractData = 9,
    ReqMktDepth = 10,
    CancelMktDepth = 11,
    ReqNewsBulletins = 12,
    CancelNewsBulletins = 13,
    SetServerLogLevel = 14,
    ReqAutoOpenOrders = 15,
    ReqAllOpenOrders = 16,
    ReqManagedAccts = 17,
    ReqFa = 18,
    ReplaceFa = 19,
    ReqHistoricalData = 20,
    ExerciseOptions = 21,
    ReqScannerSubscription = 22,
    CancelScannerSubscription = 23,
    ReqScannerParameters = 24,
    CancelHistoricalData = 25,
    ReqCurrentTime = 49,
    ReqRealTimeBars = 50,
    CancelRealTimeBars = 51,
    ReqFundamentalData = 52,
    CancelFundamentalData = 53,
    ReqCalcImpliedVolat = 54,
    ReqCalcOptionPrice = 55,
    CancelCalcImpliedVolat = 56,
    CancelCalcOptionPrice = 57,
    ReqGlobalCancel = 58,
    ReqMarketDataType = 59,
    ReqPositions = 61,
    ReqAccountSummary = 62,
    CancelAccountSummary = 63,
    CancelPositions = 64,
    VerifyRequest = 65,
    VerifyMessage = 66,
    QueryDisplayGroups = 67,
    SubscribeToGroupEvents = 68,
    UpdateDisplayGroup = 69,
    UnsubscribeFromGroupEvents = 70,
    StartApi = 71,
    VerifyAndAuthRequest = 72,
    VerifyAndAuthMessage = 73,
    ReqPositionsMulti = 74,
    CancelPositionsMulti = 75,
    ReqAccountUpdatesMulti = 76,
    CancelAccountUpdatesMulti = 77,
    ReqSecDefOptParams = 78,
    ReqSoftDollarTiers = 79,
    ReqFamilyCodes = 80,
    ReqMatchingSymbols = 81,
    ReqMktDepthExchanges = 82,
    ReqSmartComponents = 83,
    ReqNewsArticle = 84,
    ReqNewsProviders = 85,
    ReqHistoricalNews = 86,
    ReqHeadTimestamp = 87,
    ReqHistogramData = 88,
    CancelHistogramData = 89,
    CancelHeadTimestamp = 90,
    ReqMarketRule = 91,
    ReqPnl = 92,
    CancelPnl = 93,
    ReqPnlSingle = 94,
    CancelPnlSingle = 95,
    ReqHistoricalTicks = 96,
    ReqTickByTickData = 97,
    CancelTickByTickData = 98,
    ReqCompletedOrders = 99,
    ReqWshMetaData = 100,
    CancelWshMetaData = 101,
    ReqWshEventData = 102,
    CancelWshEventData = 103,
    ReqUserInfo = 104,
}

impl AsRef<str> for OutgoingMsgId {
    fn as_ref(&self) -> &str {
        match self {
            Self::ReqMktData => "1",
            Self::CancelMktData => "2",
            Self::PlaceOrder => "3",
            Self::CancelOrder => "4",
            Self::ReqOpenOrders => "5",
            Self::ReqAcctData => "6",
            Self::ReqExecutions => "7",
            Self::ReqIds => "8",
            Self::ReqContractData => "9",
            Self::ReqMktDepth => "10",
            Self::CancelMktDepth => "11",
            Self::ReqNewsBulletins => "12",
            Self::CancelNewsBulletins => "13",
            Self::SetServerLogLevel => "14",
            Self::ReqAutoOpenOrders => "15",
            Self::ReqAllOpenOrders => "16",
            Self::ReqManagedAccts => "17",
            Self::ReqFa => "18",
            Self::ReplaceFa => "19",
            Self::ReqHistoricalData => "20",
            Self::ExerciseOptions => "21",
            Self::ReqScannerSubscription => "22",
            Self::CancelScannerSubscription => "23",
            Self::ReqScannerParameters => "24",
            Self::CancelHistoricalData => "25",
            Self::ReqCurrentTime => "49",
            Self::ReqRealTimeBars => "50",
            Self::CancelRealTimeBars => "51",
            Self::ReqFundamentalData => "52",
            Self::CancelFundamentalData => "53",
            Self::ReqCalcImpliedVolat => "54",
            Self::ReqCalcOptionPrice => "55",
            Self::CancelCalcImpliedVolat => "56",
            Self::CancelCalcOptionPrice => "57",
            Self::ReqGlobalCancel => "58",
            Self::ReqMarketDataType => "59",
            Self::ReqPositions => "61",
            Self::ReqAccountSummary => "62",
            Self::CancelAccountSummary => "63",
            Self::CancelPositions => "64",
            Self::VerifyRequest => "65",
            Self::VerifyMessage => "66",
            Self::QueryDisplayGroups => "67",
            Self::SubscribeToGroupEvents => "68",
            Self::UpdateDisplayGroup => "69",
            Self::UnsubscribeFromGroupEvents => "70",
            Self::StartApi => "71",
            Self::VerifyAndAuthRequest => "72",
            Self::VerifyAndAuthMessage => "73",
            Self::ReqPositionsMulti => "74",
            Self::CancelPositionsMulti => "75",
            Self::ReqAccountUpdatesMulti => "76",
            Self::CancelAccountUpdatesMulti => "77",
            Self::ReqSecDefOptParams => "78",
            Self::ReqSoftDollarTiers => "79",
            Self::ReqFamilyCodes => "80",
            Self::ReqMatchingSymbols => "81",
            Self::ReqMktDepthExchanges => "82",
            Self::ReqSmartComponents => "83",
            Self::ReqNewsArticle => "84",
            Self::ReqNewsProviders => "85",
            Self::ReqHistoricalNews => "86",
            Self::ReqHeadTimestamp => "87",
            Self::ReqHistogramData => "88",
            Self::CancelHistogramData => "89",
            Self::CancelHeadTimestamp => "90",
            Self::ReqMarketRule => "91",
            Self::ReqPnl => "92",
            Self::CancelPnl => "93",
            Self::ReqPnlSingle => "94",
            Self::CancelPnlSingle => "95",
            Self::ReqHistoricalTicks => "96",
            Self::ReqTickByTickData => "97",
            Self::CancelTickByTickData => "98",
            Self::ReqCompletedOrders => "99",
            Self::ReqWshMetaData => "100",
            Self::CancelWshMetaData => "101",
            Self::ReqWshEventData => "102",
            Self::CancelWshEventData => "103",
            Self::ReqUserInfo => "104",
        }
    }
}

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
