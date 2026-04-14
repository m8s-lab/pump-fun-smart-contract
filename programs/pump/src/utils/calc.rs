use std::ops::{Div, Mul};

/// Convert to integer scaled value using u128 for precision
pub fn to_integer_scaled(value: u64, decimals: u8) -> u128 {
    (value as u128) * (10_u128.pow(decimals as u32))
}

/// Convert from integer scaled value back to smallest unit
pub fn from_integer_scaled(scaled_value: u128, decimals: u8) -> u64 {
    let divisor = 10_u128.pow(decimals as u32);
    (scaled_value / divisor) as u64
}

/// OLD: Float-based conversion (kept for display only - DO NOT use for financial calc)
pub fn convert_to_float(value: u64, decimals: u8) -> f64 {
    (value as f64).div(f64::powf(10.0, decimals as f64))
}

pub fn convert_from_float(value: f64, decimals: u8) -> u64 {
    value.mul(f64::powf(10.0, decimals as f64)) as u64
}
