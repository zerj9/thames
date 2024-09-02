// TODO: Optionals
#[derive(Debug, Clone, Default, PartialEq)]
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
    pub combo_legs: String, // TODO: This should be changed to a more appropriate type later
    pub delta_neutral_contract: String, // TODO: This should be changed to a more appropriate type later
}

impl Contract {
    pub fn builder() -> ContractBuilder {
        ContractBuilder::default()
    }
}

#[derive(Default)]
pub struct ContractBuilder {
    con_id: Option<i32>,
    symbol: Option<String>,
    sec_type: Option<String>,
    last_trade_date_or_contract_month: Option<String>,
    last_trade_date: Option<String>,
    strike: Option<f64>,
    right: Option<String>,
    multiplier: Option<String>,
    exchange: Option<String>,
    currency: Option<String>,
    local_symbol: Option<String>,
    primary_exch: Option<String>,
    trading_class: Option<String>,
    include_expired: Option<bool>,
    sec_id_type: Option<String>,
    sec_id: Option<String>,
    description: Option<String>,
    issuer_id: Option<String>,
    combo_legs_description: Option<String>,
    combo_legs: Option<String>,
    delta_neutral_contract: Option<String>,
}

impl ContractBuilder {
    pub fn con_id(mut self, con_id: i32) -> Self {
        self.con_id = Some(con_id);
        self
    }

    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }

    pub fn sec_type(mut self, sec_type: impl Into<String>) -> Self {
        self.sec_type = Some(sec_type.into());
        self
    }

    pub fn last_trade_date_or_contract_month(mut self, date: impl Into<String>) -> Self {
        self.last_trade_date_or_contract_month = Some(date.into());
        self
    }

    pub fn last_trade_date(mut self, date: impl Into<String>) -> Self {
        self.last_trade_date = Some(date.into());
        self
    }

    pub fn strike(mut self, strike: f64) -> Self {
        self.strike = Some(strike);
        self
    }

    pub fn right(mut self, right: impl Into<String>) -> Self {
        self.right = Some(right.into());
        self
    }

    pub fn multiplier(mut self, multiplier: impl Into<String>) -> Self {
        self.multiplier = Some(multiplier.into());
        self
    }

    pub fn exchange(mut self, exchange: impl Into<String>) -> Self {
        self.exchange = Some(exchange.into());
        self
    }

    pub fn currency(mut self, currency: impl Into<String>) -> Self {
        self.currency = Some(currency.into());
        self
    }

    pub fn local_symbol(mut self, local_symbol: impl Into<String>) -> Self {
        self.local_symbol = Some(local_symbol.into());
        self
    }

    pub fn primary_exch(mut self, primary_exch: impl Into<String>) -> Self {
        self.primary_exch = Some(primary_exch.into());
        self
    }

    pub fn trading_class(mut self, trading_class: impl Into<String>) -> Self {
        self.trading_class = Some(trading_class.into());
        self
    }

    pub fn include_expired(mut self, include_expired: bool) -> Self {
        self.include_expired = Some(include_expired);
        self
    }

    pub fn sec_id_type(mut self, sec_id_type: impl Into<String>) -> Self {
        self.sec_id_type = Some(sec_id_type.into());
        self
    }

    pub fn sec_id(mut self, sec_id: impl Into<String>) -> Self {
        self.sec_id = Some(sec_id.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn issuer_id(mut self, issuer_id: impl Into<String>) -> Self {
        self.issuer_id = Some(issuer_id.into());
        self
    }

    pub fn combo_legs_description(mut self, combo_legs_description: impl Into<String>) -> Self {
        self.combo_legs_description = Some(combo_legs_description.into());
        self
    }

    pub fn combo_legs(mut self, combo_legs: impl Into<String>) -> Self {
        self.combo_legs = Some(combo_legs.into());
        self
    }

    pub fn delta_neutral_contract(mut self, delta_neutral_contract: impl Into<String>) -> Self {
        self.delta_neutral_contract = Some(delta_neutral_contract.into());
        self
    }

    pub fn build(self) -> Contract {
        Contract {
            con_id: self.con_id.unwrap_or_default(),
            symbol: self.symbol.unwrap_or_default(),
            sec_type: self.sec_type.unwrap_or_default(),
            last_trade_date_or_contract_month: self
                .last_trade_date_or_contract_month
                .unwrap_or_default(),
            last_trade_date: self.last_trade_date.unwrap_or_default(),
            strike: self.strike.unwrap_or_default(),
            right: self.right.unwrap_or_default(),
            multiplier: self.multiplier.unwrap_or_default(),
            exchange: self.exchange.unwrap_or_default(),
            currency: self.currency.unwrap_or_default(),
            local_symbol: self.local_symbol.unwrap_or_default(),
            primary_exch: self.primary_exch.unwrap_or_default(),
            trading_class: self.trading_class.unwrap_or_default(),
            include_expired: self.include_expired.unwrap_or_default(),
            sec_id_type: self.sec_id_type.unwrap_or_default(),
            sec_id: self.sec_id.unwrap_or_default(),
            description: self.description.unwrap_or_default(),
            issuer_id: self.issuer_id.unwrap_or_default(),
            combo_legs_description: self.combo_legs_description.unwrap_or_default(),
            combo_legs: self.combo_legs.unwrap_or_default(),
            delta_neutral_contract: self.delta_neutral_contract.unwrap_or_default(),
        }
    }
}
