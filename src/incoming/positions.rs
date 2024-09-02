#[derive(Debug, Clone, PartialEq)]
pub struct PositionData {
    pub account: String,
    pub con_id: i32,
    pub symbol: String,
    pub sec_type: String,
    pub last_trade_date_or_contract_month: Option<String>,
    pub strike: Option<f64>,
    pub right: Option<String>,
    pub multiplier: Option<String>,
    pub exchange: String,
    pub currency: String,
    pub local_symbol: String,
    pub trading_class: String,
    pub position: f64,
    pub avg_cost: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PositionEnd {}
