package app.tokenizer;

import com.google.common.reflect.TypeToken;

public class Token {

    private TypeToken type;
    private int line;
    private int linepos;

    Token(TypeToken type, int line, int linepos) {
        this.type = type;
        this.line = line;
        this.linepos = linepos;       
    }


}