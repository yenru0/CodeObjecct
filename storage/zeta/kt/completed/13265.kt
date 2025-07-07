fun canBipartitioned(n: Int, edges: List<List<Int>>): Boolean {
    val colored = MutableList(n) { false }
    val visited = MutableList(n) { false }

    val deq = mutableListOf<Pair<Int, Boolean>>()

    var deadFlag = false
    (0 until n).forEach {
        deq.add(Pair(it, true))
    }

    while (deq.isNotEmpty()) {
        val (node, beforeColor) = deq.removeLast()
        if (visited[node]) {
            continue
        }
        visited[node] = true
        val nowColor = !beforeColor
        colored[node] = nowColor

        for (next in edges[node]) {
            if (visited[next] && colored[next] == nowColor) {
                deadFlag = true
                break
            }

            if (!visited[next]) {
                deq.add(Pair(next, nowColor))
            }
        }

        if (deadFlag) {
            break
        }
    }

    return !deadFlag
}


fun main() = with(System.`in`.bufferedReader()) {
    (1..readln().toInt()).forEach { _ ->
        val (n /* 동그라미 개수*/, m/* 직선 개수*/) = readln().split(' ').map { i -> i.toInt() }
        val edges: List<List<Int>> = (1..m).map {
            readln().split(' ').map { i -> i.toInt() }.let {
                Pair(it[0] - 1, it[1] - 1)
            }
        }.let {
            val res: MutableList<MutableList<Int>> = MutableList(n, { mutableListOf() })
            it.forEach {
                res[it.first].add(it.second)
                res[it.second].add(it.first)
            }
            res
        }

        println(
            if (canBipartitioned(n, edges)) {
                "possible"
            } else {
                "impossible"
            }
        )
    }
}