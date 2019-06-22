pub 
struct Token {
    type : TokenType
}

pub
enum TokenType {
    IF,
    ELSE,
    WHILE,
    NUM,
    STR,
    LIST,
    OPEN_BLOCK,
    CLOSE_BLOCK,
    OPEN_BRACKET
}