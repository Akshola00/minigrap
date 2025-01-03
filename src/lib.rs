use std::{fs, error::Error};

pub fn run(config: Data) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let result = search(&config.query, &contents);
    println!("With text:\n{}\nFound matches:\n{:?}", contents, result);
    Ok(())
}

pub fn search<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut all_matches:Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            all_matches.push(line);
        }
    }
    all_matches
}
pub struct Data {
    pub query: String,
    pub filename: String,
}

impl Data { 
    pub fn new(args: &[String]) -> Result<Data, &str> {
        if args.len() != 3 {
            return Err("expected 3 arguments")
        }
        let query = args[1].clone();
        let filename = args[2].clone();
       Ok(Data {query, filename})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_data() {
        let args = vec![String::from("minigrep"), String::from("query"), String::from("filename")];
        let data = Data::new(&args).unwrap();
        assert_eq!(data.query, "query");
        assert_eq!(data.filename, "filename");
    }
}