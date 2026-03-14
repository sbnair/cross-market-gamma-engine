use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Venue {
    Deribit,
    Cme,
    Other,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AssetClass {
    Crypto,
    EquityIndex,
    Fx,
    Commodity,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum OptionRight {
    Call,
    Put,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionInstrument {
    pub venue: Venue,
    pub asset_class: AssetClass,
    pub symbol: String,
    pub underlying: String,
    pub expiry_ts_ns: i64,
    pub strike: f64,
    pub right: OptionRight,
    pub contract_multiplier: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Bbo {
    pub bid_px: f64,
    pub bid_sz: f64,
    pub ask_px: f64,
    pub ask_sz: f64,
    pub ts_ns: i64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GreekSnapshot {
    pub delta: f64,
    pub gamma: f64,
    pub vega: f64,
    pub theta: f64,
    pub iv: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct FlowSignal {
    pub local_gex: f64,
    pub d_local_gex_dp: f64,
    pub iv_shock_z: f64,
    pub book_imbalance: f64,
    pub trade_aggr_imbalance: f64,
}
