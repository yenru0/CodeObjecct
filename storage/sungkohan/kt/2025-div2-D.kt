import java.io.StreamTokenizer

fun main() = with(StreamTokenizer(System.`in`.bufferedReader())) {
    nextToken()
    val n = nval.toInt()
    nextToken()
    val m = nval.toInt()
    val edges = (1..m).map {
        nextToken()
        val u = nval.toInt() - 1
        nextToken()
        val v = nval.toInt() - 1
        nextToken()
        val w = nval.toLong()
    }
}