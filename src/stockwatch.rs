// James Robertson
//

use rand::Rng;

use std::{thread, time};
use std::io::{self, Write};
use std::collections::VecDeque;

use termion::{cursor, color};

use crate::network;
use crate::json_lib;


const SHORT_DELAY:time::Duration = time::Duration::from_millis(150);
const DELAY:time::Duration = time::Duration::from_millis(250);
const NUM_VALUES: usize = 17;
const TITLE: &str = "S T O C K W A T C H";
const _TIME_INTERVAL: i32 = 5;
const BLOCK: &str = "■";

struct TickerData<'a>{
    ticker: &'a String,
    most_recent_value: f64,
    values: VecDeque<String>
}

// OOo OOo! Consider displaying the price temporarily
// in green or red, then overwriting it in White after
// a small delay, would be cool! WILL have to make
// it async!

/// StockWatch Main
///     Main loop for the program, called from main.rs
///
/// Args: None
///
/// Returns: None
///
pub fn stockwatch_main(){
    let args:Vec<String> = std::env::args().collect();
    let mut lines: Vec<TickerData> = Vec::new();

    // Set everything up for the title
    print!("{}{}{}{title}",
        termion::clear::All, cursor::Goto(1,1),
        cursor::Hide,
        title=TITLE);
    io::stdout().flush().unwrap();
    thread::sleep(DELAY);

    // Set up the ticker's headers
    for i in 2..args.len(){
        print_header(&args[i], i - 1);
        let ticker_struct = TickerData{
            //most_recent_value: get_most_recent_close(&args[i]),
            ticker: &args[i],
            most_recent_value: get_fake_close(&args[i]),
            values: VecDeque::new()
        };
        lines.push(ticker_struct);
        thread::sleep(DELAY)
    }

    loop {
        for i in 0..lines.len() {
            let current_num = get_fake_close(lines[i].ticker.as_str());
            println!("{}{:.3}   ", cursor::Goto(7, ((i + 1) * 3) as u16), current_num);

            // Update the data in the struct
            push_data(&mut lines[i], current_num);
            lines[i].most_recent_value = current_num;

            // If we have too many values, pop off the last one
            if lines[i].values.len() > NUM_VALUES{
                lines[i].values.pop_back();
            }

            print!("\r[");
            for block in &lines[i].values{
                print!("{}", block);
            }
            io::stdout().flush().unwrap();
        }
        //thread::sleep(DELAY);
        thread::sleep(SHORT_DELAY);
    }
} 

// TODO: consider formatting the '-' correctly for 3 letter tickers
//       to fit with 4 letter tickers nicely
/// Print Header
///     Prints the given ticker's information based on the position
///     that is given.
///
/// Args:
///     ticker    (&str): String of the ticker
///     position (usize): Position for the header to be printed
///                       this typically corresponds to the index
///                       of the ticker in the Vec of TickerData
/// Returns:
///     None
fn print_header(ticker: &str, position: usize){
    let vertical_offset: u16 = (position * 3) as u16;

    println!("{}{}{ticker}{reset} - ",
        cursor::Goto(0, vertical_offset),
        termion::style::Italic,
        ticker=ticker,
        reset=termion::style::Reset);
    println!("[{}]", " ".repeat(NUM_VALUES));
}

/// Push Data
///     Pushes an appropriately colored square onto the given
///     ticker's VecDeque
///
/// Args:
///     &ticker (TickerData): Struct from where we get the VecDeque.
///     current_num    (f64): Current price for the ticker.
///
/// Returns: 
///      None
///
fn push_data(ticker: &mut TickerData, current_num: f64){
    if current_num < ticker.most_recent_value{
        ticker.values.push_front(format!("{}{}{}", color::Fg(color::Red),
                                                   BLOCK,
                                                   color::Fg(color::Reset)));
    } 
    else if ticker.most_recent_value < current_num{
        ticker.values.push_front(format!("{}{}{}", color::Fg(color::Green),
                                                   BLOCK,
                                                   color::Fg(color::Reset)));
    }
    else{
        ticker.values.push_front(format!("{}", BLOCK));
    }
}

fn _get_most_recent_close(ticker: &str) -> f64{
    let json = network::get_json_from_ticker(ticker, _TIME_INTERVAL).unwrap();
    let json_obj = json_lib::get_json_from_string(json.text().unwrap()).unwrap();
    let most_recent_json = json_lib::_get_most_recent_entry(&json_obj);
    let string_num:&str= most_recent_json["4. close"].as_str().unwrap();
    return string_num.parse::<f64>().unwrap();
}

/// Get Fake Close
///     Purely for testing purposes, please ignore
///     
fn get_fake_close(ticker: &str) -> f64 {
    let mut rng = rand::thread_rng();
    
    let fake_num: f64 = match ticker{
        "GME" => rng.gen_range(130.0..140.0),
        "AMC" => rng.gen_range(33.0..38.0),
        _ => rng.gen_range(100.0..200.0)
    };

    return fake_num;
}

