/*

JDA Options Pricing
/src/greeks.rs
JohnDavid Abe

*/



// Packages
use std::error::Error;
use statrs::distribution::{Normal, ContinuousCDF, Continuous};

// Modules
use crate::black_scholes::{d1, d2};



// Struct to hold data on the five greeks
#[derive(Debug)]
pub struct Greeks {
    pub delta: f64,
    pub gamma: f64,
    pub vega: f64,
    pub theta: f64,
    pub rho: f64,
}

// Calculate the five greeks
pub fn calculate_greeks(spot: f64, strike: f64, time: f64, rate: f64, volatility: f64, call: bool) -> Result<Greeks, Box<dyn Error>> {

    // Standard normal distribution
    let standard_normal = Normal::standard();

    // Calculate the probability density function (PDF) and cumulative distribution function (CDF) at particular points to use in calculations
    let d1: f64 = d1(spot, strike, time, rate, volatility);
    let d2: f64 = d2(spot, strike, time, rate, volatility);

    let cdfd1: f64 = standard_normal.cdf(d1);
    let pdfd1: f64 = standard_normal.pdf(d1);

    let cdfd2: f64 = standard_normal.cdf(d2);
    let cdfnegd2: f64 = standard_normal.cdf(-1.0 * d2);

    // Calculate delta
    let mut delta: f64 = cdfd1;
    if !call { delta -= 1.0; }

    // Calculate gamma
    let gamma: f64 =  pdfd1 / (spot * volatility * time.sqrt());

    // Calculate vega
    let vega: f64 = spot * pdfd1 * time.sqrt();

    // Calculate theta
    let mut theta: f64 = (-1.0 * spot * pdfd1 * volatility) / (2.0 * time.sqrt());
    if call {
        theta -= rate * strike * (-1.0 * rate * time).exp() * cdfd2;
    } else {
        theta += rate * strike * (-1.0 * rate * time).exp() * cdfnegd2;
    }

    // Calculate rho
    let rho: f64;
    if call {
        rho = strike * time * (-1.0 * rate * time).exp() * cdfd2;
    } else {
        rho = -1.0 * strike * time * (-1.0 * rate * time).exp() * cdfnegd2;
    }

    // Return the wrapped data
    Ok(Greeks {
        delta,
        gamma,
        vega,
        theta,
        rho,
    })
}

