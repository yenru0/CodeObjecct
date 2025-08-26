fun getMastermindFeedback(n: Int, secret: String, guess: String): Pair<Int, Int> {
    var r: Int = 0
    var s: Int = 0

    val secretChecked = BooleanArray(n) { false }
    val guessChecked = BooleanArray(n) { false }

    for (i in 0 until n) {
        if (secret[i] == guess[i]) {
            r++
            secretChecked[i] = true
            guessChecked[i] = true
        }
    }

    for (i in 0 until n) {
        for (j in 0 until n) {
            if(secret[i] == guess[j] && !secretChecked[i] && !guessChecked[j]) {
                secretChecked[i] = true
                guessChecked[j] = true
                s++
                break
            }
        }
    }

    return r to s
}


fun main() = with(System.`in`.bufferedReader()) {
    val (n, a, b) = this.readLine().split(" ").let {
        Triple(it[0].toInt(), it[1].trim(), it[2].trim())
    }

    println(getMastermindFeedback(n, a, b).toList().joinToString(" "))
}