import kotlin.math.roundToInt

fun main() = with(System.`in`.bufferedReader()) {
    val (h, w) = this.readLine().split(" ").map { it.toInt() }
    var s = 0
    var cnt = 0
    var left: Int = 0
    var right: Int = 0
    for (i in h downTo 1) {
        val line = this.readLine()
        for (j in 0 until w) {
            if (line[j] == '.') {
                continue
            } else {
                s += j
                cnt++
            }
        }

        if (i == 1) {
            left = line.withIndex().first { it.value != '.' }.index
            right = line.withIndex().last { it.value != '.' }.index
        }
    }
    if (cnt == 0) {
        println("balanced")
    } else {
        val mid = (s.toDouble() / cnt.toDouble()).roundToInt()

        if (mid < left) {
            println("left")
        } else if (mid > right) {
            println("right")
        } else {
            println("balanced")
        }
    }


}