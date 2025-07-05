import kotlin.math.*


val globalFibonacciSpace = Array<Long>(100, { i ->
    if (i == 0 || i == 1) {
        1
    } else {
        0
    }
})

fun fib(i: Int): Long {
    var spaceRef = globalFibonacciSpace[i];
    if (spaceRef == 0L) {
        spaceRef = fib(i - 1) + fib(i - 2)
        globalFibonacciSpace[i] = spaceRef
    }
    return spaceRef
}


fun getCreatingFibs(n: Long): List<Long> {
    // 제켄도르프 정리에 의해 이 표현이 항상 존재함을 보증할 수 있음
    var curr = 90
    var now_counter = n
    var cfibs = mutableListOf<Long>()
    while (curr >= 1) {
        if (fib(curr) <= now_counter) {
            now_counter -= fib(curr)
            cfibs.add(fib(curr))
            if (now_counter == 0L) {
                break;
            }
            curr--;
        }

        curr--;
    }

    return cfibs

}

fun main() {
    val n = readln().trim().toLong()

    run {
        val t = getCreatingFibs(n)
        println(t.size)
        t

    }.reversed().forEach { i ->
        print("$i ")
    }
    println()
}