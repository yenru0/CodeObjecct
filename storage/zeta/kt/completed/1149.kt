import kotlin.math.min

fun List<Int>.toTriple(): Triple<Int, Int, Int> {
    return Triple(this[0], this[1], this[2])
}


fun solve(N: Int, C: Array<Triple<Int, Int, Int>>): Int {
    val T = Array<Triple<Int, Int, Int>>(N) { Triple(0, 0, 0) }
    T[0] = C[0]

    for (i in 1 until N) {
        T[i] = Triple(
            min(T[i - 1].second, T[i - 1].third) + C[i].first,
            min(T[i - 1].first, T[i - 1].third) + C[i].second,
            min(T[i - 1].first, T[i - 1].second) + C[i].third
        )
    }

    return minOf(T[N - 1].first, T[N - 1].second, T[N - 1].third)
}

fun main() {
    val N = readln().toInt()
    val C = Array<Triple<Int, Int, Int>>(N) {
        readln().split(' ').map { s: String ->
            s.toInt()
        }.toTriple()
    }

    println(solve(N, C))
}