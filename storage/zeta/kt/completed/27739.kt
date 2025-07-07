import kotlin.collections.sortedBy


fun generate(arr: List<Int>, i: Int, j: Int): List<List<Int>> {
    val low_ele = arr.getOrElse(i - 1, { Int.MIN_VALUE })
    val high_ele = arr.getOrElse(j, { Int.MAX_VALUE })
    val lists = mutableListOf<List<Int>>().also {
        it.add(
            arr.slice(0 until i)
                    + arr.slice(i until j)
                .sortedBy { item: Int ->
                    if (item > low_ele) {
                        Int.MIN_VALUE + item
                    } else {
                        item
                    }
                }
                    + arr.slice(j until arr.size)
        )
        it.add(
            arr.slice(0 until i)
                    + arr.slice(i until j)
                .sortedBy { item ->
                    if (item < high_ele) {
                        item
                    } else {
                        Int.MIN_VALUE + item
                    }
                }
                    + arr.slice(j until arr.size)
        )
    }
    return lists

}

fun getIncreasingLength(arr: List<Int>): Int {
    var before = -1
    var successive = 0

    return arr.fold(0) { accum, ele ->
        if (ele > before) {
            successive++
        } else {
            successive = 1
        }

        before = ele

        if (accum > successive) {
            accum
        } else {
            successive
        }
    }
}

fun getMaxRearrangedIncreasingLength(m: Int, arr: List<Int>): Int {
    return (0..arr.size - m).maxOf { i ->
        generate(arr, i, i + m).maxOf { item ->
            getIncreasingLength(item)
        }
    }
}


fun main() = with(System.`in`.bufferedReader()) {
    val (n: Int, m: Int) = readln().split(' ').map { i -> i.toInt() }
    val arr = readln().split(' ').map { i -> i.toInt() }.toList()

    println(getMaxRearrangedIncreasingLength(m, arr));
}