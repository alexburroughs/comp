pub struct Token {
    
}

pub enum TokenType {
    OPERATOR(String),
    IDENTIFIER(String),
    BRACKET(String),
    VALUE(String),
    TYPE(String)
}