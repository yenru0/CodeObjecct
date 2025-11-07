import kotlin.math.log10
import kotlin.math.pow

fun champernowneCount(n: Long, k: Long): Int {
    // f(1) = 1,
    // f(2) = f(1) * 10 + 2,
    // f(10) = f(9) * 100 + 10,
    // f(100) = f(99) * 1000 + 100,

    var beforeChamps = 0L

    var cnt = 0
    for (i in 1..n) {
        val logged = (log10(i.toDouble()) + 1).toInt()

        val x = (beforeChamps * 10.0.pow(logged).toLong() + i) % k
        beforeChamps = x
        if (x == 0L) cnt++
    }

    return cnt
}

fun main() = with(System.`in`.bufferedReader()) {
    val (n, k) = this.readLine().split(" ").map { it.toLong() }

    println(champernowneCount(n, k))
}