use std::{env, fs, process};
fn main() {
    let args: Vec<String> = env::args().collect();

    let parsed_data = Data::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // println!("Searching for {}", query);
    // println!("In file {}", filename);
    println!("{:?}", args);
    run(parsed_data);


}
fn run(config: Data) {
    let contents = fs::read_to_string(config.filename).expect("error trying to read the file");
    println!("With text:\n{}", contents);
}
struct Data {
    query: String,
    filename: String,
}

impl Data { 
    fn new(args: &[String]) -> Result<Data, &str> {
        if args.len() != 3 {
            return Err("expected 3 arguments")
        }
        let query = args[1].clone();
        let filename = args[2].clone();
       Ok(Data {query, filename})
    }
}
