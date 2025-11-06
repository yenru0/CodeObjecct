import java.io.BufferedReader

enum class TokenType {
    WORD_CAPITALIZED, WORD_ELSE, COMMA, DOT, SPACE, LINEBREAK,

    ERR,
}

data class Token(val type: TokenType, val word: String?) {
    constructor(type: TokenType) : this(type, null)
}

class Tokenizer(val br: BufferedReader) {
    private var queue = ArrayDeque<Char>(16)
    private var unget = 0
    private fun getChar(): Char? {
        if (unget != 0) {
            val c = queue[unget - 1]
            unget--
            return c
        }
        val read = br.read()
        if (read == -1) {
            return null
        } else {
            val c = read.toChar()
            if (queue.size > 10) {
                queue.removeLast()
            }
            queue.addFirst(c)
            return c
        }
    }

    private fun ungetChar() {
        unget++
    }

    public fun nextToken(): Token {
        var c = getChar()
        if (c == null) {
            return Token(TokenType.ERR)
        }
        while (true) {
            when (c) {
                in 'A'..'Z' -> {
                    val sb = StringBuilder()
                    sb.append(c)
                    var flag = true
                    c = getChar()
                    while (true) {
                        when (c) {
                            in 'a'..'z' -> {
                                sb.append(c)
                            }

                            in 'A'..'Z' -> {
                                flag = false
                                sb.append(c)
                            }

                            else -> {
                                ungetChar()
                                break
                            }
                        }
                        c = getChar()
                    }

                    return Token(
                        if (flag && sb.length != 1) {
                            TokenType.WORD_CAPITALIZED
                        } else {
                            TokenType.WORD_ELSE
                        }, sb.toString()
                    )
                }

                in 'a'..'z' -> {
                    val sb = StringBuilder()
                    sb.append(c)
                    c = getChar()
                    while (true) {
                        when (c) {
                            in 'a'..'z' -> {
                                sb.append(c)
                            }

                            in 'A'..'Z' -> {
                                sb.append(c)
                            }

                            else -> {
                                ungetChar()
                                break
                            }
                        }
                        c = getChar()
                    }

                    return Token(
                        TokenType.WORD_ELSE, sb.toString()
                    )
                }

                '.' -> {
                    return Token(TokenType.DOT)
                }

                ',' -> {
                    return Token(TokenType.COMMA)
                }

                ' ' -> {
                    return Token(TokenType.SPACE)
                }

                '\n' -> {
                    return Token(TokenType.LINEBREAK)
                }

                else -> {
                    return Token(TokenType.ERR)
                }
            }
        }
    }
}

class Parser(val lexer: Tokenizer) {
    private val queue = ArrayDeque<Token>()
    private var toked: Token = lexer.nextToken()

    fun nextPhrase(): String? {
        if (toked.type == TokenType.ERR) {
            toked = lexer.nextToken()
            return null
        } else if (toked.type != TokenType.WORD_CAPITALIZED) {
            val s = when (toked.type) {
                TokenType.DOT -> "."
                TokenType.COMMA -> ","
                TokenType.LINEBREAK -> "\n"
                TokenType.WORD_ELSE -> toked.word!!
                TokenType.SPACE -> " "
                else -> "<Err>"
            }
            toked = lexer.nextToken()
            return s
        } else {
            val sb = StringBuilder()
            var succ = 1
            queue.addFirst(toked)
            toked = lexer.nextToken()
            var wantSpace = true
            while (toked.type != TokenType.COMMA || toked.type != TokenType.DOT || toked.type != TokenType.WORD_ELSE || toked.type != TokenType.LINEBREAK || toked.type != TokenType.ERR) {
                if (toked.type == TokenType.SPACE && wantSpace) {
                    wantSpace = false
                    queue.addFirst(toked)
                } else if (toked.type == TokenType.WORD_CAPITALIZED && !wantSpace) {
                    wantSpace = true
                    queue.addFirst(toked)
                    succ++
                } else {
                    break
                }
                toked = lexer.nextToken()
            }

            if (succ >= 2) {
                val sb2 = StringBuilder()
                val spaceflag = (queue.size % 2 == 0)
                if (spaceflag) {
                    queue.removeFirst()
                }

                queue.reversed().forEach {
                    if (it.type == TokenType.SPACE) {
                        sb2.append(' ')
                    } else {
                        sb.append(it.word!!.first())
                        sb2.append(it.word!!)
                    }
                }
                sb.append(' ')
                sb.append('(')
                sb.append(sb2)
                sb.append(')')
                if (spaceflag) {
                    sb.append(' ')
                }
                queue.clear()
            } else {
                queue.reversed().forEach {
                    sb.append(
                        if (it.type == TokenType.SPACE) {
                            " "
                        } else {
                            it.word!!
                        }
                    )
                }
                queue.clear()
            }
            return sb.toString()
        }
    }
}

fun main() {
    val lexer = Tokenizer(System.`in`.bufferedReader())
    val parser = Parser(lexer)
    var phrase = parser.nextPhrase()
    while (phrase != null) {
        print(phrase)
        phrase = parser.nextPhrase()
    }
}