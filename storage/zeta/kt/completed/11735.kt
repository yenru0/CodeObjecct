fun 미래예측정산소(n: Long, queries: List<Pair<Int, Long>>): List<Long> {
    val isRowed = BooleanArray(n.toInt()) { false }
    val isColumned = BooleanArray(n.toInt()) { false }
    var rowSum = 0L
    var rowCnt = 0L
    var columnSum = 0L
    var columnCnt = 0L
    return queries.map { (selector, index) ->
        if (selector == 0) { // row
            if (isRowed[index.toInt()]) {
                0
            } else {
                isRowed[index.toInt()] = true

                val value = (index + 2) * (n - columnCnt) - columnSum + (n * (n - 1)) / 2

                rowCnt++
                rowSum += index
                value
            }
        } else {
            if (isColumned[index.toInt()]) {
                0
            } else {
                isColumned[index.toInt()] = true

                val value = (index + 2) * (n - rowCnt) - rowSum + (n * (n - 1)) / 2

                columnCnt++
                columnSum += index
                value
            }
        }
    }
}

fun main() = with(System.`in`.bufferedReader()) {
    val (n, q) = this.readLine().split(" ").map { it.toInt() }

    val queries = (1..q).map {
        val queryLine = this.readLine().split(" ")
        val selectorChar = queryLine[0][0]
        val selector = when (selectorChar) {
            'R' -> {
                0
            }

            'C' -> {
                1
            }

            else -> {
                -1
            }
        }
        val value = queryLine[1].toLong()
        selector to value - 1 // index value로 변환
    }

    println(
        미래예측정산소(n.toLong(), queries).joinToString("\n")
    )


}