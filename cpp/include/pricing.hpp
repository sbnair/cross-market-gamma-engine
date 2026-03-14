#pragma once
#include <cmath>
#include <algorithm>

namespace quant {

inline double norm_pdf(double x) {
    static constexpr double inv_sqrt_2pi = 0.3989422804014327;
    return inv_sqrt_2pi * std::exp(-0.5 * x * x);
}

double norm_cdf(double x);
double d1(double F, double K, double sigma, double T);
double d2(double F, double K, double sigma, double T);
double gamma_black76(double F, double K, double sigma, double T, double discount);
double gamma_exposure(double F, double K, double sigma, double T, double discount, double oi, double multiplier);

} // namespace quant
