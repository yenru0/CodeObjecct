fun main() = with(System.`in`.bufferedReader()) {
    val (n, m, q) = this.readLine()!!.split(' ').map { it.toInt() }

    val characters = (1..n).map { IntArray(m) }

    repeat(n) {
        val trait = this.readLine()!!

        for (i in 0 until m) {
            characters[it][i] = when (trait[i]) {
                'Y' -> 2
                'N' -> 1
                else -> 0
            }
        }
    }

    val queries = IntArray(m)

    repeat(q) {
        val iter = this.readLine()!!.split(' ').iterator()
        val index = iter.next().toInt() - 1
        queries[index] = when (iter.next().first()) {
            'Y' -> 2
            'N' -> 1
            else -> 0
        }
    }

    val filtered = characters.withIndex().filter { (ind, it) ->
        (it zip queries).map {
            if (it.second == 0) {
                true
            } else {
                it.first == it.second
            }
        }.all { it }
    }

    if (filtered.size == 1) {
        println("unique")
        println(filtered.first().index + 1)
    } else {
        println("ambiguous")
        println(filtered.size)
    }
}