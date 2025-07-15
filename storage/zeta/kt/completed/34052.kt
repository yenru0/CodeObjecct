import java.io.StreamTokenizer

fun main() = with(StreamTokenizer(System.`in`.bufferedReader())) {
    if ((1..4).sumOf {
            nextToken()
            nval.toInt()
        } <= 1500) {
        println("Yes")
    } else {
        println("No")
    }
}