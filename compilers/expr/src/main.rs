use std::env;

mod filemanager;

fn main() {
    
    let args: Vec<String> = env::args().collect();

    // check for at least input file and output file
    if args.len() < 3 {
        panic!("please provide a input filename and output filename as arguments");
    }

    let in_filename = &args[1];

    // load and unwrap file
    let code_string = filemanager::load_file(&in_filename).expect("error unwrapping file");

    // remove whitespace
    let code_string = code_string.replace(" ", "");

    let mut res = String::from("");
    let mut stack : Vec<char> = Vec::new();

    let mut first = true;
    let mut index = 0;

    for x in code_string.chars() {
        let c = x;

        if c == ' ' || c == '\n' {
            continue;
        }

        if c.is_alphanumeric() {
            if !first {
                res.push('\n');
            }
            res.push_str("NEW NUM");
            res.push_str(&format!("\nSET {} NUM {}", index, c));
            res.push_str(&format!("\nPUSH {}", index));
            index += 1;
            first = false;
        }

        else if c == '(' {
            stack.push(c);
        }

        else if c == ')' {
            while stack.len() > 0 && *stack.last().unwrap() != '(' {
                res.push_str(match stack.pop().expect("error popping stack"){
                    '+' => {"\nADD"},
                    '-' => {"\nSUB"},
                    '*' => {"\nMUL"},
                    '/' => {"\nDIV"},
                    '%' => {"\nMOD"},
                    _ => {panic!("Error")}
                });
            }

            if stack.len() > 0 && *stack.last().unwrap() != '(' {
                panic!("Error: Invalid expression");
            }

            else {
                stack.pop();
            }
        }

        else {
            while stack.len() > 0 && get_precedence(&c) <= get_precedence(stack.last().unwrap()) {
                res.push_str(match stack.pop().expect("error popping stack"){
                    '+' => {"\nADD"},
                    '-' => {"\nSUB"},
                    '*' => {"\nMUL"},
                    '/' => {"\nDIV"},
                    '%' => {"\nMOD"},
                    _ => {panic!("error here")}
                });
            }
            stack.push(c);
        }
    }

    while stack.len() > 0 {
        let mut tmp = String::from("");
        let val = stack.pop().expect("error popping stack");
        res.push_str(
            match val {
                '+' => {"\nADD"},
                '-' => {"\nSUB"},
                '*' => {"\nMUL"},
                '/' => {"\nDIV"},
                '%' => {"\nMOD"},
                _ => {
                        if val.is_alphanumeric() {
                            tmp.push_str("\nNEW NUM");
                            tmp.push_str(&format!("\nSET {} NUM {}", index, val));
                            tmp.push_str(&format!("\nPUSH {}", index));
                            index += 1;
                            tmp.as_str()
                        }
                        else {
                            ""
                        }
                    }
        });
    }

    res.push_str("\nNEW NUM");
    res.push_str(&format!("\nPOP {}", index));
    res.push_str(&format!("\nSYS PRINT {}", index));

    filemanager::out_file(&args[2], &res);
}

fn get_precedence(opt : &char) -> i32 {
    match opt {
        '+' | '-' => {1},
        '*' | '/' | '%' => {2},
        _ => {-1}
    }
}