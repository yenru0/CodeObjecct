fun canWin(n: Int): Boolean {
    return n % 2 != 0
}

fun main() = with(System.`in`.bufferedReader()) {
    val n = this.readLine().toInt()

    System.out.bufferedWriter().use {
        it.write(
            if (canWin(n)) {
                "SK"
            } else {
                "CY"
            }
        )
    }
}