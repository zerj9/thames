#[derive(Debug, Clone, PartialEq)]
pub struct Contract {
    pub con_id: i32,
    pub symbol: String,
    pub sec_type: String,
    pub last_trade_date_or_contract_month: String,
    pub last_trade_date: String,
    pub strike: f64,
    pub right: String,
    pub multiplier: String,
    pub exchange: String,
    pub currency: String,
    pub local_symbol: String,
    pub primary_exch: String,
    pub trading_class: String,
    pub include_expired: bool,
    pub sec_id_type: String,
    pub sec_id: String,
    pub description: String,
    pub issuer_id: String,
    pub combo_legs_description: String,
    pub combo_legs: String, // This should be changed to a more appropriate type later
    pub delta_neutral_contract: String, // This should be changed to a more appropriate type later
}

//impl Contract {
//    pub fn new() -> Self {
//        Default::default()
//    }
//}
