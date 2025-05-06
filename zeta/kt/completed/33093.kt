import java.io.*

class GoldenTicketSolver(
        val n: Int,
        val m: Int,
        val k: Int,
        val teams: List<Pair<String, String>>
) {

    fun solve(): List<String> {
        val goldenticked = mutableListOf<String>()
        val inst_goldenticked = mutableMapOf<String, Int>()
        var kcnt = 0
        for ((index, teaminst) in this.teams.withIndex()) {
            if (index < this.m) {
                inst_goldenticked[teaminst.second] = 1
            } else {
                if (kcnt >= this.k) {
                    break
                }
                val tmp = inst_goldenticked.getOrDefault(teaminst.second, 0)
                if (tmp == 0) {
                    inst_goldenticked[teaminst.second] = 2
                    goldenticked.add(teaminst.first)
                    kcnt += 1
                } else if (tmp == 1) {
                    continue
                } else if (tmp == 2) {
                    continue
                }
            }
        }
        return goldenticked
    }
}

fun main() {
    val tknz = StreamTokenizer(System.`in`.bufferedReader())

    fun StreamTokenizer.nextInt(): Int {
        nextToken()
        return nval.toInt()
    }
    fun StreamTokenizer.nextString(): String {
        nextToken()
        return sval
    }

    val (n, m, k) = (1..3).map { tknz.nextInt() }
    val tms = (1..n).map { Pair(tknz.nextString(), tknz.nextString()) }
    val solver = GoldenTicketSolver(n, m, k, tms)

    val solved = solver.solve()
    println(solved.size)
    solved.forEach { i -> println(i) }
}
