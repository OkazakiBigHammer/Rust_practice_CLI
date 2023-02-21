use std::env;
use std::process;
//process help us handle withotu panicking

use ::minirep::run;
use ::minirep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    //let config = parse_config(&args);
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments:{}", err);
        process::exit(1);
    });
    //closure!!!
    //anonymous functions that can capture variables from their surrounding environment

    println!("Finding {} in {}", config.query, config.filename);

    //Why use run instead of unwrap_or_else?
    //run does NOT return a value!!!(like config we need a Config)
    if let Err(e) = run(config) {
        eprintln!("Application Error! {}", e);
        process::exit(1);
    };
}

// fn parse_config(args: &[String]) -> (&str, &str){
//     let query = &args[1];
//     let filename = &args[2];

//     Config{query, filename}
// }
