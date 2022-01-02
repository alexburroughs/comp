package main.tokenizer;

import java.util.List;
import java.lang.instrument.UnmodifiableClassException;
import java.util.ArrayList;
import com.google.common.collect.ImmutableMap; 

import main.tokenizer.Token;
import main.tokenizer.TokenType;
import main.tokenizer.Matcher;
import main.exceptions.MatcherException;
import main.exceptions.Sys;


public class Tokenizer {

    private final ImmutableMap<String, TokenType> allTokens;

    Tokenizer (ImmutableMap<String, TokenType> tokens) {
        this.allTokens = ImmutableMap.of(tokens);
    }


    public List<Token> generateTokens(String inp) {

        List<Token> tokens = new ArrayList<Token>();

        int position = 0;
        int line = 0;
        int linepos = 0;

        char[] asChar = inp.toCharArray();
        
        boolean matched;
        Matcher matcher = new Matcher(asChar, position);

        while (position <= asChar.length) {
            
            matcher.setPosition(position);
            matched = false;

            try { 

                if (matcher.match(" ")) {
                    linepos++;
                    position += 1;
                }
                else if (matcher.match("\n")) {
                    line++;
                    linepos = 0;
                    position += 1;
                }

                for (TokenType t : allTokens.keys) {
                    if (matched = matcher.match(t)) {
                        tokens.add(new Token());
                        break;
                    }
                }
            } 
            catch (MatcherException e) {
                Sys.panic("Error parsing tokens");
            }
            
        }               

        return tokens;
    }

}