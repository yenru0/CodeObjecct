import kotlin.math.max
import kotlin.math.min

fun maxSplitDiscordance(s: String): Int {
    val n = s.length
    val dp = IntArray((n + 1) * (n + 1))

    fun getLCS(i: Int, j: Int): Int { // where i and j means string index
        return dp[(i + 1) * (n + 1) + (j + 1)]
    }

    fun setLCS(i: Int, j: Int, v: Int): Int {
        dp[(i + 1) * (n + 1) + (j + 1)] = v
        return v
    }

    var maxDiscordance = 0
    var lcs: Int
    val candidate = if (n % 2 == 0) {
        listOf(n / 2)
    } else {
        listOf(n / 2, n / 2 + 1)
    }
    for (splitLoc in candidate) {
        lcs = 0
        for (x in 0 until splitLoc) {
            setLCS(x, splitLoc - 1, 0)
        }


        for (i in 0 until splitLoc) {
            for (j in splitLoc until n) {

                if (s[i] == s[j]) {
                    lcs = max(setLCS(i, j, getLCS(i - 1, j - 1) + 1), lcs)
                } else {
                    lcs = max(
                        setLCS(
                            i, j, max(
                                getLCS(i, j - 1),
                                getLCS(i - 1, j)
                            )
                        ),
                        lcs
                    )
                }
            }
        }

        maxDiscordance = max(maxDiscordance, min(splitLoc, n - splitLoc) - lcs)
    }

    return maxDiscordance

}

fun main() {
    val s = readln()

    println(maxSplitDiscordance(s))
}