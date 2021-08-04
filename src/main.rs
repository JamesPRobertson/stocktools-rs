// James Robertson
//

use serde_json::Result;
use std::env::args;
use termion::color;

mod json_lib;
mod stockwatch;
mod network;

fn main() -> Result<()>{
    let args: Vec<String> = args().collect();
    if args.len() < 2_usize{
        declare_exception("Too few arguments supplied.");
    }
    let function: &str = &args[1].as_str();

    match function {
        "graph" => stockgraph_driver(&args[2]).ok(),
        "watch" => stockwatch_driver().ok(),
        _ => panic!("Incorrect argument supplied.")
    };

    Ok(())
}

// TODO: Remove stockgraph from json_lib
fn stockgraph_driver(ticker: &str) -> Result<()>{
    let json = network::get_json_from_ticker(ticker, 5).unwrap();
    let json_string: String = json.text().unwrap();

    println!("Generating report for {}{}{}", color::Fg(color::Yellow), ticker, color::Fg(color::Reset));
    let json_obj = json_lib::get_json_from_string(json_string).unwrap();
    json_lib::generate_graph(&json_obj);

    Ok(())

}

fn stockwatch_driver() -> Result<()>{
    stockwatch::stockwatch_main();
    Ok(())
}

fn declare_exception(err_str: &str) -> !{
    eprintln!("{}{}{}", color::Fg(color::Red), err_str, color::Fg(color::Reset));
    panic!();
}

