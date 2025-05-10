import java.util.PriorityQueue
import java.util.Scanner

fun solve(
    N: Int,
    M: Int,
    D: MutableList<Int>,
    E: HashMap<Int, MutableList<Pair<Int, Int>>>,
    start: Int,
    end: Int
): Triple<Int, Int, List<Int>> {
    val pq = PriorityQueue<Triple<Int, Int, List<Int>>> { a, b -> a.first.compareTo(b.first) } // min heap
    pq.add(Triple(0, start, listOf(start)))
    D[start] = 0

    val ret: MutableList<Triple<Int, Int, List<Int>>> = mutableListOf()

    while (pq.isNotEmpty()) {
        val (cost, now, path) = pq.remove()

        if (cost > D[now]) {
            continue
        }
        if (now == end) {
            return Triple(cost, path.size, path)
        }
        for ((v, w) in E[now]!!) {
            val nd = cost + w
            if (D[v] > nd) {
                D[v] = nd
                pq.add(Triple(nd, v, path + listOf(v)))
            }
        }
    }

    println(D.toString())
    return ret.last()
}

fun main() = with(Scanner(System.`in`)) {
    val N = nextLine().toInt()
    val D = MutableList<Int>(N + 1, { Int.MAX_VALUE });
    val M = nextLine().toInt()

    val E: HashMap<Int, MutableList<Pair<Int, Int>>> = HashMap()
    for (i in 0..N) {
        E[i] = mutableListOf()
    };
    for (i in 0 until M) {
        val v = nextLine().trim().split(' ').map(String::toInt)
        E[v.first()]!!.add(v[1] to v[2])

    }
    val (start, end) = nextLine().trim().split(' ').map(String::toInt)

    val result = solve(start, M, D, E, start, end)
    println(result.first)
    println(result.second)
    println(result.third.joinToString(separator = " "))
}