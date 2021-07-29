// James Robertson
//

use serde_json::Result;
use std::env::args;

mod network;
mod json_lib;

const C_RED:&str    = "\x1b[91;1m";
const C_YELLOW:&str = "\x1b[93;1m";
const C_WHITE:&str  = "\x1b[31;0m";


fn main() -> Result<()>{
    let args: Vec<String> = args().collect();
    if args.len() == 1 as usize{
        declare_exception("Too few arguments supplied.");
    }

    let json = network::get_json(&args[1]).unwrap();
    let json_string: String = json.text().unwrap();

    println!("Generating report for {}{}{}", C_YELLOW, &args[1], C_WHITE);
    let json_obj = json_lib::get_json_from_string(json_string).unwrap();
    json_lib::generate_graph(&json_obj);

    Ok(())
}

fn declare_exception(err_str: &str) -> !{
    eprintln!("{}{}{}", C_RED, err_str, C_WHITE);
    panic!();
}

