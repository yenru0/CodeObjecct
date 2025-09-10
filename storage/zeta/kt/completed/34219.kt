data class Coin(var index: Int, var type: Int, var state: Boolean) {

}


class GoldCoinDetector(val n: Int) {
    val coins = mutableListOf<Coin>()

    init {
        (0 until n).forEach {
            coins.add(Coin(it, 0, true))
        }
    }

    var response = -1

    var result = -1

    fun injectResult() {

        response = readLine()!!.toInt()

    }

    fun processResult() {
        // 0 1 2 3
        val normal = coins.sumOf { it.type * 9 }

        val select = response - normal
        coins.forEach {
            if (it.type != select) {
                it.state = false
            }
        }
    }

    fun preQuery(): Boolean {
        var cnt = 0
        var last = -1
        coins.forEach {
            it.type = 0
            if (it.state) {
                cnt++
                last = it.index
            }
        }

        if (cnt == 1) {
            result = last + 1 // index to real order
            return true
        }

        return false
    }

    fun query() {
        val query = IntArray(n)
        coins.filter { it.state }.withIndex().forEach {
            it.value.type = it.index % 6
        }

        coins.forEach {
            query[it.index] = it.type
        }

        println("? ${query.joinToString(" ")}")
        System.out.flush()
    }

    fun claimResult() {
        println("! $result")
        System.out.flush()
    }
}


fun main() {
    val n = readLine()!!.toInt()

    val detector = GoldCoinDetector(n)
    while (!detector.preQuery()) {
        detector.query()
        detector.injectResult()
        detector.processResult()
    }

    detector.claimResult()
}