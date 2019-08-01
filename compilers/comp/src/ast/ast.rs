use super::super::tokenizer::tokenizer as Token;

fn advance(tokens : &Vec<Token::Token>, index : &mut usize) -> Token::Token {
    *index+=1;
    tokens[*index].clone()
}

fn match_expr(tokens : &Vec<Token::Token>, index : usize) -> Vec<Token::Token> {

    let mut i = index;
    let mut new_vec : Vec<Token::Token> = Vec::new();
    loop {
        new_vec.push(tokens[i].clone());
        i+=1;
        
        match tokens[i].t_type { Token::TokenType::SEMICOLON => {break}, _ => {}}
    }

    new_vec
}

enum ParseState {
    Start,
    Open,
    Close,
    Operand,
    Operator
}

fn parse_expr(tokens : Vec<Token::Token>) -> Value {

    let base = Value {
        v_type : ValueType::EXPRESSION,
        name : None,
        children : Vec::new()
    };

    let mut state = ParseState::Start;

    let mut x = 0;

    while x < tokens.len() {
        
        match state {
            ParseState::Start => {
                match tokens[x].t_type {
                    Token::TokenType::OPENBRACKET => {state = ParseState::Open},
                    Token::TokenType::IDENTIFIER => {state = ParseState::Operand},
                    _ => {panic!("Error: Invalid expression")}
                }
            },
            ParseState::Open => {},
            ParseState::Close => {},
            ParseState::Operand => {},
            ParseState::Operator => {}
        }
    }

    base
}

pub enum ValueType {
    STATEMENT,
    EXPRESSION,
    VALUE,
    OPERATOR,
    RETURN,
    FUNCTION,
    ARGUMENT,
    PARAMETER,
    DECLARATION,
    ASSIGNMENT,
    VARIABLE,
    CONDITION,
    IF,
    ELSE,
    WHILE,
    ADDRESS,
    GOTO,
    NUM,
    STR,
}

impl Clone for ValueType {
    fn clone(&self) -> ValueType {

        match self {
            ValueType::IF => {ValueType::IF},
            ValueType::ELSE => {ValueType::ELSE},
            ValueType::WHILE => {ValueType::WHILE},
            ValueType::NUM => {ValueType::NUM},
            ValueType::STR => {ValueType::STR},
            ValueType::STATEMENT => {ValueType::STATEMENT},
            ValueType::EXPRESSION => {ValueType::EXPRESSION},
            ValueType::OPERATOR => {ValueType::OPERATOR},
            ValueType::ADDRESS => {ValueType::ADDRESS},
            ValueType::GOTO => {ValueType::GOTO},
            ValueType::CONDITION => {ValueType::CONDITION},
            ValueType::VARIABLE => {ValueType::VARIABLE},
            ValueType::ASSIGNMENT => {ValueType::ASSIGNMENT},
            ValueType::ARGUMENT => {ValueType::ARGUMENT},
            ValueType::FUNCTION => {ValueType::FUNCTION},
            ValueType::VALUE => {ValueType::VALUE},
            ValueType::RETURN => {ValueType::RETURN},
            ValueType::PARAMETER => {ValueType::PARAMETER},
            ValueType::DECLARATION => {ValueType::DECLARATION},
        }
    }
}

pub struct Value {
    v_type : ValueType,
    name : Option<String>,
    children : Vec<Value>
}

impl Value {
    pub fn new_function() -> Value {
        Value {
            v_type : ValueType::FUNCTION,
            name : None,
            children : Vec::new()
        }
    }

    pub fn get_left(&self) -> Value {
        (*self.children.get(0).expect("Error: No left node")).clone()
    }
}

impl Clone for Value {
    fn clone(&self) -> Value {

        Value {
            name : self.name.clone(),
            v_type : self.v_type.clone(),
            children : self.children.clone()
        }
    }
}

enum Statement {
    FUNCTION(String),
    IF(u64),
    WHILE(u64)
}

pub struct AST {
    functions : Vec<Value>
}

impl AST {
    pub fn new() -> AST {
        AST {
            functions : Vec::new()
        }
    }

    pub fn generate_tree(&mut self, tokens: Vec<Token::Token>) {

        let mut index = 0;
        let mut ast = AST::new();
        let mut curr_func : Value;
        let mut block_stack : Vec<Statement> = Vec::new();
        let mut conditional_num : u64 = 0;

        while index < tokens.len() {
            match tokens[index].t_type {
                Token::TokenType::FUNCTION => {
                    curr_func = Value::new_function();
                    let next = advance(&tokens, &mut index);
                    match next.t_type {Token::TokenType::IDENTIFIER => {}, _ => {panic!("Error: Invalid token")}}
                    curr_func.name = next.name;
                    if !advance(&tokens, &mut index).t_type.is_openblock() {panic!("Error: Invalid token")} 
                    block_stack.push(Statement::FUNCTION(curr_func.name.expect("Error: Can't parse function name")));
                },
                Token::TokenType::NUM => {
                    let declaration = Value {
                        v_type : ValueType::NUM,
                        name : advance(&tokens, &mut index).name,
                        children : Vec::new()
                    };

                    match advance(&tokens, &mut index).t_type {
                        Token::TokenType::ASSIGN => {
                            let expr = match_expr(&tokens, index);

                        },
                        Token::TokenType::SEMICOLON => {},
                        _ => {panic!("Error: Invalid token")}
                    }

                },
                Token::TokenType::STR => {
                    let declaration = Value {
                        v_type : ValueType::NUM,
                        name : advance(&tokens, &mut index).name,
                        children : Vec::new()
                    };

                    match advance(&tokens, &mut index).t_type {
                        Token::TokenType::ASSIGN => {
                            let expr = match_expr(&tokens, index);

                        },
                        Token::TokenType::SEMICOLON => {},
                        _ => {panic!("Error: Invalid token")}
                    }
                },
                Token::TokenType::LIST => {},
                Token::TokenType::WHILE => {

                    conditional_num+=1;
                },
                Token::TokenType::IF => {

                    conditional_num+=1;
                },
                Token::TokenType::IDENTIFIER => {},
                Token::TokenType::GOTO => {},
                Token::TokenType::ADDR => {},
                Token::TokenType::CLOSEBLOCK => {}
                _ => {panic!("Error: Invalid token")}
            }
        }
    }
}