enum class VarEdge() {
    DECREASING,
    STATIC,
    INCREASING;

    companion object {
        fun getEdgeFrom(before: Int, ahead: Int): VarEdge {
            return if (before - ahead > 0) {
                DECREASING
            } else if (before - ahead == 0) {
                STATIC
            } else {
                INCREASING
            }
        }
    }

}


fun getCountZigzagSubarray(arr: List<Int>): Long {

    val arrEdges = arr.slice(0 until arr.size - 1).zip(arr.slice(1 until arr.size)).map { (before, ahead) ->
        VarEdge.getEdgeFrom(before, ahead)
    }
    val intervals: MutableList<Pair<Int, Int>> = mutableListOf()

    var last = -1
    var beforeEdge = VarEdge.STATIC

    arrEdges.withIndex().forEach { (index, edge) ->
        if (edge == VarEdge.DECREASING) {
            if (beforeEdge == VarEdge.INCREASING) {

            } else if (beforeEdge == VarEdge.STATIC) {
                last = index
            } else {
                intervals.add(Pair(last, index))
                last = index
            }
        } else if (edge == VarEdge.INCREASING) {
            if (beforeEdge == VarEdge.DECREASING) {

            } else if (beforeEdge == VarEdge.STATIC) {
                last = index
            } else {
                intervals.add(Pair(last, index))
                last = index
            }
        } else {
            if (beforeEdge != VarEdge.STATIC) {
                intervals.add(Pair(last, index))
                last = index
            }
        }

        beforeEdge = edge
    }
    if (beforeEdge != VarEdge.STATIC) {
        intervals.add(Pair(last, arr.size - 1))
    }

    return intervals.sumOf {
        val n = (it.second - it.first).toLong()
        (n) * (n + 1) / 2
    }
}


fun main() = with(System.`in`.bufferedReader()) {
    val t = this.readLine().toInt()
    repeat(t) {
        this.readLine()
        val arr = this.readLine().split(' ').map { it.toInt() }
        println(getCountZigzagSubarray(arr))
    }
}