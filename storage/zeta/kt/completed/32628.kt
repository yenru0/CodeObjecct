import kotlin.math.max
import kotlin.math.min

class Bag(items: List<Long>) {
    val cumulativeWeight: List<Long>

    init {
        var x = 0L

        cumulativeWeight = listOf(0L) + items.map {
            x += it
            x
        }
    }

    fun weight(to: Int): Long {
        return cumulativeWeight[to]
    }
}

class BagManager(val n: Int, val bag1: Bag, val bag2: Bag) {
    fun smallestBig(k: Int): Long {
        return (max(0, k - n)..min(n, k)).map { i -> i to (k - i) }.minOf { (i, j) ->
            val w1 = bag1.weight(n - i)
            val w2 = bag2.weight(n - j)
            max(w1, w2)
        }
    }

}

fun main() = with(System.`in`.bufferedReader()) {
    val (n, k) = this.readLine()!!.split(' ').map { it.toInt() }

    val bag1 = Bag(ArrayDeque<Long>(this.readLine()!!.split(' ').map { it.toLong() }))
    val bag2 = Bag(ArrayDeque<Long>(this.readLine()!!.split(' ').map { it.toLong() }))

    val manager = BagManager(n, bag1, bag2)
    println(manager.smallestBig(k))
}