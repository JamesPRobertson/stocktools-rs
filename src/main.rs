// James Robertson
//
//
//

use serde_json::Result;
use std::env::args;

mod network;
mod json_lib;
mod stockline;

const C_RED:&str    = "\x1b[91;1m";
const C_YELLOW:&str = "\x1b[93;1m";
const C_WHITE:&str  = "\x1b[31;0m";


fn main() -> Result<()>{
    let args: Vec<String> = args().collect();
    if args.len() < 2_usize{
        declare_exception("Too few arguments supplied.");
    }

    let function: &str = &args[1].as_str();

    match function {
        "graph" => stockgraph_driver(&args[2]).ok(),
        "line"  => stockline_driver().ok(),
        _ => panic!("AH GOD")
    };

    Ok(())
}

fn stockgraph_driver(ticker: &str) -> Result<()>{
    let json = network::get_json(ticker).unwrap();
    let json_string: String = json.text().unwrap();

    println!("Generating report for {}{}{}", C_YELLOW, ticker, C_WHITE);
    let json_obj = json_lib::get_json_from_string(json_string).unwrap();
    json_lib::generate_graph(&json_obj);

    Ok(())

}

fn stockline_driver() -> Result<()>{
    stockline::stockline_main();
    Ok(())
}

fn declare_exception(err_str: &str) -> !{
    eprintln!("{}{}{}", C_RED, err_str, C_WHITE);
    panic!();
}

