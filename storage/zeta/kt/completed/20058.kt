import java.io.StreamTokenizer
import kotlin.math.max

fun Array<Array<Int>>.deepClone(): Array<Array<Int>> = Array(size) { get(it).clone() }


fun resultFirestorm(n: Int, ices: Array<Array<Int>>, levels: List<Int>): Pair<Int, Int> {
    var curIces = ices.deepClone()
    var nextIces = ices.deepClone()
    val iceSide = 1 shl n

    for (level in levels) {
        val side = 1 shl level
        val ix = iceSide / side

        // tornado
        for (rb in 0 until ix) {
            for (cb in 0 until ix) {
                for (rs in 0 until side) {
                    for (cs in 0 until side) {
                        val cur = curIces[rs + rb * side][cs + cb * side] // 0, 1
                        nextIces[rb * side + cs][cb * side + (side - 1) - rs] = cur
                    }
                }
            }
        }
        curIces = nextIces.deepClone()

        // fire update
        for (r in 0 until iceSide) {
            for (c in 0 until iceSide) {
                if (curIces[r][c] == 0) {
                    continue
                }
                var adjs = 0
                if (r - 1 >= 0) {
                    if (curIces[r - 1][c] > 0) adjs++
                }
                if (r + 1 < iceSide) {
                    if (curIces[r + 1][c] > 0) adjs++
                }
                if (c - 1 >= 0) {
                    if (curIces[r][c - 1] > 0) adjs++
                }
                if (c + 1 < iceSide) {
                    if (curIces[r][c + 1] > 0) adjs++
                }

                if (adjs <= 2) nextIces[r][c]--
            }
        }
        curIces = nextIces.deepClone()
    }

    val vis = Array(iceSide) { Array(iceSide) { false } }
    var maxJoint = 0
    for (r in 0 until iceSide) {
        for (c in 0 until iceSide) {
            if (vis[r][c]) continue
            if (curIces[r][c] == 0) {
                vis[r][c] = true
                continue
            }

            val q = ArrayDeque<Pair<Int, Int>>()
            q.addLast(r to c)
            var joint = 0
            while (q.isNotEmpty()) {
                val (now_r, now_c) = q.removeLast()
                if (vis[now_r][now_c]) continue
                vis[now_r][now_c] = true
                if (curIces[now_r][now_c] == 0) {
                    continue
                }
                joint++


                if (now_r + 1 < iceSide) {
                    q.addLast(now_r + 1 to now_c)
                }
                if (now_c + 1 < iceSide) {
                    q.addLast(now_r to now_c + 1)
                }
                if (now_r - 1 >= 0) {
                    q.addLast(now_r - 1 to now_c)
                }
                if (now_c - 1 >= 0) {
                    q.addLast(now_r to now_c - 1)
                }
            }

            maxJoint = max(joint, maxJoint)
        }
    }


    return curIces.sumOf { it.sum() } to maxJoint
}


fun main() = with(StreamTokenizer(System.`in`.bufferedReader())) {
    nextToken()
    val n = nval.toInt()
    nextToken()
    val q = nval.toInt()

    val side = 1 shl n

    val ices = Array(side) { Array(side) { 0 } }
    for (i in 0 until side) {
        for (j in 0 until side) {
            nextToken()
            ices[i][j] = nval.toInt()
        }
    }

    val levels = List(q) {
        nextToken()
        nval.toInt()
    }

    resultFirestorm(n, ices, levels).run {
        println(first)
        println(second)
    }
}