import kotlin.math.*

class NarockSolver(val s: String) {
    companion object {
        val HAXIM: Int = 1000000007
    }
    fun solve(): Int {
        var cntR = 0
        var cntO = 0
        var cntC = 0
        var cntS = 0
        var rfront = 1
        for (v: Char in s) {
            if (v == 'R') {
                cntR += rfront
                cntR %= HAXIM
            } else if (v == 'O') {
                cntO += cntR
                cntO %= HAXIM
            } else if (v == 'C') {
                cntC += cntO
                cntC %= HAXIM
            } else if (v == 'K') {
                cntS += cntC
                cntS %= HAXIM
            }
            rfront *= 2
            rfront %= HAXIM
        }

        return cntS
    }
}

fun main() {
    readLine()
    val s: String = readLine()!!
    val solver = NarockSolver(s)
    println(solver.solve())
}
