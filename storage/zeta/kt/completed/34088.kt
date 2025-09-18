import kotlin.math.absoluteValue
import kotlin.math.max
import kotlin.math.sqrt

fun getAllDivisor(x: Int): List<Int> {
    val divs = mutableListOf<Int>()

    for (i in 2..sqrt(x.toDouble()).toInt()) {
        if (x % i == 0) {
            val oppo = x / i
            if (oppo != i) {
                divs.add(oppo)
            }
            divs.add(i)
        }

    }

    divs.add(x)
    return divs
}


fun modmod(arr: List<Int>): Int {
    if (arr.size == 1) {
        return 1
    }
    var maxGroupSize = -1

    maxGroupSize = max(arr.count { it % 2 == 0 }, arr.count { it % 2 == 1 })

    for (i in 0 until 24) {
        val a = arr.random()
        val b = arr.random()

        if ((a - b).absoluteValue <= 1) {
            continue
        }

        val divs = getAllDivisor((a - b).absoluteValue)

        for (div in divs) {
            val alpha = a % div

            maxGroupSize = max(arr.count { it % div == alpha }, maxGroupSize)
        }
    }

    return maxGroupSize
}


fun main() = with(System.`in`.bufferedReader()) {
    this.readLine().toInt()

    val arr = this.readLine().split(' ').map { it.toInt() }

    println(modmod(arr))

}