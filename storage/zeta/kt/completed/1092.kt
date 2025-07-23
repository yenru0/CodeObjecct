fun <T : Comparable<T>> List<T>.lowerBound(value: T): Int {
    var left = 0
    var right = this.size

    while (left < right) {
        val mid = (left + right) / 2
        if (this[mid] < value) {
            left = mid + 1
        } else {
            right = mid
        }
    }
    return left
}

fun minimumPortTime(weightLimits: List<Int> /* sorted */, weights: List<Int>): Int? {

    val weightsCntCategorized = IntArray(weightLimits.size + 1)


    weights.forEach {
        val cat = weightLimits.lowerBound(it)
        weightsCntCategorized[cat]++
    }

    if (weightsCntCategorized.last() > 0) {
        return null
    }

    var time = 0
    while (true) {
        repeat(weightLimits.size) {
            for (i in (0..it).reversed()) {
                if (weightsCntCategorized[i] == 0) {
                    continue
                } else {
                    weightsCntCategorized[i]--
                    break
                }
            }
        }

        time++
        if (weightsCntCategorized.all { it == 0 }) {
            break
        }
    }


    return time
}

fun main() = with(System.`in`.bufferedReader()) {
    this.readLine()

    val weightLimits = this.readLine().split(' ').map { it.toInt() }.sorted()
    this.readLine()

    val weights = this.readLine().split(' ').map { it.toInt() }

    println(
        minimumPortTime(weightLimits, weights) ?: -1
    )
}