import kotlin.math.max
import kotlin.math.min
import kotlin.random.Random

data class Element(
    var consMin: Long, var consMax: Long, var onlyOne: Long, var noChoose: Long
) {
    fun max(): Long = max(max(consMax, consMin), max(onlyOne, noChoose))
}

fun maximaAlternatingSum(arr: List<Long>): Long {
    if (arr.size == 1) {
        return arr.first()
    }
    val dp = mutableListOf<Element>()
    dp.add(Element(arr.last(), arr.last(), arr.last(), Long.MIN_VALUE))

    for (x in arr.slice(0 until arr.size - 1).reversed()) {
        val last = dp.last()

        val e: Element = Element(0, 0, 0, 0)

        e.consMax = max(x - last.consMin, x - last.onlyOne)
        e.consMin = min(x - last.consMax, x - last.onlyOne)
        e.onlyOne = x
        e.noChoose = last.max()

        //println(e)
        dp.add(e)
    }

    return dp.last().max()
}

fun testAltSum(arr: List<Long>): Long {
    var m = Long.MIN_VALUE
    val cum1 = mutableListOf<Long>()
    val cum2 = mutableListOf<Long>()
    cum1.add(0)
    cum2.add(0)

    var sign = -1

    for (item in arr) {
        sign *= -1
        cum1.add(cum1.last() + sign * item)
        cum2.add(cum2.last() - sign * item)
    }

    for (l in 0 until arr.size) {
        for (r in l + 1..arr.size) {
            val s = if (l % 2 == 0) {
                cum1[r] - cum1[l]
            } else {
                cum2[r] - cum2[l]
            }
            m = max(s, m)
        }
    }
    return m
}

fun main() = with(System.`in`.bufferedReader()) {
    this.readLine()
    val arr = this.readLine().split(" ").map { it.toLong() }
    println(maximaAlternatingSum(arr))
}