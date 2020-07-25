use std::fs;
use std::env;

mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    
    println!("Opening {}", filename);

    let contents = fs::read_to_string(filename).expect("Couldn't open the file");
    parser::ags_parser(contents);
}
