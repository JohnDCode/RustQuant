/*

JDA Options Pricing
/src/binomial.rs
JohnDavid Abe

*/



// Packages
use statrs::distribution::{Normal, ContinuousCDF};



// Calculate d1
pub fn d1(spot: f64, strike: f64, time: f64, rate: f64, volatility: f64) -> f64 {
    return ((spot / strike).ln() + ((rate + (0.5 * volatility.powi(2))) * time)) / (volatility * time.sqrt());
}

// Calculate d2
pub fn d2(spot: f64, strike: f64, time: f64, rate: f64, volatility: f64) -> f64 {
    return d1(spot, strike, time, rate, volatility) - (volatility * time.sqrt());
}



// Take in option data and return the price based on the black-scholes model for pricing American and European options
pub fn black_scholes(spot: f64, strike: f64, time: f64, rate: f64, volatility: f64, call: bool) -> f64 {

    // Standard normal distribution and the distribution at particular points to use
    let standard_normal = Normal::standard();

    // Calculate d1 and d2
    let d1: f64 = d1(spot, strike, time, rate, volatility);
    let d2: f64 = d2(spot, strike, time, rate, volatility);

    // Apply the formula based on the type of option

    if call {
        return (((spot * standard_normal.cdf(d1)) - (strike * (-1.0 * rate * time).exp() * standard_normal.cdf(d2))) * 100.0).round() / 100.0;
    } else {
        return ((((strike * (-1.0 * rate * time).exp() * standard_normal.cdf(-1.0 * d2)) - (spot * standard_normal.cdf(-1.0 * d1))) * 100.0)).round() / 100.0;
    }
}

