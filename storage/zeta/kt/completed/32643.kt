class SequenceMaster(n: Int) {
    val primes = mutableListOf<Int>()

    init {
        primes.add(2)
        for (i in 3..n step 2) {
            var flag = true
            for (p in primes) {
                if (p * p > i) {
                    break
                }
                if (i % p == 0) {
                    flag = false
                    break
                }
            }
            if (flag) {
                primes.add(i)
            }
        }
    }

    fun request(a: Int, b: Int): Int {
        val pa = primes.binarySearch(a)
        val pb = primes.binarySearch(b)

        return if (a == 1) {
            1
        } else {
            0
        } + if (pa >= 0 && pb >= 0) {
            pb - pa + 1
        } else if (pa < 0 && pb >= 0) {
            pb - (-pa - 1) + 1
        } else if (pa >= 0) {
            (-pb - 1) - pa
        } else {
            (-pb - 1) - (-pa - 1)
        }
    }
}

fun main() = with(System.`in`.bufferedReader()) {
    val (n, m) = this.readLine().split(' ').map { it.toInt() }
    val seq = SequenceMaster(n)
    System.out.bufferedWriter().use { bw ->
        repeat(m) {
            val (a, b) = this.readLine().split(' ').map { it.toInt() }
            bw.write("${seq.request(a, b)}\n")
        }
    }

}