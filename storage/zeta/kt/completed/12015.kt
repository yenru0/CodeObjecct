fun getLengthLIS(arr: List<Int>): Int {
    val lisProps = mutableListOf<Int>()

    arr.forEach {
        val x = lisProps.binarySearch(it).let { it ->
            if (it < 0) {
                -(it + 1)
            } else {
                it
            }
        }
        if (x == lisProps.size) {
            lisProps.add(it)
        } else {
            lisProps[x] = it
        }
    }

    return lisProps.size
}


fun main() {
    readln()
    val arr = readln().split(' ').map { it.toInt() }

    println(getLengthLIS(arr))
}