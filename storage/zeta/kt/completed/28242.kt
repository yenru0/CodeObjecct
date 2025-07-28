fun divisors(n: Long): List<Pair<Long, Long>> {
    val res = mutableListOf<Pair<Long, Long>>()

    var i = 1L

    while (i * i <= n) {
        if (n % i == 0L) {
            res.add(i to n / i)
            if (i != n / i) {
                res.add(n / i to i)
            }
        }
        i++
    }

    return res
}


fun main() {
    val n = readln().toLong()

    // nx^2 + (n+1)x -(n+2)
    // = (ax+b)(cx+d)
    // = acx^2 + (ad + bc)x + bd

    // 다음 조건을 만족하는 a > 0, b, c > 0,d를 구하는 프로그램을 작성하시오.
    // n = a * c
    // (n+1) = a * d + b * c
    // b * d = -(n+2)


    divisors(n).flatMap { first ->
        divisors(n + 2).let {
            it.map {
                it.first to -it.second
            } + it.map {
                -it.first to it.second
            }
        }.map {
            first to it
        }
    }.firstOrNull { (first, second) ->
        val (a, c) = first
        val (b, d) = second
        n + 1 == a * d + b * c
    }?.let { (first, second) ->
        val (a, c) = first
        val (b, d) = second
        println("$a $b $c $d")
    } ?: println(-1)
}