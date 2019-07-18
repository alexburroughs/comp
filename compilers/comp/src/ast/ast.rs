use super::super::tokenizer::tokenizer as Token;

pub enum ValueType {
    STATEMENT,
    EXPRESSION,
    VALUE,
    OPERATOR,
    RETURN,
    FUNCTION,
    ARGUMENT,
    PARAMETER,
    DECALARATION,
    ASSIGNMENT,
    VARIABLE,
    CONDITION,
    IF,
    ELSE,
    WHILE,
    ADDRESS,
    GOTO
}

pub struct Value {
    v_type : ValueType,
    name : Option<String>,
    children : Vec<Value>
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

    }
}