// James Robertson
//

//use serde_json::Result;

//mod __json_generator;

use std::{thread, time};
use std::io::{self, Write};

use termion::{cursor, color};

const TEST_DATA_ARR: [i32; 10] = [-1, 1, 2, 3, 2, 1, -1, -2, -1, 0];
const DELAY:time::Duration = time::Duration::from_millis(600);

pub fn stockline_main(){
    let args:Vec<String> = std::env::args().collect();
    let title = "S T O C K L I N E";
    // Set everything up for the title
    print!("{}{}{}{title}",
        termion::clear::All, cursor::Goto(2,1),
        cursor::Hide,
        title=title);
    io::stdout().flush().unwrap();
    thread::sleep(DELAY);
    print_stuff(&args[2]);

    // Please be of reenabling the cursors
    println!("{}", cursor::Show);
}

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

