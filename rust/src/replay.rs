use crate::fair_pricing::fair_option_price;
use crate::signal::{classify_regime, gamma_blast_score};
use crate::types::{Bbo, FlowSignal};

#[derive(Debug, Clone)]
pub struct ReplayEvent {
    pub symbol: String,
    pub bbo: Bbo,
    pub local_gex: f64,
    pub d_local_gex_dp: f64,
    pub iv_shock_z: f64,
    pub book_imbalance: f64,
    pub trade_aggr_imbalance: f64,
}

#[derive(Debug, Clone)]
pub struct ReplayOutput {
    pub symbol: String,
    pub fair_px: f64,
    pub blast_score: f64,
    pub regime: String,
}

pub fn run_replay(events: &[ReplayEvent]) -> Vec<ReplayOutput> {
    events.iter().map(|e| {
        let fair_px = fair_option_price(&e.bbo).unwrap_or(0.0);
        let signal = FlowSignal {
            local_gex: e.local_gex,
            d_local_gex_dp: e.d_local_gex_dp,
            iv_shock_z: e.iv_shock_z,
            book_imbalance: e.book_imbalance,
            trade_aggr_imbalance: e.trade_aggr_imbalance,
        };
        ReplayOutput {
            symbol: e.symbol.clone(),
            fair_px,
            blast_score: gamma_blast_score(&signal),
            regime: classify_regime(signal.local_gex, signal.d_local_gex_dp).to_string(),
        }
    }).collect()
}
