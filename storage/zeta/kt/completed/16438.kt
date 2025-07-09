fun cluster(schedule: Array<Array<Boolean>>, depth: Int, lo: Int, hi: Int) {
    if (hi - lo <= 1) {
        return
    }
    val mid = (lo + hi) / 2
    for (i in lo until mid) {
        schedule[depth][i] = false
    }
    for (i in mid until hi) {
        schedule[depth][i] = true
    }
    cluster(schedule, depth + 1, lo, mid)
    cluster(schedule, depth + 1, mid, hi)
}

fun getWeeklySchedule(n: Int): Array<Array<Boolean>> {
    val schedule = Array(7, { i ->
        Array(n, { j ->
            if (j == 0) {
                true
            } else {
                false
            }
        })
    })
    cluster(schedule, 0, 0, n)
    return schedule
}

fun main() {
    println(
        getWeeklySchedule(readln().toInt()).joinToString(separator = "\n") { i ->
            i.joinToString(separator = "") {
                if (it) {
                    "B"
                } else {
                    "A"
                }
            }
        })
}