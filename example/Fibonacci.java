public class Fibonacci {
    public static void fibonacci(int a, int b) {
        System.out.println(a);
        fibonacci(b, a + b);
    }

    public static void main(String[] args) {
        fibonacci(1, 1);
    }
}
