import java.io.StreamTokenizer

fun convertPSeq(k: Int, seq: List<Int>): List<Int> {
    val seqOrig = mutableListOf<Int>()

    repeat(seq[0]) {
        seqOrig.add(1)
    }
    var before = seq[0]
    for (i in 1 until k) {
        repeat(seq[i] - before) {
            seqOrig.add(i + 1)
        }
        before = seq[i]
    }
    return seqOrig;
}

fun main() = with(StreamTokenizer(System.`in`.bufferedReader())) {
    do {
        nextToken()
        val k = nval.toInt()
        if (k == 0) {
            break;
        }
        val pSeq = (1..k).map {
            nextToken()
            nval.toInt()
        }
        println(convertPSeq(k, pSeq).joinToString(" "))
    } while (true)
}
