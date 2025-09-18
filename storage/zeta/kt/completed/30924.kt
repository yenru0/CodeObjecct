fun main() = with(System.`in`.bufferedReader()) {
    val a = run {
        (1..9999).shuffled().forEach {
            println("? A $it")
            if (this.readLine().toInt() == 1) {
                return@run it
            }
        }
        10000
    }

    val b: Int = run {
        val cand = (1..10000).shuffled()
        cand.slice(0 until 9998).forEach {
            println("? B $it")
            if (this.readLine().toInt() == 1) {
                return@run it
            }
        }
        cand.last()
    }

    println("! ${a + b}")
}