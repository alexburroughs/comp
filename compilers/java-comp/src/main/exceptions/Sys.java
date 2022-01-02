package app.exceptions;

public class Sys {

    public static void panic() {
        System.err.println("Exiting...");
        System.exit(1);
    }

    public static void panic(String message) {
        System.err.println(message);
        panic();
    }
}