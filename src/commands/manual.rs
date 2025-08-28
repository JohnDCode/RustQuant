/*

JDA Options Pricing
/src/commands/manual.rs
JohnDavid Abe

*/


use crate::binomial::binomial;
use crate::black_scholes::black_scholes;
use crate::greeks::{calculate_greeks};


// Entry point for the command
pub fn run(spot: f64, strike: f64, time: f64, rate: f64, volatility: f64, steps: u32, call: bool, put: bool, american: bool, european: bool, greeks: bool) {

    // Ensure option is either only call OR put
    let mut call_opt: bool = call;

    // If neither flag has been specified, default to call
    if !call && !put { call_opt = true; } else if call && put { 
        // Check for double call/put flags in arguments
        eprintln!("ERROR: Ambigious arguments, only specify a single option type.");
        return;
    }

    // Ensure option is either only American OR European
    let mut american_opt: bool = american;

    // If neither flag has been specified, default to call
    if !american && !european { american_opt = true; } else if american && european { 
        // Check for double call/put flags in arguments
        eprintln!("ERROR: Ambigious arguments, only specify a single option region.");
        return;
    }


    // Output
    println!("\u{1F4B0} Manual Options Pricing Tool");
    println!("--------------------------------\n");

    println!("Option Type:        {}", if call_opt { "Call" } else { "Put" } );
    println!("Spot Price:         {}", spot);
    println!("Strike Price:       {}", strike);
    println!("Years to Maturity:  {}", time);
    println!("Risk-Free Rate:     {}", rate);
    println!("Volatility:         {}", volatility);
    println!("Steps:              {}", steps);

    println!("\n--------------------------------\n");

    println!("Option Price using Binomial Model:            {}", binomial(spot, strike, time, rate, volatility, steps, call_opt, american_opt));
    if !american_opt { println!("Option Price using Black-Scholes Model:       {}", black_scholes(spot, strike, time, rate, volatility, call_opt)); }

    // Output Greeks if specified in flag
    if greeks {

        let greeks = calculate_greeks(spot, strike, time, rate, volatility, call_opt).unwrap();
                
        // Output
        println!("\n--------------------------------\n");
        
        println!("Delta:              {}", (greeks.delta * 10000.0).round() / 10000.0);
        println!("Gamma:              {}", (greeks.gamma * 10000.0).round() / 10000.0);
        println!("Vega:               {}", (greeks.vega * 10000.0).round() / 10000.0);
        println!("Theta:              {}", (greeks.theta * 10000.0).round() / 10000.0);
        println!("Rho:                {}", (greeks.rho * 10000.0).round() / 10000.0);

    }
}

