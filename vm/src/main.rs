use std::env;

mod filemanager;
mod tokenizer;
use tokenizer::command;

fn main() {

    let args: Vec<String> = env::args().collect();

    // check for at least input file and output file
    if args.len() < 3 {
        panic!("please provide a input filename and output filename as arguments");
    }

    let in_filename = &args[1];
    let out_filename = &args[2];

    // load and unwrap file
    let code_string = filemanager::load_file(&in_filename).expect("error unwrapping file");

    // get instructions
    let instructions = tokenizer::tokenize(&code_string);
}
