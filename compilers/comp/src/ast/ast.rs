use super::super::tokenizer::tokenizer as Token;

fn advance(tokens : &Vec<Token::Token>, index : &mut usize) -> Token::Token {
    *index+=1;
    tokens[*index].clone()
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
                        Token::TokenType::ASSIGN => {},
                        Token::TokenType::SEMICOLON => {},
                        _ => {panic!("Error: Invalid token")}
                    }

                },
                Token::TokenType::STR => {},
                Token::TokenType::LIST => {},
                Token::TokenType::WHILE => {},
                Token::TokenType::IF => {},
                Token::TokenType::IDENTIFIER => {},
                Token::TokenType::GOTO => {},
                Token::TokenType::ADDR => {},
                Token::TokenType::CLOSEBLOCK => {}
                _ => {panic!("Error: Invalid token")}

            }
        }
    }
}