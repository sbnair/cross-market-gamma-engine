pub fn gamma_exposure(
    gamma: f64,
    open_interest: f64,
    contract_multiplier: f64,
    forward: f64,
) -> f64 {
    gamma * open_interest * contract_multiplier * forward * forward
}

pub fn local_gex_gradient(prev: f64, next: f64, dp: f64) -> f64 {
    if dp.abs() < 1e-12 { 0.0 } else { (next - prev) / dp }
}
