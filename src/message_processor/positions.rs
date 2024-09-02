use crate::{Contract, MessageProcessor, PositionData, PositionEnd};
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
            contract: Contract::builder()
                .con_id(parts[2].parse()?)
                .symbol(parts[3])
                .sec_type(parts[4])
                .last_trade_date_or_contract_month(
                    parse_optional_string(parts[5]).unwrap_or_default(),
                )
                .strike(parse_strike(parts[6])?.unwrap_or_default())
                .right(parse_optional_string(parts[7]).unwrap_or_default())
                .multiplier(parse_optional_string(parts[8]).unwrap_or_default())
                .exchange(parts[9])
                .currency(parts[10])
                .local_symbol(parts[11])
                .trading_class(parts[12])
                .build(),
            position: parts[13].parse()?,
            avg_cost: parts[14].parse()?,
        })
    }
    pub(crate) fn parse_position_end(_parts: &[&str]) -> Result<PositionEnd> {
        Ok(PositionEnd {})
    }
}
