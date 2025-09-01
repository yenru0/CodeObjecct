fun main() = with(System.`in`.bufferedReader()) {
    val n = this.readLine().toInt()
    var locations = Array(n * n) { 0 to 0 }
    for (i in 0 until n) {
        this.readLine().split(" ").map { it.toInt() }.forEachIndexed { j, v ->
            locations[v - 1] = i to j
        }
    }

    //

    var boxLeftTop = locations[0].first to locations[0].second
    var boxRightTop = locations[0].first to locations[0].second
    var boxLeftDown = locations[0].first to locations[0].second
    var boxRightDown = locations[0].first to locations[0].second

    var cnt = 1

    for (item in 1 until n * n) {
        val row = locations[item].first
        val col = locations[item].second

        if (boxLeftTop.first <= row && boxLeftTop.second >= col) {
            boxLeftTop = row to col
        }
        if (boxRightTop.first <= row && boxRightTop.second <= col) {
            boxRightTop = row to col
        }
        if (boxLeftDown.first >= row && boxLeftDown.second >= col) {
            boxLeftDown = row to col
        }
        if (boxRightDown.first >= row && boxRightDown.second <= col) {
            boxRightDown = row to col
        }
        if (boxLeftTop.first == boxRightTop.first && boxLeftDown.first == boxRightDown.first) {
            if (boxLeftTop.second == boxLeftDown.second && boxRightTop.second == boxRightDown.second) {
                if ((boxRightTop.first - boxLeftDown.first + 1) * (boxRightTop.second - boxLeftDown.second + 1) == item + 1) {
                    cnt++
                }
            }
        }
    }

    println(cnt)
}