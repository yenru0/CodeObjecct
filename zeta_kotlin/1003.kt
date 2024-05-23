import kotlin.text.toInt

operator fun Pair<Int, Int>.plus(b: Pair<Int, Int>) = Pair<Int, Int>(first + b.first, second + b.second)


val Mem = Array<Pair<Int, Int>>(50) { idx ->
    if (idx == 0) {
        Pair<Int, Int>(1, 0)
    } else if (idx == 1) {
        Pair<Int, Int>(0, 1)
    } else {
        Pair<Int, Int>(0, 0)
    }
}

fun getFib(n: Int): Pair<Int, Int> {

    val m = Mem[n];
    if (m.first == 0 && m.second == 0) {
        Mem[n] = getFib(n - 1) + getFib(n - 2)
        return Mem[n]
    } else {
        return m;
    }
}

fun main() {
    val T = readLine()!!.toInt()
    for (i in 1..T) {
        val N = readLine()!!.toInt()

        println(getFib(N).toList().joinToString(" "))
    }
}