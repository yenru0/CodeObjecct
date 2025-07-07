data class Record(
    var counts: Int,
    var lastModified: Int,
)

fun main() = with(System.`in`.bufferedReader()) {
    val (n, k) = readln().split(' ').map { i -> i.toInt() }
    val history = mutableMapOf<String, Record>()

    (1..(3 * n)).forEach { i ->
        val key = readln()

        if (key in history) {
            history[key]?.let {
                it.counts++
                it.lastModified = i
            }
        } else {
            history[key] = Record(1, i)
        }
    }

    history.toList().sortedWith(compareBy({ it.second.counts }, { it.second.lastModified })).takeLast(k).reversed()
        .forEach { item ->
            println(item.first)
        }
}