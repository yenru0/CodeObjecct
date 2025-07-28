fun main() = with(System.`in`.bufferedReader()) {
    val iter = this.readLine().split(" ").iterator()

    val m = iter.next().toLong()
    val seed = iter.next().toLong()
    val x1 = iter.next().toLong()
    val x2 = iter.next().toLong()

    // x1 = (a * seed + c) % m
    // x2 = (a * x1 + c) % m

    (0 until m).asSequence().map { a ->
        val t = (a * seed) % m
        val c = if (t <= x1) {
            x1 - t
        } else {
            m - (t - x1)
        }
        a to c
    }.first { (a, c) ->
        x2 == (a * x1 + c) % m
    }.let { (a, c) ->
        println("$a $c")
    }
}