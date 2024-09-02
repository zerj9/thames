use crate::incoming::{PositionData, PositionEnd};
use crate::MessageProcessor;
use anyhow::Result;
use std::num::ParseFloatError;

fn parse_optional_string(s: &str) -> Option<String> {
    if s.is_empty() {
        None
    } else {
        Some(s.to_string())
    }
}

fn parse_strike(s: &str) -> Result<Option<f64>, ParseFloatError> {
    let value = s.parse::<f64>()?;
    if value == 0.0 {
        Ok(None)
    } else {
        Ok(Some(value))
    }
}

impl MessageProcessor {
    pub(crate) fn parse_position_data(parts: &[&str]) -> Result<PositionData> {
        Ok(PositionData {
            account: parts[1].to_string(),
            con_id: parts[2].parse()?,
            symbol: parts[3].to_string(),
            sec_type: parts[4].to_string(),
            last_trade_date_or_contract_month: parse_optional_string(parts[5]),
            strike: parse_strike(parts[6])?,
            right: parse_optional_string(parts[7]),
            multiplier: parse_optional_string(parts[8]),
            exchange: parts[9].to_string(),
            currency: parts[10].to_string(),
            local_symbol: parts[11].to_string(),
            trading_class: parts[12].to_string(),
            position: parts[13].parse()?,
            avg_cost: parts[14].parse()?,
        })
    }

    pub(crate) fn parse_position_end(_parts: &[&str]) -> Result<PositionEnd> {
        Ok(PositionEnd {})
    }
}
