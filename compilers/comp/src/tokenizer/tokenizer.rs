
pub struct Token {
    pub t_type : TokenType,
    pub name : Option<String>
}

impl Clone for Token {
    fn clone(&self) -> Token {
        Token {
            t_type : match self.t_type {
                TokenType::IF => {TokenType::IF},
                TokenType::ELSE => {TokenType::ELSE},
                TokenType::WHILE => {TokenType::WHILE},
                TokenType::NUM => {TokenType::NUM},
                TokenType::STR => {TokenType::STR},
                TokenType::LIST => {TokenType::LIST},
                TokenType::OPENBLOCK => {TokenType::OPENBLOCK},
                TokenType::CLOSEBLOCK => {TokenType::CLOSEBLOCK},
                TokenType::OPENBRACKET => {TokenType::OPENBRACKET},
                TokenType::CLOSEBRACKET => {TokenType::CLOSEBRACKET},
                TokenType::SEMICOLON => {TokenType::SEMICOLON},
                TokenType::COMMA => {TokenType::COMMA},
                TokenType::FUNCTION => {TokenType::FUNCTION},
                TokenType::IDENTIFIER => {TokenType::IDENTIFIER},
                TokenType::GOTO => {TokenType::GOTO},
                TokenType::ADDR => {TokenType::ADDR},
                TokenType::ASSIGN => {TokenType::ASSIGN},
                TokenType::EQUAL => {TokenType::EQUAL},
                TokenType::GREATER => {TokenType::GREATER},
                TokenType::LESS => {TokenType::LESS},
                TokenType::GREATEREQ => {TokenType::GREATEREQ},
                TokenType::LESSEQ => {TokenType::LESSEQ},
                TokenType::NOT => {TokenType::NOT},
                TokenType::NOTEQ => {TokenType::NOTEQ},
                TokenType::AND => {TokenType::AND},
                TokenType::OR => {TokenType::OR},
                TokenType::XOR => {TokenType::XOR},
                TokenType::MOD => {TokenType::MOD},
                TokenType::ADD => {TokenType::ADD},
                TokenType::SUB => {TokenType::SUB},
                TokenType::MUL => {TokenType::MUL},
                TokenType::DIV => {TokenType::DIV},
                TokenType::STRING => {TokenType::STRING},
                TokenType::NUMBER => {TokenType::NUMBER}
            },
            name : self.name.clone()
        }
    }
}

impl TokenType {
    pub fn get_precendence(&self) -> i32{
        match self {
            TokenType::OR | TokenType::XOR => {1},
            TokenType::AND => {2},
            TokenType::LESS | TokenType::LESSEQ | TokenType::GREATER | TokenType::GREATEREQ | TokenType::EQUAL => {3}
            TokenType::ADD | TokenType::SUB => {4},
            TokenType::MUL | TokenType::DIV | TokenType::MOD => {5},
            TokenType::NOT => {6},
            _ => {-1}
        }
    }
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

impl TokenType {
    pub fn is_openblock(&self) -> bool {
        match self {
            TokenType::OPENBLOCK => {true},
            _ => {false}
        }
    }

    pub fn is_openbracket(&self) -> bool {
        match self {
            TokenType::OPENBRACKET => {true},
            _ => {false}
        }
    }

    pub fn is_identifier(&self) -> bool {
        match self {
            TokenType::IDENTIFIER => {true},
            _ => {false}
        }
    }

    pub fn is_comma(&self) -> bool {
        match self {
            TokenType::COMMA => {true},
            _ => {false}
        }
    }

