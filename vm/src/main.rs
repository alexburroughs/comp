use std::env;

mod filemanager;
mod instructionmanager {
    pub mod tokenizer;
    pub mod vm;
    pub mod command;
}

use instructionmanager::tokenizer;
use instructionmanager::vm as VirtualMachine;

fn main() {

    let args: Vec<String> = env::args().collect();

    // check for at least input file and output file
    if args.len() < 2 {
        panic!("please provide a input filename and output filename as arguments");
    }

    let in_filename = &args[1];

    // load and unwrap file
    let code_string = filemanager::load_file(&in_filename).expect("error unwrapping file");

    // get instructions and a map of functions
    let (instructions, f_map) = tokenizer::tokenize(&code_string);

    // run all commands
    let mut vm = VirtualMachine::Vm::new();
    vm.run(instructions, f_map);
}
