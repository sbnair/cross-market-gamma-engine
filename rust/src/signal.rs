use crate::types::FlowSignal;

pub fn gamma_blast_score(s: &FlowSignal) -> f64 {
    0.35 * s.d_local_gex_dp.abs()
        + 0.25 * s.iv_shock_z.abs()
        + 0.20 * s.book_imbalance.abs()
        + 0.20 * s.trade_aggr_imbalance.abs()
}

pub fn classify_regime(local_gex: f64, d_local_gex_dp: f64) -> &'static str {
    if local_gex > 0.0 && d_local_gex_dp.abs() < 1e5 {
        "positive_gamma"
    } else if local_gex < 0.0 && d_local_gex_dp.abs() >= 1e5 {
        "negative_gamma_acceleration"
    } else if local_gex < 0.0 {
        "negative_gamma"
    } else {
        "neutral"
    }
}
