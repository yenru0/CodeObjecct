const val MOD = 10_007

val MEMORY = Array<IntArray>(1003) { IntArray(10) { -1 } }

fun countUprisingFrom(from: Int /*[0, 9]*/, length: Int): Int {
    if (MEMORY[length][from] != -1) {
        return MEMORY[length][from]
    }
    return if (length == 1) {
        MEMORY[length][from] = 1
        1
    } else {
        val s = (from..9).fold(0) { total, now ->
            (total + countUprisingFrom(now, length - 1)) % MOD
        }
        MEMORY[length][from] = s
        s
    }
}

fun countTotalUprising(length: Int): Int {
    return ((0..9).sumOf { countUprisingFrom(it, length) } % MOD)
}


fun main() {
    val n = readln().toInt()
    println(countTotalUprising(n))
}