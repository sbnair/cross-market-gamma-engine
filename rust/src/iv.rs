use anyhow::Result;
use implied_vol::{DefaultSpecialFn, ImpliedBlackVolatility};

pub fn black76_iv(
    option_price: f64,
    forward: f64,
    strike: f64,
    time_to_expiry: f64,
    is_call: bool,
) -> Result<f64> {
    let iv = ImpliedBlackVolatility::builder()
        .option_price(option_price)
        .forward(forward)
        .strike(strike)
        .expiry(time_to_expiry)
        .is_call(is_call)
        .build()?
        .calculate::<DefaultSpecialFn>()?;
    Ok(iv)
}
