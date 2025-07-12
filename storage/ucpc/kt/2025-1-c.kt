fun minCostAvoid(intervals: List<Pair<Int, Int>>, laserRanges: List<Pair<Int, Int>>): List<Pair<Int, Int>> {
    val (n: Int, q: Int) = Pair(intervals.size, laserRanges.size)

    
}

fun main() = with(System.`in`.bufferedReader()) {
    val (n, q) = this.readLine().split(' ').map { it.toInt() }

    val intervals = (1..n).map {
        val split = this.readLine().split(' ').map { it.toInt() }
        Pair(split[0], split[1])
    }

    val laserRanges = (1..q).map {
        val split = this.readLine().split(' ').map { it.toInt() }
        Pair(split[0], split[1])
    }

}