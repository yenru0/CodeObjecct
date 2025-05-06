import kotlin.math.min

fun solve(N: Int, W: Array<Array<Int>>): Int {
    val D = ArrayDeque<Triple<Int, Int, Array<Int>>>() // now, cost, visited
    D.addLast(Triple(0, 0, Array(N) { 0 }.also { it[0] = 1 }))

    var min_cost = Int.MAX_VALUE

    while (D.isNotEmpty()) {
        val (now, cost, visited) = D.removeLast()
        if (visited.all { it == 1 }) {
            if (W[now][0] > 0) min_cost = min(min_cost, cost + W[now][0])
            continue
        } else {
            for (i in 1 until N) {
                if (visited[i] == 0 && W[now][i] > 0) {
                    D.addLast(Triple(i, W[now][i] + cost, visited.copyOf().also { it[i] = 1 }))
                }
            }
        }

    }
    return min_cost
}


fun main() {
    val N = readln().toInt()
    val W = Array<Array<Int>>(N) { // 0 to N-1
        readln().split(' ').map(String::toInt).toTypedArray()
    }

    println(solve(N, W))
}