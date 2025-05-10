fun solve(N: Int, A: Array<Int>): Int {
    val T = Array<Triple<Int, Int, Int>>(N) {
        Triple(0, 0, 0)
    } // 0 안 마시기 ,1번째 마시기, 2번째 마시기

    T[0] = Triple(0, A[0], 0)

    for (i in 1 until N) {
        T[i] = Triple(
            maxOf(T[i - 1].first, T[i - 1].second, T[i - 1].third), T[i - 1].first + A[i], T[i - 1].second + A[i]
        )
    }

    return maxOf(T[N - 1].first, T[N - 1].second, T[N - 1].third)
}

fun main() {
    val N = readln().toInt()
    val A = Array<Int>(N) {
        readln().toInt()
    }
    println(solve(N, A))

}