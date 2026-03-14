#include "execution.hpp"

ExecDecision decide(double blast_score,
                    double short_horizon_alpha,
                    double queue_ahead,
                    double adverse_selection_cost_ticks,
                    double expected_fill_edge_ticks) {
    ExecDecision d{};

    if (blast_score > 3.0 &&
        short_horizon_alpha > 0.0 &&
        expected_fill_edge_ticks > adverse_selection_cost_ticks) {
        d.cross_now = true;
        d.qty = 1;
        return d;
    }

    if (short_horizon_alpha > 0.0 && queue_ahead < 20.0) {
        d.join_bid = true;
        d.qty = 1;
        return d;
    }

    if (short_horizon_alpha < 0.0 && queue_ahead < 20.0) {
        d.join_ask = true;
        d.qty = 1;
        return d;
    }

    return d;
}
