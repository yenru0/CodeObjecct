fun diffStr(s1: String, s2: String) =
    (s1.toList() zip s2.toList()).count { it.first != it.second }

fun 비밀번호변경을위한최소수정횟수(pwdOld: String, k: Int): Int {
    val n = pwdOld.length

    if (2 * k <= n) { // <분리> 케이스
        return diffStr(pwdOld.substring(0, k), pwdOld.substring(n - k))
    } else {
        var modify = 0
        val space = n - k
        for (i in 0 until space) {
            var pos = i
            val cnts = IntArray(26) { 0 }

            while (pos < n) {
                cnts[pwdOld[pos].code - 'a'.code]++
                pos += space
            }

            val maxSent = cnts.withIndex().maxBy { it.value }

            modify += cnts.sum() - maxSent.value
        }
        return modify
    }

}

fun main() = with(System.`in`.bufferedReader()) {
    val pwdOld = this.readLine().trim()
    val k = this.readLine().toInt()

    println(
        비밀번호변경을위한최소수정횟수(pwdOld, k)
    )
}