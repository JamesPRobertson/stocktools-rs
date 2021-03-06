// James Robertson
//

use std::io::prelude::*;
use std::fs::File;

/// Get JSON from str
///     Uses reqwest to call Alphavantage's API with the given
///     argument to determine the ticker.
///
/// Args:
///     ticker  (&str): the stock ticker desired e.g. GME, IBM
///     interval (i32): time interval for the data received, must
///                     adhere to the AlphaVantage options (1,5,15, etc.)
///
/// Returns:
///     reqwest::blocking::Response to be Stringified
///
pub fn get_json_from_ticker(ticker: &str, interval: i32) -> Result<reqwest::blocking::Response, ()>{
    let mut api_file = File::open("./alpha_vantage_key").unwrap();
    let mut api_key: String = String::new();
    api_file.read_to_string(&mut api_key).unwrap();

    let url = format!(
            "https://www.alphavantage.co/query?function=TIME_SERIES_INTRADAY&symbol={}&interval={}min&apikey={}",
            ticker,
            interval,
            api_key);
    let response = reqwest::blocking::get(url).unwrap();
    //println!("{:?}", response);
    Ok(response)
}

