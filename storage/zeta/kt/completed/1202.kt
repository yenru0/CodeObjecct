import java.util.PriorityQueue

fun main() = with(System.`in`.bufferedReader()) {
    val (n, k) = this.readLine().split(' ').map { it.toInt() }

    val items = (1..n).map {
        this.readLine().split(' ').let {
            it[0].toInt() to it[1].toInt()
        }
    }.sortedBy { it.first /* weight */ }

    val capacities = (1..k).map {
        this.readLine().toInt()
    }.sortedBy { it }

    val heap = PriorityQueue<Int>(Comparator.reverseOrder<Int>())

    var curr = 0
    var total_value = 0L
    for (cap in capacities) {
        while (curr < n && cap >= items[curr].first) {
            heap.add(items[curr].second)
            curr++;
        }
        heap.poll()?.let {
            total_value += it
        }
    }

    println(total_value)
}