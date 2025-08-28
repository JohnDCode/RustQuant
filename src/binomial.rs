/*

JDA Options Pricing
/src/binomial.rs
JohnDavid Abe

*/



// Take in option data and return the price based on the binomial model for pricing American or European options
pub fn binomial(spot: f64, strike: f64, time: f64, rate: f64, volatility: f64, steps: u32, call: bool, american: bool) -> f64 {
    
    // Calculate step size
    let dt: f64 = time / (steps as f64);

    // Calculate up/down factors
    let u: f64 = (volatility * f64::sqrt(dt)).exp();
    let d: f64 = 1.0 / u;

    // Calculate risk neutral pseudo probability of an up move
    let p: f64 = ((rate * dt).exp() - d) / (u - d);

    // Ensure p is valid
    if p > 1.0 || p < 0.0 { 
        eprintln!("ERROR: Incorrect arguments, pseudo up move probability invalid: {}", p);
        return 1.0;
    }

    // Vector that holds all possible option payoffs (intrinsic values)
    let mut intrinsic_values = Vec::with_capacity((steps + 1) as usize);


    // Simulate option prices at maturity
        // Each price at the end of the binomial tree is Spot * u^x * d^(steps-x) because there are steps + 1 number possible end prices
    for i in 0..=steps {

        // The price of the stock
        let price: f64 = spot * u.powi(i as i32) * d.powi((steps - i) as i32);

        // Calculate the intrinsic value of the option based on type
        let intrinsic: f64 = if call { (price-strike).max(0.0) } else { (strike-price).max(0.0) };

        intrinsic_values.push(intrinsic);
    }


    // Work backwards to discount option price back towards today

    // Loop backwards from final node - 1 to present
    for step in (0..steps).rev() {

        // Loop across the nodes at this time step (there are step nodes at each time step)
        for i in 0..=step {

            // Calculate the price of the stock at the node
            let current_price: f64 = spot * u.powi(i as i32) * d.powi((step - i) as i32);

            // Calculate the expected value of the option
                // Uses probability of an up/down move
            let expected_value = (-rate * dt).exp() * (p * intrinsic_values[(i + 1) as usize] + (1.0 - p) * intrinsic_values[i as usize]);
            
            
            // Handle american options, which can be exercised early
                // Option is worth the early exercise price if it is greater than the price it is expected to mature to

            // The early exercise intrinsic value
            let early_exercise = if call { (current_price - strike).max(0.0) } else { (strike - current_price).max(0.0) };

            // Take the maximum between value it is expected to mature to and the current price (if it were to be early exercised) for American options
            // For European options, early exercise is not applicable

            if american {
                intrinsic_values[i as usize] = expected_value.max(early_exercise);
            } else {
                intrinsic_values[i as usize] = expected_value;
            }
        }
    }


    // Return the option price
    return (intrinsic_values[0 as usize] * 100.0).round() / 100.0;
    
}

