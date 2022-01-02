package app.tokenizer;

import java.util.Arrays;
import app.exceptions.MatcherException;

public class Matcher {

    private char[] arr;
    private int position;

    public Matcher(char[] arr, int position) {
        this.arr = arr;
        this.position = position;
    }

    public Matcher setArr(char[] arr) {
        this.arr = arr;
        return this;
    }

    public Matcher setPosition(int position) {
        this.position = position;
        return this;
    }

    public boolean match(String match) throws MatcherException {

        if (arr == null)
            throw new MatcherException("Array must be defined");
        return (Arrays.copyOfRange(arr, position, match.length() + position).toString() == match);
    }
}