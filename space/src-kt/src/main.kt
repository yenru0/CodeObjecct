import java.util.Scanner

fun fib(n: Int): Int {
    if (n == 0 || n == 1) {
        return n
    } else return fib(n - 1) + fib(n - 2)
}

fun main() {
    val sc = Scanner(System.`in`)
    val n = sc.nextInt()
    println(fib(n))
}