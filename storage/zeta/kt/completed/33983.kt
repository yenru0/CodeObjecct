fun isPureWaterString(s: String): Boolean {
    if (s.length % 3 != 0) {
        return false
    }
    var leftCntH = 0
    var rightCntH = 0
    var leftCntO = 0
    var rightCntO = 0

    for ((left, right) in s zip s.reversed()) {
        when (left) {
            'H' -> leftCntH++
            'O' -> leftCntO++
        }

        when (right) {
            'H' -> rightCntH++
            'O' -> rightCntO++
        }

        if (leftCntO > leftCntH) {
            return false
        }

        if (rightCntO > rightCntH) {
            return false
        }
    }

    return leftCntH == leftCntO * 2
}


fun main() = with(System.`in`.bufferedReader()) {
    this.readLine()
    val s = this.readLine()
    if (isPureWaterString(s)) {
        println("pure")
    } else {
        println("mix")
    }
}