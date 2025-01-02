use std::env;
use std::process;

use minigrep::Data;
fn main() {
    let args: Vec<String> = env::args().collect();

    let parsed_data = Data::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // println!("Searching for {}", query);
    // println!("In file {}", filename);
    println!("{:?}", args);
    if let Err(e) = minigrep::run(parsed_data) {
        println!("Application error: {}", e);
        process::exit(1);
    }


}
