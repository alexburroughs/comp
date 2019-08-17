use std::env;


pub mod tokenizer {
    pub mod tokenizer;
}

pub mod ast {
    pub mod ast;
}

pub mod generator {
    pub mod generator;
}

use ast::ast as SyntaxTree;
use generator::generator as Generator;

use tokenizer::tokenizer as Token;

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
    };

    let tok_list = Token::tokenize(code_string);

    let tree = SyntaxTree::AST::new();
}   