package main.tokenizer;

public class TokenType {
    
    private String name;


    public TokenType(String name) {

    }

    public String getName() {
        return name;
    }

    private enum t_type {
        MATCH("match"),
        FUNCTION("function"),
        OPEN_BRACKET("("),
        CLOSE_BRACKET(")"),
        OPEN_BLOCK("{"),
        CLOSE_BLOCK("}"),
        NUM("num"),
        STR("str"),
        LIST("list"),
        ADD("+"),
        SUB("-"),
        MUL(""),
        DIV("/"),
        MOD("%"),
        CMP("=="),
        CMPG(">"),
        CMPL("<"),
        ARROW("->"),
        RET("ret"),
        EQUAL(":="),
        IDENTIFIER("")
        
        
    }
}