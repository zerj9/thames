use crate::MessageProcessor;
use anyhow::Result;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum IncomingMsgId {
    TickPrice = 1,
    TickSize = 2,
    OrderStatus = 3,
    ErrorMsg = 4,
    OpenOrder = 5,
    AccountValue = 6,
    PortfolioValue = 7,
    AccountUpdateTime = 8,
    NextValidId = 9,
    ContractData = 10,
    ExecutionData = 11,
    MarketDepth = 12,
    MarketDepthL2 = 13,
    NewsBulletins = 14,
    ManagedAccounts = 15,
    ReceiveFa = 16,
    HistoricalData = 17,
    BondContractData = 18,
    ScannerParameters = 19,
    ScannerData = 20,
    TickOptionComputation = 21,
    TickGeneric = 45,
    TickString = 46,
    TickEfp = 47,
    CurrentTime = 49,
    RealTimeBars = 50,
    FundamentalData = 51,
    ContractDataEnd = 52,
    OpenOrderEnd = 53,
    AccountDownloadEnd = 54,
    ExecutionDataEnd = 55,
    DeltaNeutralValidation = 56,
    TickSnapshotEnd = 57,
    MarketDataType = 58,
    CommissionReport = 59,
    PositionData = 61,
    PositionEnd = 62,
    AccountSummary = 63,
    AccountSummaryEnd = 64,
    VerifyMessageApi = 65,
    VerifyCompleted = 66,
    DisplayGroupList = 67,
    DisplayGroupUpdated = 68,
    VerifyAndAuthMessageApi = 69,
    VerifyAndAuthCompleted = 70,
    PositionMulti = 71,
    PositionMultiEnd = 72,
    AccountUpdateMulti = 73,
    AccountUpdateMultiEnd = 74,
    SecurityDefinitionOptionParameter = 75,
    SecurityDefinitionOptionParameterEnd = 76,
    SoftDollarTiers = 77,
    FamilyCodes = 78,
    SymbolSamples = 79,
    MktDepthExchanges = 80,
    TickReqParams = 81,
    SmartComponents = 82,
    NewsArticle = 83,
    TickNews = 84,
    NewsProviders = 85,
    HistoricalNews = 86,
    HistoricalNewsEnd = 87,
    HeadTimestamp = 88,
    HistogramData = 89,
    HistoricalDataUpdate = 90,
    RerouteMktDataReq = 91,
    RerouteMktDepthReq = 92,
    MarketRule = 93,
    Pnl = 94,
    PnlSingle = 95,
    HistoricalTicks = 96,
    HistoricalTicksBidAsk = 97,
    HistoricalTicksLast = 98,
    TickByTick = 99,
    OrderBound = 100,
    CompletedOrder = 101,
    CompletedOrdersEnd = 102,
    ReplaceFaEnd = 103,
    WshMetaData = 104,
    WshEventData = 105,
    HistoricalSchedule = 106,
    UserInfo = 107,
}

