fun solve(N: Int, S: Int, I: List<Int>): Int {
    val D = ArrayDeque<Pair<Int, Int>>()
    D.addLast(Pair(0, 0))

    var cnt = 0

    while (D.isNotEmpty()) {
        val (now, accum) = D.removeLast()
        if (now == N) {
            if (accum == S) {
                cnt += 1
            }
            continue
        } else {
            D.addLast(Pair(now + 1, accum)) // 안 더하기
            D.addLast(Pair(now + 1, accum + I[now]))

        }

    }

    if (S == 0) {
        cnt -= 1;
    }

    return cnt
}

fun main() {
    val (N, S) = readln().split(' ').map(String::toInt)
    val I = readln().split(' ').map(String::toInt)
    println(solve(N, S, I))
}