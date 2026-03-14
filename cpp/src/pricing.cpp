#include "pricing.hpp"

namespace quant {

double norm_cdf(double x) {
    return 0.5 * std::erfc(-x / std::sqrt(2.0));
}

double d1(double F, double K, double sigma, double T) {
    const double vsqrt = sigma * std::sqrt(T);
    return (std::log(F / K) + 0.5 * sigma * sigma * T) / std::max(vsqrt, 1e-12);
}

double d2(double F, double K, double sigma, double T) {
    return d1(F, K, sigma, T) - sigma * std::sqrt(T);
}

double gamma_black76(double F, double K, double sigma, double T, double discount) {
    const double x = d1(F, K, sigma, T);
    return discount * norm_pdf(x) / std::max(F * sigma * std::sqrt(T), 1e-12);
}

double gamma_exposure(double F, double K, double sigma, double T, double discount, double oi, double multiplier) {
    return gamma_black76(F, K, sigma, T, discount) * oi * multiplier * F * F;
}

} // namespace quant
