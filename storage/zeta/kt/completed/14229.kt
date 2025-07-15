import kotlin.math.pow

// A C G T

class DNAIncludeManager {
    val dnaRegistry = BooleanArray(5460) { false }

    fun updateRegistry(dna: String) {
        val n = dna.length
        for (i in 0 until n) {
            for (len in 1..6) {
                if (i + len > n) {
                    break;
                }
                dnaRegistry[encodeDNA(dna.subSequence(i, i + len))] = true
            }
        }
    }

    fun getShortest(): String {
        dnaRegistry.withIndex().forEach { (i, c) ->
            if (!c) {
                return decodeDNA(i).joinToString("")
            }
        }
        return ""
    }

    companion object {
        fun convertDNAChar(c: Char): Int {
            return when (c) {
                'A' -> 0
                'C' -> 1
                'G' -> 2
                'T' -> 3
                else -> 0
            }

        }

        fun decodeDNAChar(x: Int): Char {
            return when (x) {
                0 -> 'A'
                1 -> 'C'
                2 -> 'G'
                3 -> 'T'
                else -> 'X'
            }
        }

        fun encodeDNA(seq: CharSequence): Int {
            val n = seq.length

            return (4.0.pow(n) - 4).toInt() / 3 + seq.withIndex().sumOf { (i, c) ->
                4.0.pow(i.toDouble()).toInt() * convertDNAChar(c)
            }
        }

        fun decodeDNA(x: Int): List<Char> {
            var target = x

            for (i in 1..6) {
                val cx = 4.0.pow(i).toInt()
                if (target < cx) {
                    val res = MutableList<Char>(i) { 'A' }
                    if (i != 1) {
                        target -= (4.0.pow(i) - 4).toInt() / 3
                    }

                    for (j in (0 until i).reversed()) {
                        val ct = 4.0.pow(j).toInt()
                        res[j] = decodeDNAChar(target / ct)
                        target %= ct
                    }

                    return res
                }
            }
            return listOf()
        }
    }
}

fun main() = with(System.`in`.bufferedReader()) {
    val manager = DNAIncludeManager()
    val s = this.readLine().trimEnd()

    manager.updateRegistry(s)
    println(manager.getShortest())
}