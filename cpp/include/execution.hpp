#pragma once

struct ExecDecision {
    bool join_bid{false};
    bool join_ask{false};
    bool cross_now{false};
    double limit_px{0.0};
    int qty{0};
};

ExecDecision decide(double blast_score,
                    double short_horizon_alpha,
                    double queue_ahead,
                    double adverse_selection_cost_ticks,
                    double expected_fill_edge_ticks);
