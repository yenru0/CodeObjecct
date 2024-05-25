val Mem = Array<Int>(1001) { i ->
    if (i == 1) {
        1
    } else if (i == 2) {
        3
    } else {
        0
    }
}

fun get(N: Int): Int {
    if (Mem[N] != 0) {
        return Mem[N]
    } else {
        Mem[N] = (get(N - 1) + 2 * get(N - 2)) % 10007
        return Mem[N]
    }
}


fun main() {
    val N = readln().toInt()
    println(get(N))
}