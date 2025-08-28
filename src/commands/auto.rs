/*

JDA Options Pricing
/src/commands/auto.rs
JohnDavid Abe

*/


// Fetch data module
use dialoguer::Select;
use crate::fetch::{fetch_expiration_dates, fetch_american_option_data};
use crate::binomial::binomial;
use crate::greeks::calculate_greeks;


// Entry point for the command
pub async fn run(symbol: String, strike: f64, steps: u32, call: bool, put: bool, greeks: bool) {


    // Ensure option is either only call OR put
    let mut call_opt: bool = call;

    // If neither flag has been specified, default to call
    if !call && !put { call_opt = true; } else if call && put { 
        // Check for double call/put flags in arguments
        eprintln!("ERROR: Ambigious arguments, only specify a single option type.");
        return;
    }



    // Get valid expiration dates for the symbol from the option chain
    match fetch_expiration_dates(&symbol, call).await {

        Ok(dates) => {

            // Get the user to select the expiration date from drop down
            let items = dates;
            let selection = Select::new()
                .with_prompt("Choose an options expiration date")
                .items(&items)
                .default(0)
                .interact()
                .unwrap();


            // Pull the option data
            match fetch_american_option_data(&symbol, strike, items[selection], call).await {
                Ok(data) => {
                    
                        // Output
                        println!("\u{1F4B0} Auto Options Pricing Tool");
                        println!("--------------------------------\n");
                        
                        println!("Symbol:             {}", symbol);
                        println!("Option Type:        {}", if call_opt { "Call" } else { "Put" } );
                        println!("Spot Price:         {}", data.spot);
                        println!("Strike Price:       {}", data.strike);
                        println!("Years to Maturity:  {}", data.expiration);
                        println!("Risk-Free Rate:     {}", data.rate);
                        println!("Volatility:         {}", data.volatility);
                        println!("Steps:              {}", steps);

                        println!("\n--------------------------------\n");

                        // Price using American Binomial Model
                        println!("Option Price using Binomial Model:       {}", binomial(data.spot, data.strike, data.expiration, data.rate, data.volatility, steps, call_opt, true));


                        // Attempt to get the Greeks if the flag is specified
                        if greeks {
                            match calculate_greeks(data.spot, data.strike, data.expiration, data.rate, data.volatility, call_opt) {
                                Ok(greeks) => {
                                    
                                        // Output
                                        println!("\n--------------------------------\n");
                                        
                                        println!("Delta:              {}", (greeks.delta * 10000.0).round() / 10000.0);
                                        println!("Gamma:              {}", (greeks.gamma * 10000.0).round() / 10000.0);
                                        println!("Vega:               {}", (greeks.vega * 10000.0).round() / 10000.0);
                                        println!("Theta:              {}", (greeks.theta * 10000.0).round() / 10000.0);
                                        println!("Rho:                {}", (greeks.rho * 10000.0).round() / 10000.0);


                                }
                                Err(e) => eprintln!("Failed to calculate the greeks: {}", e),
                            }
                        }
                }
                Err(e) => eprintln!("Failed to fetch option data: {}", e),
            }
        }
        Err(e) => eprintln!("Failed to fetch expiration dates: {}", e),
    }
}

