
pub struct Token {
    t_type : TokenType,
    name : Option<String>
}

pub enum TokenType {
    IF,
    ELSE,
    WHILE,
    NUM,
    STR,
    LIST,
    OPENBLOCK,
    CLOSEBLOCK,
    OPENBRACKET,
    CLOSEBRACKET,
    SEMICOLON,
    COMMA,
    FUNCTION,
    IDENTIFIER,
    GOTO,
    ADDR,
    ASSIGN,
    EQUAL,
    GREATER,
    LESS,
    GREATEREQ,
    LESSEQ,
    NOT,
    NOTEQ,
    AND,
    OR,
    XOR,
    MOD,
    ADD,
    SUB,
    MUL,
    DIV,
    STRING,
    NUMBER
}

fn match_next(list : &Vec<char>, index : usize, op : &str) -> bool {
    let mut curr = 0;
    let op_list : Vec<char> = op.chars().collect();

    while curr < list.len() {
            if list[index + curr] != op_list[curr] {
                return false;
            }
        curr+=1;
    }

    return true;
}

pub fn tokenize(inp : String) -> Vec<Token> {

    let mut tok_list : Vec<Token> = Vec::new();
    let inp_list : Vec<char> = inp.chars().collect();
    let mut index : usize = 0;

    while index < inp_list.len() {

        let curr = inp_list[index];

        if curr.is_alphabetic() {
            if match_next(&inp_list, index, "if") {
                tok_list.push(Token {
                    t_type : TokenType::IF,
                    name : None
                });
                
                index+=2;
            }
            else if match_next(&inp_list, index, "else") {
                tok_list.push(Token {
                    t_type : TokenType::ELSE,
                    name : None
                });
                
                index+=4;
            }
            else if match_next(&inp_list, index, "while") {
                tok_list.push(Token {
                    t_type : TokenType::WHILE,
                    name : None
                });
                
                index+=5;
            }
            else if match_next(&inp_list, index, "num") {
                tok_list.push(Token {
                    t_type : TokenType::NUM,
                    name : None
                });
                
                index+=3;
            }
            else if match_next(&inp_list, index, "str") {
                tok_list.push(Token {
                    t_type : TokenType::STR,
                    name : None
                });
                
                index+=3;
            }
            else if match_next(&inp_list, index, "list") {
                tok_list.push(Token {
                    t_type : TokenType::LIST,
                    name : None
                });
                
                index+=4;
            }
            else if match_next(&inp_list, index, "function") {
                tok_list.push(Token {
                    t_type : TokenType::FUNCTION,
                    name : None
                });
                
                index+=8;
            }
            else if match_next(&inp_list, index, "goto") {
                tok_list.push(Token {
                    t_type : TokenType::GOTO,
                    name : None
                });
                
                index+=4;
            }
            else if match_next(&inp_list, index, "and") {
                tok_list.push(Token {
                    t_type : TokenType::AND,
                    name : None
                });
                
                index+=3;
            }
            else if match_next(&inp_list, index, "or") {
                tok_list.push(Token {
                    t_type : TokenType::OR,
                    name : None
                });
                
                index+=2;
            }
            else if match_next(&inp_list, index, "xor") {
                tok_list.push(Token {
                    t_type : TokenType::GOTO,
                    name : None
                });
                
                index+=3;
            }
            else {

                let mut tmp = String::from("");

                while !inp_list[index].is_alphanumeric() {
                    tmp.push(inp_list[index]);
                    index+=1;
                }

                tok_list.push(Token {
                    t_type : TokenType::IDENTIFIER,
                    name : Some(tmp)
                });
            }
        }
        else if curr.is_numeric() {

            let mut tmp = String::from("");

            while !inp_list[index].is_numeric() {
                tmp.push(inp_list[index]);
                index+=1
            }

            tok_list.push(Token {
                t_type : TokenType::NUMBER,
                name : Some(tmp)
            });   
        }
        else {
            if curr == '=' {
                if inp_list[index+1] == '=' {
                    tok_list.push(Token {
                        t_type : TokenType::EQUAL,
                        name : None
                    });
                    index+=2;
                }

                else {
                    tok_list.push(Token {
                        t_type : TokenType::ASSIGN,
                        name : None
                    });
                    index+=1;
                }
            }
            else if curr == '<' {
                if inp_list[index+1] == '=' {
                    tok_list.push(Token {
                        t_type : TokenType::LESSEQ,
                        name : None
                    });
                    index+=2;
                }

                else {
                    tok_list.push(Token {
                        t_type : TokenType::LESS,
                        name : None
                    });
                    index+=1;
                }
            }
            else if curr == '>' {
                if inp_list[index+1] == '=' {
                    tok_list.push(Token {
                        t_type : TokenType::GREATEREQ,
                        name : None
                    });
                    index+=2;
                }

                else {
                    tok_list.push(Token {
                        t_type : TokenType::GREATER,
                        name : None
                    });
                    index+=1;
                }
            }
            else if curr == '!' {
                if inp_list[index+1] == '=' {
                    tok_list.push(Token {
                        t_type : TokenType::NOTEQ,
                        name : None
                    });
                    index+=2;
                }

                else {
                    tok_list.push(Token {
                        t_type : TokenType::NOT,
                        name : None
                    });
                    index+=1;
                }
            }
            else if curr == '+' {
                tok_list.push(Token {
                    t_type : TokenType::ADD,
                    name : None
                });
                
                index+=1;
            }
            else if curr == '-' {
                tok_list.push(Token {
                    t_type : TokenType::SUB,
                    name : None
                });
                index+=1;
            }
            else if curr == '/' {
                tok_list.push(Token {
                        t_type : TokenType::DIV,
                        name : None
                    });
                    index+=1;
            }
            else if curr == '*' {
                tok_list.push(Token {
                    t_type : TokenType::MUL,
                    name : None
                });
                    
                index+=1;
            }
            else if curr == '%' {
                tok_list.push(Token {
                    t_type : TokenType::MOD,
                    name : None
                });
                
                index+=1;
            }
            else if curr == '(' {
                tok_list.push(Token {
                        t_type : TokenType::OPENBRACKET,
                        name : None
                    });
                    index+=1;
            }
            else if curr == ')' {
                tok_list.push(Token {
                    t_type : TokenType::CLOSEBRACKET,
                    name : None
                });
                    
                index+=1;
            }
            else if curr == '{' {
                tok_list.push(Token {
                    t_type : TokenType::OPENBLOCK,
                    name : None
                });
                
                index+=1;
            }
            else if curr == '}' {
                tok_list.push(Token {
                    t_type : TokenType::CLOSEBLOCK,
                    name : None
                });
                    
                index+=1;
            }
            else if curr == ';' {
                tok_list.push(Token {
                    t_type : TokenType::SEMICOLON,
                    name : None
                });
                
                index+=1;
            }
            else if curr == ',' {
                tok_list.push(Token {
                    t_type : TokenType::COMMA,
                    name : None
                });
                
                index+=1;
            }
            else if curr == '"' {
                index+=1;
                let mut tmp = String::from("");
                while inp_list[index] != '"' {
                    tmp.push(inp_list[index]);
                    index+=1
                }
                tok_list.push(Token {
                        t_type : TokenType::MOD,
                        name : Some(tmp)
                });

                index+=1;
            }
            else if curr == ':' {
                index+=1;
                let mut tmp = String::from("");
                while inp_list[index] != ':' {
                    tmp.push(inp_list[index]);
                    index+=1
                }
                tok_list.push(Token {
                        t_type : TokenType::ADDR,
                        name : Some(tmp)
                });

                index+=1;
            }
        }
    }

    tok_list
}