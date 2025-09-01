import java.io.StreamTokenizer

fun calcs(maxCycle: Int, calcs: List<Pair<Int, Int>>, query: List<Int>) {
    var sCalcs = calcs.sortedBy { it.first }

    


}


fun main() = with(StreamTokenizer(System.`in`.bufferedReader())) {
    nextToken()
    val n = nval.toInt()
    nextToken()
    val maxCycle = nval.toInt()

    val calcs = (1..n).map {
        nextToken()
        val s = nval.toInt()
        nextToken()
        val e = nval.toInt()
        Pair(s, e)
    }

    val q = nval.toInt()
    val times = (1..q).map {
        nextToken()
        nval.toInt()
    }
}