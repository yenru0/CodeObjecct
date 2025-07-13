fun getMissingISBN(numbers: List<Int>, missingIndex: Int): Int {
    val check = numbers.last()

    val others = (numbers.slice(0 until 12).withIndex().sumOf {
        if (it.index % 2 == 0) {
            it.value
        } else {
            3 * it.value
        }
    } + check) % 10
    val res = if (missingIndex % 2 == 0) {
        if (others == 0) {
            0
        } else {
            10 - others
        }
    } else {
        if (others == 0) {
            0
        } else {
            if ((10 - others) % 3 == 0) {
                (10 - others) / 3
            } else if ((20 - others) % 3 == 0) {
                (20 - others) / 3
            } else {
                (30 - others) / 3
            }
        }
    }
    return res
}

fun main() {
    var missingIndex = -1
    val numbers = readln().trim().withIndex().map {
        if (it.value == '*') {
            missingIndex = it.index
            0
        } else {
            it.value.digitToInt()
        }
    }
    println(getMissingISBN(numbers, missingIndex))
}