import kotlin.math.min

fun chocolatePyramidSum(r: Int, c: Int): Long {
    val m = min(r, c).toLong()
    return m * (4 * m * m - 6 * m * (r.toLong() + c.toLong()) + 12 * r.toLong() * c.toLong() - 1) / 3
}

fun chocolatePyramidDark(r: Int, c: Int): Long {
    val m = min(r, c).toLong()
    return m * (2 * m * m - 3 * m * (r.toLong() + c.toLong()) + 6 * r.toLong() * c.toLong() - 2) / 3
}

fun chocolatePyramid(r: Int, c: Int): Pair<Long, Long> {
    // W(r, c) = (r * c) + (r-1) * (c-1) + D(r-1, c-1)
    // D(r, c) = (r - 1) * c + r * (c-1) + W(r-1, c-1)
    // therefore,
    // W(r, c) = (r * c) + 2*(r-1)*(c-1) ...(until r-k or c-k is equal to 0)
    // D(r, c) = (r - 1) * c + r * (c-1) + (r-2) * (c-1) + (r - 1) * (c-2)
    //(r*c) + (r-1) * c + r * (c-1) + (r-1) * (c-1)
    // S(r, c) = (2r-1)(2c-1) + S(r-1, c-1)

    val s = chocolatePyramidSum(r, c)
    val d = chocolatePyramidDark(r, c)
    return s - d to d
}

fun main() = with(System.`in`.bufferedReader()) {
    val t = this.readLine().toInt()
    repeat(t) {
        val (r, c) = this.readLine().split(" ").map { it.toInt() }

        println(chocolatePyramid(r, c).let {
            "${it.first} ${it.second}"
        })
    }
}