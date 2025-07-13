import java.io.StreamTokenizer

const val K: Long = 1_000_000_007

fun find(parents: IntArray, a: Int): Int {
    if (parents[a] == a) {
        return a
    }

    parents[a] = find(parents, parents[a]);

    return parents[a]
}

fun union(parents: IntArray, a: Int, b: Int) {
    val ra = find(parents, a)
    val rb = find(parents, b)

    if (ra == rb) {
        return
    } else if (ra < rb) {
        parents[rb] = ra
    } else {
        parents[ra] = rb
    }

}


fun intervalPollutionArea(polluted: List<Long>): Pair<Int, Int> {
    val n = polluted.size
    val parents = IntArray(n) { it }
    val poses = ArrayList<Long>()

    var now_x = 1L
    var before_y = 0L

    for (p in polluted) {
        if (p <= before_y) now_x++
        before_y = p
        poses.add(K * now_x + p)
    }

    for ((i, p) in poses.withIndex()) {
        val px = p / K
        val py = p % K
        val dx = listOf(
            K * (px + 1) + py,
            p + 1,
        )

        for (x in dx) {
            val findPos = poses.binarySearch(x)
            if (findPos > 0) {
                union(parents, i, findPos)
            }
        }
    }

    return Pair(parents.map { find(parents, it) }.distinct().count(), n)
}

fun main() = with(StreamTokenizer(System.`in`.bufferedReader())) {
    nextToken()
    val n = nval.toInt()

    val polluted = (1..n).map {
        nextToken()
        this.nval.toLong()
    }

    intervalPollutionArea(polluted).run {
        println(this.first)
        println(this.second)
    }
}