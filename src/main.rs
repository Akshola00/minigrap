use std::{env, fs};
fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let filename = &args[2];
    println!("Searching for {}", query);
    println!("In file {}", filename);
    println!("{:?}", args);

    let contents = fs::read_to_string(filename).expect("error trying to read the file");
    println!("With text:\n{}", contents);
}
