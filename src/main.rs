use std::{{env, fs, process}, error::Error};
fn main() {
    let args: Vec<String> = env::args().collect();

    let parsed_data = Data::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // println!("Searching for {}", query);
    // println!("In file {}", filename);
    println!("{:?}", args);
    if let Err(e) = run(parsed_data) {
        println!("Application error: {}", e);
        process::exit(1);
    }
    run(parsed_data);


}
fn run(config: Data) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    Ok(())
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
