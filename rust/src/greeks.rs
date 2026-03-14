use crate::types::GreekSnapshot;
use std::f64::consts::PI;

fn norm_pdf(x: f64) -> f64 {
    (1.0 / (2.0 * PI).sqrt()) * (-0.5 * x * x).exp()
}

fn norm_cdf(x: f64) -> f64 {
    0.5 * (1.0 + statrs_approx::erf(x / 2.0_f64.sqrt()))
}

fn d1(forward: f64, strike: f64, sigma: f64, t: f64) -> f64 {
    ((forward / strike).ln() + 0.5 * sigma * sigma * t) / (sigma * t.sqrt()).max(1e-12)
}

fn d2(forward: f64, strike: f64, sigma: f64, t: f64) -> f64 {
    d1(forward, strike, sigma, t) - sigma * t.sqrt()
}

pub fn black76_greeks(forward: f64, strike: f64, sigma: f64, t: f64, discount: f64, is_call: bool) -> GreekSnapshot {
    let x1 = d1(forward, strike, sigma, t);
    let x2 = d2(forward, strike, sigma, t);
    let nd1 = norm_pdf(x1);

    let delta = if is_call {
        discount * norm_cdf(x1)
    } else {
        discount * (norm_cdf(x1) - 1.0)
    };

    let gamma = discount * nd1 / (forward * sigma * t.sqrt()).max(1e-12);
    let vega = discount * forward * nd1 * t.sqrt();

    let theta = -0.5 * discount * forward * nd1 * sigma / t.sqrt().max(1e-12)
        + if is_call {
            0.0_f64.max(-discount * strike * 0.0 * norm_cdf(x2))
        } else {
            0.0_f64.max(discount * strike * 0.0 * norm_cdf(-x2))
        };

    GreekSnapshot { delta, gamma, vega, theta, iv: sigma }
}

mod statrs_approx {
    pub fn erf(x: f64) -> f64 {
        let t = 1.0 / (1.0 + 0.3275911 * x.abs());
        let a1 = 0.254829592;
        let a2 = -0.284496736;
        let a3 = 1.421413741;
        let a4 = -1.453152027;
        let a5 = 1.061405429;
        let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * (-x * x).exp();
        if x >= 0.0 { y } else { -y }
    }
}
