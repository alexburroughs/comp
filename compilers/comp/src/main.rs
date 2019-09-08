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

    let in_filename = String::from("/home/alex/Documents/github/comp/compilers/expr_scripts/test.nni");
    //&args[1];
    let out_filename = &args[2];

    // load and unwrap file
    let code_opt = filemanager::load_file(&in_filename);
    println!("loaded file");
    let mut code_string : String;

    match code_opt {
        Some(x) => code_string = x,
        None => panic!("error unwrapping file: {}", in_filename),
    };
    let tok_list = Token::tokenize(code_string);
    println!("Tokenized");
    let mut tree = SyntaxTree::AST::new();
    tree.generate_tree(tok_list);
    println!("Functions: {}", tree.functions.len());
    println!("Args: {}", tree.functions[0].children[0].children.len());
    println!("Converted to tree");

    println!("Done\n{}", Generator::generate_code(&mut tree));
}   