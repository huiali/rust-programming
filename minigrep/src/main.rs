use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (query, filename) = parse_config(&args);
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    print!("With text :\n{}", content);

    // println!("Searching for {} and in file {}", query, filename);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];
    (query, filename)
}
