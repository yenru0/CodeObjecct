import java.io.StreamTokenizer

fun main() = with(StreamTokenizer(System.`in`.bufferedReader())) {
    nextToken()
    val n = nval.toInt()
    val lisProps = IntArray(n)
    var size = 0
    (1..n).forEach { _ ->
        nextToken()
        val it = nval.toInt()
        val x = lisProps.binarySearch(it, 0, size).let { it ->
            if (it < 0) {
                -(it + 1)
            } else {
                it
            }
        }
        if (x == size) {
            size++
        }
        lisProps[x] = it

    }
    println(size)
}