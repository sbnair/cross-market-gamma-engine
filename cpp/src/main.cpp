#include <iostream>
#include "pricing.hpp"
#include "execution.hpp"

int main() {
    const double F = 20500.0;
    const double K = 20500.0;
    const double sigma = 0.22;
    const double T = 2.0 / 365.0;
    const double discount = 1.0;
    const double oi = 12000.0;
    const double multiplier = 20.0;

    const double gex = quant::gamma_exposure(F, K, sigma, T, discount, oi, multiplier);
    auto decision = decide(4.2, 0.8, 12.0, 0.15, 0.40);

    std::cout << "gex=" << gex << "\n";
    std::cout << "cross_now=" << decision.cross_now << "\n";
    std::cout << "join_bid=" << decision.join_bid << "\n";
    std::cout << "join_ask=" << decision.join_ask << "\n";
    return 0;
}
