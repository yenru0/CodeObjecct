fun main() = with(System.`in`.bufferedReader()) {
    this.readLines().map {
        val words = it.split(' ').toMutableList()
        val vowelStarts =
            words.withIndex().filter { it.value.first().lowercaseChar() in listOf('a', 'e', 'i', 'o', 'u') }
        (vowelStarts zip vowelStarts.reversed()).forEach { (a, b) ->
            words[a.index] = b.value
        }
        words.joinToString(" ")
    }.forEach(::println)
}