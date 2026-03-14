mod types;
mod fair_pricing;
mod iv;
mod greeks;
mod gex;
mod signal;
mod deribit;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("cross-market-gamma-engine bootstrap");
    deribit::run_public_demo().await?;
    Ok(())
}