impl FromStr for IncomingMsgId {
    type Err = ParseIncomingMsgIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<i32>() {
            Ok(id) => match id {
                1 => Ok(Self::TickPrice),
                2 => Ok(Self::TickSize),
                3 => Ok(Self::OrderStatus),
                4 => Ok(Self::ErrorMsg),
                5 => Ok(Self::OpenOrder),
                6 => Ok(Self::AccountValue),
                7 => Ok(Self::PortfolioValue),
                8 => Ok(Self::AccountUpdateTime),
                9 => Ok(Self::NextValidId),
                10 => Ok(Self::ContractData),
                11 => Ok(Self::ExecutionData),
                12 => Ok(Self::MarketDepth),
                13 => Ok(Self::MarketDepthL2),
                14 => Ok(Self::NewsBulletins),
                15 => Ok(Self::ManagedAccounts),
                16 => Ok(Self::ReceiveFa),
                17 => Ok(Self::HistoricalData),
                18 => Ok(Self::BondContractData),
                19 => Ok(Self::ScannerParameters),
                20 => Ok(Self::ScannerData),
                21 => Ok(Self::TickOptionComputation),
                45 => Ok(Self::TickGeneric),
                46 => Ok(Self::TickString),
                47 => Ok(Self::TickEfp),
                49 => Ok(Self::CurrentTime),
                50 => Ok(Self::RealTimeBars),
                51 => Ok(Self::FundamentalData),
                52 => Ok(Self::ContractDataEnd),
                53 => Ok(Self::OpenOrderEnd),
                54 => Ok(Self::AccountDownloadEnd),
                55 => Ok(Self::ExecutionDataEnd),
                56 => Ok(Self::DeltaNeutralValidation),
                57 => Ok(Self::TickSnapshotEnd),
                58 => Ok(Self::MarketDataType),
                59 => Ok(Self::CommissionReport),
                61 => Ok(Self::PositionData),
                62 => Ok(Self::PositionEnd),
                63 => Ok(Self::AccountSummary),
                64 => Ok(Self::AccountSummaryEnd),
                65 => Ok(Self::VerifyMessageApi),
                66 => Ok(Self::VerifyCompleted),
                67 => Ok(Self::DisplayGroupList),
                68 => Ok(Self::DisplayGroupUpdated),
                69 => Ok(Self::VerifyAndAuthMessageApi),
                70 => Ok(Self::VerifyAndAuthCompleted),
                71 => Ok(Self::PositionMulti),
                72 => Ok(Self::PositionMultiEnd),
                73 => Ok(Self::AccountUpdateMulti),
                74 => Ok(Self::AccountUpdateMultiEnd),
                75 => Ok(Self::SecurityDefinitionOptionParameter),
                76 => Ok(Self::SecurityDefinitionOptionParameterEnd),
                77 => Ok(Self::SoftDollarTiers),
                78 => Ok(Self::FamilyCodes),
                79 => Ok(Self::SymbolSamples),
                80 => Ok(Self::MktDepthExchanges),
                81 => Ok(Self::TickReqParams),
                82 => Ok(Self::SmartComponents),
                83 => Ok(Self::NewsArticle),
                84 => Ok(Self::TickNews),
                85 => Ok(Self::NewsProviders),
                86 => Ok(Self::HistoricalNews),
                87 => Ok(Self::HistoricalNewsEnd),
                88 => Ok(Self::HeadTimestamp),
                89 => Ok(Self::HistogramData),
                90 => Ok(Self::HistoricalDataUpdate),
                91 => Ok(Self::RerouteMktDataReq),
                92 => Ok(Self::RerouteMktDepthReq),
                93 => Ok(Self::MarketRule),
                94 => Ok(Self::Pnl),
                95 => Ok(Self::PnlSingle),
                96 => Ok(Self::HistoricalTicks),
                97 => Ok(Self::HistoricalTicksBidAsk),
                98 => Ok(Self::HistoricalTicksLast),
                99 => Ok(Self::TickByTick),
                100 => Ok(Self::OrderBound),
                101 => Ok(Self::CompletedOrder),
                102 => Ok(Self::CompletedOrdersEnd),
                103 => Ok(Self::ReplaceFaEnd),
                104 => Ok(Self::WshMetaData),
                105 => Ok(Self::WshEventData),
                106 => Ok(Self::HistoricalSchedule),
                107 => Ok(Self::UserInfo),
                _ => Err(ParseIncomingMsgIdError),
            },
            Err(_) => Err(ParseIncomingMsgIdError),
        }
    }
}

#[derive(Debug)]
pub struct ParseIncomingMsgIdError;

impl std::fmt::Display for ParseIncomingMsgIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to parse IncomingMsgId")
    }
}

impl std::error::Error for ParseIncomingMsgIdError {}

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

pub trait MessageHandler: Send + Sync {
    fn managed_accounts(&self, message: ManagedAccounts) -> Result<()> {
        println!("{message:?}");
        Ok(())
    }

    fn account_summary(&self, account_summary: AccountSummary) -> Result<()> {
        println!("{account_summary:?}");
        Ok(())
    }

    fn account_summary_end(&self, account_summary_end: AccountSummaryEnd) -> Result<()> {
        println!("{account_summary_end:?}");
        Ok(())
    }

    fn account_update_time(&self, account_update_time: AccountUpdateTime) -> Result<()> {
        println!("{account_update_time:?}");
        Ok(())
    }

    fn account_value(&self, account_value: AccountValue) -> Result<()> {
        println!("{account_value:?}");
        Ok(())
    }

    fn account_download_end(&self, message: AccountDownloadEnd) -> Result<()> {
        println!("{message:?}");
        Ok(())
    }

    fn handle_message(&self, parts: Vec<&str>) -> Result<()> {
        let msg_id: IncomingMsgId = parts[0]
            .parse()
            .map_err(|e| anyhow::anyhow!("Failed to parse message ID: {}", e))?;

        match msg_id {
            IncomingMsgId::ManagedAccounts => {
                let message = MessageProcessor::parse_managed_accounts(&parts[1..])?;
                self.managed_accounts(message)
            }
            IncomingMsgId::AccountSummary => {
                let message = MessageProcessor::parse_account_summary(&parts[1..])?;
                self.account_summary(message)
            }
            IncomingMsgId::AccountSummaryEnd => {
                let message = MessageProcessor::parse_account_summary_end(&parts[1..])?;
                self.account_summary_end(message)
            }

            IncomingMsgId::AccountUpdateTime => {
                let message = MessageProcessor::parse_account_update_time(&parts[1..])?;
                self.account_update_time(message)
            }
            IncomingMsgId::AccountValue => {
                let message = MessageProcessor::parse_account_value(&parts[1..])?;
                self.account_value(message)
            }
            IncomingMsgId::AccountDownloadEnd => {
                let message = MessageProcessor::parse_account_download_end(&parts[1..])?;
                self.account_download_end(message)
            }
            _ => {
                println!("Received message: {:?}, parts: {:?}", msg_id, parts);
                Ok(())
            }
        }
    }
}
