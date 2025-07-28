import java.io.StreamTokenizer


fun find(parents: IntArray, x: Int): Int {
    return if (parents[x] == x) {
        x
    } else {
        parents[x] = find(parents, parents[x])
        parents[x]
    }
}

fun union(parents: IntArray, a: Int, b: Int): Boolean {
    val ra = find(parents, a)
    val rb = find(parents, b)

    if (ra == rb) {
        return false
    } else if (ra < rb) {
        parents[rb] = ra
    } else {
        parents[ra] = rb
    }
    return true
}


fun minimumDistanceOfExploration(n: Int, edges: List<Triple<Int, Int, Int>>): Int {
    val par = IntArray(n) { it }
    var minDist = 0
    edges.sortedBy { it.third }.forEach {
        val (a, b, w) = it
        if (union(par, a, b)) {
            minDist += w
        }
    }

    return minDist
}


fun main() = with(StreamTokenizer(System.`in`.bufferedReader())) {
    nextToken()
    val t = nval.toInt()

    (1..t).forEach {
        nextToken()
        val n = nval.toInt()
        nextToken()
        val m = nval.toInt()

        val edges = (1..m).map {
            nextToken()
            val s = nval.toInt() - 1
            nextToken()
            val e = nval.toInt() - 1
            nextToken()
            val w = nval.toInt()

            Triple(s, e, w)
        }

        println("Case #${it}: ${minimumDistanceOfExploration(n, edges)} meters")

    }
}