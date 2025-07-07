const val DIVIDER = 1_000_000_007

data class Counts(
    var cntBiggers: Long,
    var cntSmallers: Long
)

fun inversionCount(s: String, n: Long): Int {
    val alphabetCounts = MutableList(26, { 0 });

    val counts = Counts(0, 0)

    s.forEach { ch ->
        val index = ch.code - 'a'.code
        val cntBiggers = alphabetCounts.slice(index + 1 until 26).sum()
        val cntSmallers = alphabetCounts.slice(0 until index).sum()
        alphabetCounts[index] += 1

        counts.cntBiggers += cntBiggers
        counts.cntSmallers += cntSmallers
    }
    val (n_mod: Long, np_mod: Long, nm_mod: Long) = if (n % 2 == 0L) {
        Triple((n / 2) % DIVIDER, (n + 1) % DIVIDER, (n - 1) % DIVIDER)
    } else {
        Triple(n % DIVIDER, ((n + 1) / 2) % DIVIDER, ((n - 1) / 2) % DIVIDER)
    }

    return ((
            (((((counts.cntBiggers % DIVIDER) * np_mod) % DIVIDER) * n_mod) % DIVIDER) + (((((counts.cntSmallers % DIVIDER) * n_mod) % DIVIDER) * nm_mod) % DIVIDER)
            ) % DIVIDER
            ).toInt()
}

fun main() = with(System.`in`.bufferedReader()) {
    val s = readln().trim()
    val n = readln().trim().toLong()

    println(inversionCount(s, n))
}