fun maxFoodEaten(cntsFoods: List<Int>): Int {
    val dp = Array(cntsFoods.size) { Array<Int>(3) { 0 } }

    // 0: 안먹을경우
    // 1: 첫번째인경우
    // 2: 두번째인경우

    for ((i, food) in cntsFoods.withIndex()) {
        if (i == 0) {
            dp[i][1] = food
            dp[i][2] = 0
            dp[i][0] = 0
        } else if (i == 1) {
            dp[i][2] = dp[i - 1][1] + food / 2
            dp[i][1] = dp[i - 1][0] + food
            dp[i][0] = dp[i - 1].max()
        } else {
            dp[i][0] = dp[i - 1].max()
            dp[i][1] = dp[i - 1][0] + food
            dp[i][2] = dp[i - 1][1] + food / 2
        }

    }

    return dp.last().max()
}

fun main() = with(System.`in`.bufferedReader()) {
    val n = this.readLine().toInt()
    val cntsFoods = (1..n).map {
        this.readLine().toInt()

    }

    println(maxFoodEaten(cntsFoods))
}