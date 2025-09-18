fun main() = with(System.`in`.bufferedReader()) {
    val a: Int = run {
        for (i in 1 until 10) {
            println("? A $i")

            if (this.readLine().toInt() == 1) {
                return@run i
            }
        }
        0
    }

    val b: Int = run {
        for (i in 1 until 10) {
            println("? B $i")

            if (this.readLine().toInt() == 1) {
                return@run i
            }
        }
        0
    }

    println("! ${a + b}")
}