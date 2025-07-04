import java.io.StreamTokenizer

fun main() = with(StreamTokenizer(System.`in`.bufferedReader())) {
    this.resetSyntax()
    this.wordChars('a'.code, 'z'.code)
    this.wordChars('A'.code, 'Z'.code)
    this.whitespaceChars(0, 'A'.code - 1)
    this.whitespaceChars('z'.code + 1, 255)

    val words = mutableSetOf<String>()

    var gameCount = 0
    var wordCount = 0
    while (nextToken() != StreamTokenizer.TT_EOF) {
        if (sval == null) {
            continue
        }
        val potentialWord = sval.trim()
        if (potentialWord == "BULLSHIT") {
            gameCount += 1
            wordCount += words.count()
            words.clear()
        } else {
            words.add(potentialWord.lowercase())
        }
    }

    (2..500).forEach { i ->
        while (wordCount % i == 0 && gameCount % i == 0) {
            wordCount /= i; gameCount /= i
        }
    }

    println("$wordCount / $gameCount")
}