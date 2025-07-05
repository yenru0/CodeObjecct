fun getGoogolStringSplitLocation(n: Int): Long {
    return (1L shl n)
}

fun getGoogolStringK(k: Long, n: Int): Boolean {
    if (n == 1) {
        return false
    }
    val split = getGoogolStringSplitLocation(n - 1)
    return if (k > split) {
        !getGoogolStringK(split - (k - split), n - 1)
    } else if (k == split) {
        false
    } else { // k < split
        getGoogolStringK(k, n - 1)
    }
}

fun main() = with(System.`in`.bufferedReader()) {
    val t = readln().toInt()
    (1..t).forEach { i ->
        val k = readln().toLong()
        println("Case #${i}: ${if (getGoogolStringK(k, 62)) 1 else 0}")
    }
}