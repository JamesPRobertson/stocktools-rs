// James Robertson
//

use std::io::prelude::*;
use std::fs::File;
use serde_json::{Result, Value};

const _C_RED:&str    = "\x1b[91;1m";
const _C_GREEN:&str  = "\x1b[92;1m";
const _C_YELLOW:&str = "\x1b[93;1m";
const _C_BLUE:&str   = "\x1b[34;1m";
const _C_WHITE:&str  = "\x1b[31;0m";

const DATA_TIME_KEY:  &str = "Time Series (5min)";
const DATA_OPEN_KEY:  &str = "1. open";
const DATA_CLOSE_KEY: &str = "4. close";

const GREEN_BLOCK: &str = "\x1b[92;1m▮\x1b[31;0m";
const RED_BLOCK: &str = "\x1b[91;1m▮\x1b[31;0m";

struct JsonValues{
    highest:     f64,
    lowest:      f64,
    num_elems:   usize,
    price_tiers: [f64; 10],
}

pub fn get_json_from_string(input: String) -> Result<Value>{
    let stonks: Result<Value> = serde_json::from_str(&input);
    return stonks;
}

pub fn _get_json_from_file(file_path: &str) -> Result<Value>{
    let mut file = File::open(file_path).unwrap();
    let mut file_contents: String = String::new();
    file.read_to_string(&mut file_contents).unwrap();
    //println!("{}", file_contents);
    let stonks: Result<Value> = serde_json::from_str(&file_contents);
    return stonks;
}

pub fn _display_json(json_obj: &Value) -> (){
    let display_string: String = serde_json::to_string_pretty(json_obj).unwrap();
    println!("{}", display_string);
}


pub fn generate_graph(json_obj: &Value) -> () {
    let mut counter = 0;
    let mut previous_close: f64 = 0 as f64;

    let block: &str = "▮";
    
    let data_values = get_data(json_obj);

    //This will stay the same, we just have to change length to struct::num_elements
    //let mut str_arr: [[&str; data_values.num_elems]; 9] = [[" "; data_values.num_elems]; 9];
    let mut str_arr = vec![vec![" "; data_values.num_elems]; 10];

    for (_time, dict) in json_obj[DATA_TIME_KEY].as_object().unwrap(){
        let open_str: &str = dict[DATA_OPEN_KEY].as_str().unwrap();
        let close_str: &str= dict[DATA_CLOSE_KEY].as_str().unwrap();

        let _open_num: f64 = open_str.parse().unwrap();
        let close_num: f64 = close_str.parse().unwrap();
        
        let mut _determinant: f64 = data_values.price_tiers[0];

        for i in 0..data_values.price_tiers.len(){
            _determinant = data_values.price_tiers[i];
            if _determinant < close_num{
                if close_num < previous_close{
                    str_arr[i][counter] = RED_BLOCK;
                }
                else if previous_close < close_num{
                    str_arr[i][counter] = GREEN_BLOCK;
                }
                else{
                    str_arr[i][counter] = block;
                }
                break;
            }
        }

        // This is to print out the flat line
        /*
        if _open_num <= close_num{
            print!("{}{}{}", _C_GREEN, _block, _C_WHITE);
        }
        else{
            print!("{}{}{}", _C_RED, _block, _C_WHITE);
        }
        */

        counter = counter + 1;
        previous_close = close_num;
    }

    println!("\n{}", data_values.highest);

    for i in 0..str_arr.len(){
        print!("|");
        for j in 0..str_arr[i].len(){
            print!("{}", str_arr[i][j]);
        }
        println!("");
    }
    println!("{}", "―".repeat(79));
    println!("{}", data_values.lowest);
    println!("\nThere are {} entries", counter);
    println!("Highest: {}, lowest: {}", data_values.highest, data_values.lowest);
}

fn determine_price_tiers(highest: f64, lowest: f64) -> [f64; 10]{
    let mut tiers: [f64; 10] = [0 as f64; 10];
    let difference: f64 = highest - lowest;
    let step: f64 = difference / 10 as f64;

    for i in 0..10{
        tiers[i] = highest - (step * i as f64);
    }

    return tiers;
}

fn get_data(json_obj: &Value) -> JsonValues {
    let mut cur_highest: f64 = 0 as f64;
    let mut cur_lowest: f64 = f64::MAX;
    let mut count: usize = 0 as usize;

    for (_time, dict) in json_obj[DATA_TIME_KEY].as_object().unwrap(){
        let open_str: &str = dict[DATA_OPEN_KEY].as_str().unwrap();
        let close_str: &str = dict[DATA_CLOSE_KEY].as_str().unwrap();

        let _open_num: f64 = open_str.parse().unwrap();
        let close_num: f64 = close_str.parse().unwrap();

        if cur_highest < close_num{
            cur_highest = close_num;
        }
        else if close_num < cur_lowest{
            cur_lowest = close_num;
        }

        count = count + 1;
    }

    let data_values = JsonValues{
        highest: cur_highest,
        lowest: cur_lowest,
        num_elems: count,
        price_tiers: determine_price_tiers(cur_highest, cur_lowest)
    };

    return data_values;
}
