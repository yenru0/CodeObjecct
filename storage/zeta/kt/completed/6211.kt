import kotlin.math.max

fun getTargetCalories(upper: Int, buckets: List<Int>): Int {
    val deq = mutableListOf<Pair<Int, Int>>()

    deq.add(Pair(0, 0))

    var res = -1;

    while (deq.isNotEmpty()) {
        val (cal, index) = deq.removeLast()
        if (index == buckets.size) {
            res = max(res, cal)
            continue
        }

        val new0 = cal
        val new1 = cal + buckets[index]

        deq.add(Pair(new0, index + 1))
        if (new1 <= upper) {
            deq.add(Pair(new1, index + 1))
        }
    }
    return res
}

fun main() {
    val (upperCalories: Int, _) = readln().split(' ').map { i -> i.toInt() }
    val buckets: List<Int> = readln().split(' ').map { i -> i.toInt() }

    println(getTargetCalories(upperCalories, buckets))
}