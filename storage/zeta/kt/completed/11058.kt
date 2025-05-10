import kotlin.math.max

val Mem = Array<Long>(101) { 0 }

fun solve(N: Int): Long {
    for (i in 1..N) {
        Mem[i] = Mem[i - 1] + 1
        for (j in 3 until i) {
            Mem[i] = max(Mem[i], Mem[i - j] * (j - 1))
        }
    }
    return Mem[N]
}

fun main() {
    val N = readln().toInt()
    println(solve(N))
}