fun getMaximumEfficientModules(modules: List<Long>): List<Int> {
    val sortedModules = modules.withIndex().sortedBy { it.value }

    var maxEff = sortedModules[modules.size - 1].value * 3
    var curEff = maxEff
    var maxBound = modules.size - 1
    for (i in modules.size - 2 downTo 0) {
        val delta = 2 * sortedModules[i].value - sortedModules[i + 1].value
        curEff += delta
        if(curEff > maxEff) {
            maxEff = curEff
            maxBound = i
        }

    }
    return sortedModules.slice(maxBound until sortedModules.size).map {
        it.index + 1
    }
}

fun main() = with(System.`in`.bufferedReader()) {
    this.readLine()
    val modules = this.readLine().split(" ").map { it.toLong() }

    getMaximumEfficientModules(modules).run {
        println(this.size)
        println(this.joinToString(" "))
    }
}