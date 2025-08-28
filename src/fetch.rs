/*

JDA Options Pricing
/src/fetch.rs
JohnDavid Abe

*/



// Packages
use reqwest::Client;
use chrono::{NaiveDate, Utc};
use std::error::Error;



// Struct to hold data on a chosen option from the chain
#[derive(Debug)]
pub struct OptionData {
    pub symbol: String,
    pub spot: f64,
    pub strike: f64,
    pub expiration: f64,
    pub volatility: f64,
    pub rate: f64,
}

// Fetch expiration dates from the option chain for a particular symbol
pub async fn fetch_expiration_dates(symbol: &str, call: bool) -> Result<Vec<NaiveDate>, Box<dyn Error>> {

    // Make request to the options chain
    let client = Client::new();
    let side = if call { "call" } else { "put" };
    let url = format!("https://api.marketdata.app/v1/options/expirations/{}?side={}", symbol, side);
    let resp = client.get(&url).send().await?;
    let json: serde_json::Value = resp.json().await?;

    // Return expirations as a list of NaiveDate objects
    let expirations = json["expirations"]
        .as_array()
        .ok_or("Invalid expiration data")?
        .iter()
        .filter_map(|d| d.as_str())
        .filter_map(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok())
        .collect();

    Ok(expirations)
}



// Fetch option data given an expiration date and a symbol and a type, choosing the option from the options chain with the closest strike price to the target_strike
pub async fn fetch_american_option_data(symbol: &str, target_strike: f64, expiration: NaiveDate, call: bool) -> Result<OptionData, Box<dyn Error>> {

    // Make the request to the options chain
    let client = Client::new();
    let side = if call { "call" } else { "put" };
    let url = format!(
        "https://api.marketdata.app/v1/options/chain/{}?side={}&expiration={}",
        symbol, side, expiration
    );
    let resp = client.get(&url).send().await?;
    let json: serde_json::Value = resp.json().await?;


    // Get all strike price data
    let strike_prices = json["strike"]
        .as_array()
        .ok_or("Missing options strike data")?;


    // Find the option w/ the strike price closest to the user inputted strike price
    let mut option_index: u32 = 0;
    let mut min_strike_diff: f64 = ((strike_prices[0 as usize]).as_f64().unwrap() - target_strike).abs();
    let mut current_diff: f64 = 0.0;

    // Loop through all the strike prices
    for i in 1..strike_prices.len() {
        current_diff = ((strike_prices[i as usize]).as_f64().unwrap() - target_strike).abs();
        
        // Find the minimum difference between desired strike and each contract's strike
        if current_diff < min_strike_diff {
            min_strike_diff = current_diff;
            option_index = i as u32;
        }
    }


    // Get the strike from the closest option
    let option_strike: f64 = (strike_prices[option_index as usize]).as_f64().unwrap();


    // Get the implied volatility (iv) of the option selected
    let ivs = json["iv"]
        .as_array()
        .ok_or("Missing options iv data")?;

    let option_iv: f64 = (ivs[option_index as usize]).as_f64().unwrap();


    // Get spot price
    let quote_url = format!("https://api.marketdata.app/v1/stocks/quotes/{}", symbol);
    let quote_json: serde_json::Value = client.get(&quote_url).send().await?.json().await?;
    let wrapped_mid = quote_json["last"]
        .as_array()
        .ok_or("Missing spot/last price")?;

    let mid_price: f64 = (wrapped_mid[0 as usize]).as_f64().unwrap();



    // Get the years to expiration of the option
    let today = Utc::now().date_naive();
    let days_until = (expiration - today).num_days();
    let expiry: f64 = (days_until as f64) / 365.0;


    // Select the US treasury risk free interest rate with the maturity time closest matching the expiration of the option
    let mut rates: Vec<(f64, String)> = Vec::new();
    rates.push((0.125, "GS1M".to_string()));
    rates.push((0.25, "GS3M".to_string()));
    rates.push((0.5, "GS6M".to_string()));
    rates.push((1.0, "GS1".to_string()));
    rates.push((2.0, "GS2".to_string()));
    rates.push((3.0, "GS3".to_string()));
    rates.push((5.0, "GS5".to_string()));
    rates.push((7.0, "GS7".to_string()));
    rates.push((10.0, "GS10".to_string()));
    rates.push((20.0, "GS20".to_string()));
    rates.push((30.0, "GS30".to_string()));

    let mut closest_rate = &rates[0 as usize];

    for i in 1..rates.len() {
        if ((rates[i as usize].0) - expiry).abs() < (closest_rate.0 - expiry).abs() {
            closest_rate = &rates[i as usize];
        }
    }

    // Pull the according risk free rate from US Treasury via FRED API
    let fred_url = format!("https://api.stlouisfed.org/fred/series/observations?series_id={}&api_key=730334457025754885efeced5149e476&file_type=json&sort_order=desc&limit=1", closest_rate.1);
    let fred_json: serde_json::Value = client.get(&fred_url).send().await?.json().await?;
    let wrapped_rate = fred_json["observations"]
        .as_array()
        .ok_or("Missing spot/mid price")?;

    let rate_str = wrapped_rate.first().unwrap()["value"].as_str().unwrap();
    let rate: f64 = rate_str.parse().unwrap();

    // Return the wrapped data
    Ok(OptionData {
        symbol: symbol.to_string(),
        spot: mid_price,
        strike: option_strike,
        expiration: expiry,
        volatility: option_iv,
        rate: rate / 100.0,
    })
}

