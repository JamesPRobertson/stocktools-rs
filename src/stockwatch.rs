// James Robertson
//

use std::{thread, time};
use std::io::{self, Write};
use std::collections::VecDeque;

use termion::{cursor, color};

use crate::network;
use crate::json_lib;

const DELAY:time::Duration = time::Duration::from_millis(600);
const NUM_VALUES: usize = 17;
const TITLE: &str = "S T O C K W A T C H";

struct TickerData{
    most_recent_value: f64,
    values: VecDeque<f64>
}


// Hi, let's think about this whole rendering thing.
// I am going to be using absolute placing here for ease
//S T O C K W A T C H
//
//GME  - [Price Goes here] <- (3,7), or (i + 1) * 3, 7
//[x                ] <- (4, 1) or ((i + 1) * 3) + 1, 1
//
//-------------------------------------------------------
// If we just think of i as the position in the args that
// the ticker occurs, we can use that to format stuff
// easily.
//
// So move cursor to ((i+1), 7) to draw price
// Then move DOWN one, draw \r, then overwrite the
// entire price movement line, using the VecDeque
// when you get new price, push it front into the
// VecDeque and pop the back.
// 
// OOo OOo! Consider displaying the price temporarily
// in green or red, then overwriting it in White after
// a small delay, would be cool! WILL have to make
// it async!
//

pub fn stockwatch_main(){
    let args:Vec<String> = std::env::args().collect();

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
    }

    let json = network::get_json_from_ticker(&args[2], 1).unwrap();
    let json_obj = json_lib::get_json_from_string(json.text().unwrap()).unwrap();
    let most_recent_json = json_lib::get_most_recent_entry(&json_obj);

    // Please be of reenabling the cursors
    println!("{}", cursor::Show);
} 

// TODO: consider formatting the '-' correctly for 3 letter ticks
//       to fit with 4 letter tickers nicely
fn print_header(ticker: &str, position: usize){
    let vertical_offset: u16 = (position * 3) as u16;

    println!("{}{}{ticker}{reset} - ",
        cursor::Goto(0, vertical_offset),
        termion::style::Italic,
        ticker=ticker,
        reset=termion::style::Reset);
    println!("[{}]", " ".repeat(NUM_VALUES));
}

/*
fn print_stuff(ticker: &str){
    // position the cursor where we want it..........
    println!("{}{}{ticker}{reset} - ",
        cursor::Goto(0, 3),
        termion::style::Italic,
        ticker=ticker,
        reset=termion::style::Reset);

    let mut starting_value: i32 = 0;
    let short_delay: time::Duration = time::Duration::from_millis(300);

    print!("[");
    for value in &TEST_DATA_ARR{
        if starting_value < *value{
            print!("{}■{}", color::Fg(color::Red), color::Fg(color::Reset));
        }
        else{
            print!("{}■{}", color::Fg(color::Green), color::Fg(color::Reset));
        }
        starting_value = *value;
        io::stdout().flush().unwrap();
        thread::sleep(short_delay);
    }
    io::stdout().flush().unwrap();
    print!("]");
}
*/
