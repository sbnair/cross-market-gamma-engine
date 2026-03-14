use crate::types::Bbo;

impl Bbo {
    pub fn raw_mid(&self) -> Option<f64> {
        if self.bid_px > 0.0 && self.ask_px > self.bid_px {
            Some((self.bid_px + self.ask_px) * 0.5)
        } else {
            None
        }
    }

    pub fn microprice(&self) -> Option<f64> {
        let denom = self.bid_sz + self.ask_sz;
        if self.bid_px > 0.0 && self.ask_px > self.bid_px && denom > 0.0 {
            Some((self.ask_px * self.bid_sz + self.bid_px * self.ask_sz) / denom)
        } else {
            None
        }
    }

    pub fn spread(&self) -> Option<f64> {
        if self.ask_px > self.bid_px && self.bid_px > 0.0 {
            Some(self.ask_px - self.bid_px)
        } else {
            None
        }
    }
}

pub fn fair_option_price(bbo: &Bbo) -> Option<f64> {
    match (bbo.microprice(), bbo.raw_mid()) {
        (Some(mp), Some(mid)) => Some(0.7 * mp + 0.3 * mid),
        (Some(mp), None) => Some(mp),
        (None, Some(mid)) => Some(mid),
        _ => None,
    }
}