    pub fn is_closebracket(&self) -> bool {
        match self {
            TokenType::CLOSEBRACKET => {true},
            _ => {false}
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            TokenType::STRING => {true},
            _ => {false}
        }
    }
}

fn match_next(list : &Vec<char>, index : usize, op : &str) -> bool {
    let mut curr = 0;
    let op_list : Vec<char> = op.chars().collect();
    while curr < op_list.len() {
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
        println!("current: {}", curr);
        if curr.is_whitespace() {
            index += 1;
            continue;
        }
        if curr.is_alphabetic() {
            if match_next(&inp_list, index, "if") {
                tok_list.push(Token {
                    t_type : TokenType::IF,
                    name : Some(String::from("if"))
                });
                
                index+=2;
            }
            else if match_next(&inp_list, index, "else") {
                tok_list.push(Token {
                    t_type : TokenType::ELSE,
                    name : Some(String::from("else"))
                });
                
                index+=4;
            }
            else if match_next(&inp_list, index, "while") {
                tok_list.push(Token {
                    t_type : TokenType::WHILE,
                    name : Some(String::from("while"))
                });
                
                index+=5;
            }
            else if match_next(&inp_list, index, "num") {
                tok_list.push(Token {
                    t_type : TokenType::NUM,
                    name : Some(String::from("num"))
                });
                
                index+=3;
            }
            else if match_next(&inp_list, index, "str") {
                tok_list.push(Token {
                    t_type : TokenType::STR,
                    name : Some(String::from("str"))
                });
                
                index+=3;
            }
            else if match_next(&inp_list, index, "list") {
                tok_list.push(Token {
                    t_type : TokenType::LIST,
                    name : Some(String::from("list"))
                });
                
                index+=4;
            }
            else if match_next(&inp_list, index, "function") {
                tok_list.push(Token {
                    t_type : TokenType::FUNCTION,
                    name : Some(String::from("function"))
                });
                
                index+=8;
            }
            else if match_next(&inp_list, index, "goto") {
                tok_list.push(Token {
                    t_type : TokenType::GOTO,
                    name : Some(String::from("goto"))
                });
                
                index+=4;
            }
            else if match_next(&inp_list, index, "and") {
                tok_list.push(Token {
                    t_type : TokenType::AND,
                    name : Some(String::from("and"))
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

                while inp_list[index].is_alphanumeric() {
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

            while inp_list[index].is_numeric() {
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
                        name : Some(String::from("=="))
                    });
                    index+=2;
                }

                else {
                    tok_list.push(Token {
                        t_type : TokenType::ASSIGN,
                        name : Some(String::from("="))
                    });
                    index+=1;
                }
            }
            else if curr == '<' {
                if inp_list[index+1] == '=' {
                    tok_list.push(Token {
                        t_type : TokenType::LESSEQ,
                        name : Some(String::from("<="))
                    });
                    index+=2;
                }

                else {
                    tok_list.push(Token {
                        t_type : TokenType::LESS,
                        name : Some(String::from("<"))
                    });
                    index+=1;
                }
            }
            else if curr == '>' {
                if inp_list[index+1] == '=' {
                    tok_list.push(Token {
                        t_type : TokenType::GREATEREQ,
                        name : Some(String::from(">="))
                    });
                    index+=2;
                }

                else {
                    tok_list.push(Token {
                        t_type : TokenType::GREATER,
                        name : Some(String::from(">"))
                    });
                    index+=1;
                }
            }
            else if curr == '!' {
                if inp_list[index+1] == '=' {
                    tok_list.push(Token {
                        t_type : TokenType::NOTEQ,
                        name : Some(String::from("!="))
                    });
                    index+=2;
                }

                else {
                    tok_list.push(Token {
                        t_type : TokenType::NOT,
                        name : Some(String::from("!"))
                    });
                    index+=1;
                }
            }
            else if curr == '+' {
                tok_list.push(Token {
                    t_type : TokenType::ADD,
                    name : Some(String::from("+"))
                });
                
                index+=1;
            }
            else if curr == '-' {
                tok_list.push(Token {
                    t_type : TokenType::SUB,
                    name : Some(String::from("-"))
                });
                index+=1;
            }
            else if curr == '/' {
                tok_list.push(Token {
                        t_type : TokenType::DIV,
                        name : Some(String::from("/"))
                    });
                    index+=1;
            }
            else if curr == '*' {
                tok_list.push(Token {
                    t_type : TokenType::MUL,
                    name : Some(String::from("*"))
                });
                    
                index+=1;
            }
            else if curr == '%' {
                tok_list.push(Token {
                    t_type : TokenType::MOD,
                    name : Some(String::from("%"))
                });
                
                index+=1;
            }
            else if curr == '(' {
                tok_list.push(Token {
                        t_type : TokenType::OPENBRACKET,
                        name : Some(String::from("("))
                    });
                    index+=1;
            }
            else if curr == ')' {
                tok_list.push(Token {
                    t_type : TokenType::CLOSEBRACKET,
                    name : Some(String::from(")"))
                });
                    
                index+=1;
            }
            else if curr == '{' {
                tok_list.push(Token {
                    t_type : TokenType::OPENBLOCK,
                    name : Some(String::from("{"))
                });
                
                index+=1;
            }
            else if curr == '}' {
                tok_list.push(Token {
                    t_type : TokenType::CLOSEBLOCK,
                    name : Some(String::from("}"))
                });
                    
                index+=1;
            }
            else if curr == ';' {
                tok_list.push(Token {
                    t_type : TokenType::SEMICOLON,
                    name : Some(String::from(";"))
                });
                
                index+=1;
            }
            else if curr == ',' {
                tok_list.push(Token {
                    t_type : TokenType::COMMA,
                    name : Some(String::from(","))
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