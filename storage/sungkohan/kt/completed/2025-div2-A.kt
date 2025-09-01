import java.io.StreamTokenizer

fun rectanglePermCount(n: Int, table: Array<Array<Int>>): Int {
    var cnt = 0
    for (r1 in 0 until n) {
        for (c1 in 0 until n) {
            for (r2 in r1 until n) {
                for (c2 in c1 until n) {
                    val list: MutableList<Int> = mutableListOf()
                    for (i in r1..r2) {
                        for (j in c1..c2) {
                            list.add(table[i][j])
                        }
                    }
                    list.sort()
                    if (list.withIndex().all { it.index + 1 == it.value }) {
                        cnt++
                    }
                }
            }
        }
    }
    return cnt
}

fun main() = with(StreamTokenizer(System.`in`.bufferedReader())) {
    nextToken()
    val n = nval.toInt()

    val table = Array(n) { Array(n) { 0 } }
    for (i in 0 until n) {
        for (j in 0 until n) {
            nextToken()
            table[i][j] = nval.toInt()
        }
    }

    println(rectanglePermCount(n, table))
}