val mem = IntArray(1001) {
    if (it == 1) {
        1
    } else if (it == 2) {
        2
    } else {
        0
    }
}

const val MOD = 10_007

fun binaryTiling(n: Int): Int {
    if (mem[n] != 0) {
        return mem[n]
    } else {
        mem[n] = (binaryTiling(n - 1) + binaryTiling(n - 2)) % MOD
        return mem[n]
    }
}

fun main() = with(System.`in`.bufferedReader()) {
    val n = readLine().toInt()
    println(binaryTiling(n))
}