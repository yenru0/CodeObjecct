import java.io.StreamTokenizer
import kotlin.math.pow


class BinomialSuccessiveCalculator(val p: Double /* ensure 1 > p > 0*/, val n: Int /*300 >= n >= 1*/) {
    var cumulative: Double = 0.0
    var before: Double = 0.0
    var k: Int = -1

    val p_ratio = p / (1.0 - p)
    fun next() {

        if (k == -1) {
            before = (1 - this.p).pow(n)
        } else {
            before = before * (n - k).toDouble() / (k + 1).toDouble() * p_ratio
        }

        cumulative += before
        k++
    }

    fun minTrialToTrust(): Int {
        while (cumulative < 0.05) {
            next()
        }
        return this.k
    }
}

fun main() = with(StreamTokenizer(System.`in`.bufferedReader())) {
    nextToken()
    val 타율 = nval
    nextToken()
    val 타석 = nval.toInt()


    val bsc = BinomialSuccessiveCalculator(타율, 타석)
    println(bsc.minTrialToTrust())


}