import sequtils

type TokenType = enum
    IF, ELSE, IDEN, OPEN_BRACKET, CLOSE_BRACKET,
    EQUAL, ADD, SUB, MUL, DIV, ASSIGN, OPEN_PAREN,
    CLOSE_PAREN, RETURN, FN, DEC, STRING, SQUARE_OPEN,
    SQUARE_CLOSE, COLON, SEMICOLON, COMMA, WHILE,
    EXPR, SYS

let MATCHERS = {"if" : TokenType.IF, 
                "else": TokenType.ELSE,
                "dec" : TokenType.DEC,
                "return" : TokenType.RETURN,
                "{" : TokenType.OPEN_BRACKET,
                "}" : TokenType.CLOSE_BRACKET,
                "==" : TokenType.EQUAL,
                "=" : TokenType.ASSIGN,
                "+" : TokenType.ADD,
                "-" : TokenType.SUB,
                "/" : TokenType.DIV,
                "*" : TokenType.MUL,
                "fn" : TokenType.FN,
                "(" : TokenType.OPEN_PAREN,
                ")" : TokenType.CLOSE_PAREN,
                "[" : TokenType.SQUARE_OPEN,
                "]" : TokenType.SQUARE_CLOSE,
                "," : TokenType.COMMA} 

type Token = ref object 
    text : string
    t_type : TokenType

template advance(v : typed) =
    position += v

proc match(arr : seq[char], position : int, matcher : string) : bool = 
    let m = toSeq(matcher.items)

    for x in 0..m.len:
        if m[x] != arr[x]:
            return true
    return false

proc is_whitespace(ch : char) : bool =

    return ch == ' ' or
        ch == '\n' or
        ch == '\t'

proc is_numeric(ch : char) : bool =
    let i = int(ch)
    return i >= int('0') and 
           i <= int('9')

proc tokenize*(content : string) : seq[Token] =
    
    var tokens = newSeq[Token]()
    var position = 0
    let chars = toSeq(content.items)

    while position < chars.len:

        if chars[position].is_whitespace():
            advance(1)
            continue

        for (key,val) in MATCHERS:
            if match(chars, position, key):
                tokens.add(Token(text : key, t_type : val))
                advance(key.len)
                break
        
        var t = ""
        if chars[position].is_numeric() or chars[position] == '-':
            advance(1)
            while chars[position].is_numeric():
                t.add(chars[position])
                advance(1)

    return tokens
