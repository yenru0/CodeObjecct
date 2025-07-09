fun minimumTimeTaken(times: List<Int>): Int {
    var total = 0;
    return times.sorted().sumOf {
        total += it
        total
    }
}


fun main() = with(System.`in`.bufferedReader()) {
    readLine()
    val times = this.readLine().split(' ').map { it.toInt() }
    println(minimumTimeTaken(times))
}