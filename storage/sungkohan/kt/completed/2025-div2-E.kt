import java.util.Stack

fun countNearestFarRelation(n: Int, edges: Array<MutableList<Pair<Int, Int>>>): Long {
    val originXor = IntArray(n)

    val deq = Stack<Pair<Int, Int>>()
    deq.add(0 to 0)
    while (deq.isNotEmpty()) {
        val (now, before) = deq.pop()

        for ((nxt, w) in edges[now]) {
            if (nxt == before) {
                continue
            } else {
                originXor[nxt] = originXor[now] xor w
                deq.add(nxt to now)
            }
        }
    }
    originXor.sort()
    var cnt: Long = 0
    var before = -1
    var cntSliced: Long = 0
    for (i in originXor) {
        if (i == before) {
            cntSliced++
        } else {
            cnt += (cntSliced) * (cntSliced - 1) / 2
            before = i
            cntSliced = 1
        }
    }
    cnt += (cntSliced) * (cntSliced - 1) / 2

    return cnt
}

fun main() = with(System.`in`.bufferedReader()) {
    val n = this.readLine().toInt()
    var edges: Array<MutableList<Pair<Int, Int>>> = Array(n + 1) { mutableListOf() }
    for (i in 0 until (n - 1)) {
        val (u, v, w) = this.readLine().split(" ").map { it.toInt() }
        edges[u - 1].add(v - 1 to w)
        edges[v - 1].add(u - 1 to w)
    }

    println(countNearestFarRelation(n, edges))

}