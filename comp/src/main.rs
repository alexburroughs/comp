use std::env;

mod filemanager;

fn main() {
    let args: Vec<String> = env::args().collect();

    // check for at least input file and output file
    if args.len() < 3 {
        panic!("please provide a input filename and output filename as arguments");
    }

    let in_filename = &args[1];
    let out_filename = &args[2];

    // load and unwrap file
    let code_opt = filemanager::load_file(&in_filename);
    let mut code_string : String;

    match code_opt {
        Some(x) => code_string = x,
        None => panic!("error unwrapping file"),
    }
}