const val MOD = 1_000_000_007

val MEM_CHOOSE = Array<LongArray>(2001) { LongArray(2000) { -1 } }
val MEM_POW = LongArray(2001) { -1 }

fun choose(n: Int, k: Int): Long {
    if (MEM_CHOOSE[n][k] != -1L) {
        return MEM_CHOOSE[n][k]
    }
    if (n == 1 || n == 0) {
        MEM_CHOOSE[n][k] == 1L
        return 1
    } else if (k == 0) {
        MEM_CHOOSE[n][k] = 1
        return 1
    } else if (k == 1) {
        MEM_CHOOSE[n][k] = n.toLong()
        return n.toLong()
    } else if (k == n) {
        MEM_CHOOSE[n][k] = 1
        return 1
    } else if (k == n - 1) {
        MEM_CHOOSE[n][k] = n.toLong()
        return n.toLong()
    } else {
        val res = (choose(n - 1, k - 1) + choose(n - 1, k)) % MOD
        MEM_CHOOSE[n][k] = res
        return res
    }
}


fun powOf2(n: Int): Long {
    if (MEM_POW[n] != -1L) {
        return MEM_POW[n]
    }
    if (n == 0) {
        MEM_POW[n] = 1L
        return 1
    } else {
        val res = (powOf2(n - 1) * 2) % MOD
        MEM_POW[n] = res
        return res
    }
}

fun modifier(k: Int): Long {
    return powOf2(k - 1)
}


fun main() = with(System.`in`.bufferedReader()) {
    val (n, k) = this.readLine().split(" ").map { it.toInt() }
    // val arr = this.readLine().split(" ").map { it.toInt() }
    println(choose(n, k) * modifier(k) % MOD)
}