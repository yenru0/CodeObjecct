fun bfs(N: Int): Int {
    val D = ArrayDeque<Pair<Int, Int>>()
    D.addLast(Pair(N, 0))

    while (D.isNotEmpty()) {
        val (now, step) = D.removeFirst()
        if (now == 1) {
            return step
        }
        if (now % 3 == 0) {
            D.addLast(Pair(now / 3, step + 1))
        }
        if (now % 2 == 0) {
            D.addLast(Pair(now / 2, step + 1))
        }
        D.addLast(Pair(now - 1, step + 1))
    }
    return -1
}

fun main() {
    val N = readln().toInt()
    println(bfs(N));
}