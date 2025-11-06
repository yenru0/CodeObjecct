import java.util.Stack

enum class TokenType {
    ID, ANDREF, STAR, BRACK, COMMA, SEMI,

    ERR
}

data class Token(val type: TokenType, val name: String?) {
    constructor(type: TokenType) : this(type, null)
}

class Tokenizer(val line: String) {
    var index: Int = 0

    fun getChar(): Char? {
        if (line.length == index) {
            return null
        }
        val t = line[index]
        index++
        return t
    }

    fun ungetChar() {
        index--;
    }

    fun nextToken(): Token? {
        val c = getChar() ?: return null
        val tok: Token
        when (c) {
            '[' -> {
                getChar()
                tok = Token(TokenType.BRACK, null)
            }

            '&' -> {
                tok = Token(TokenType.ANDREF)
            }

            '*' -> {
                tok = Token(TokenType.STAR)
            }

            ',' -> {
                tok = Token(TokenType.COMMA)
            }

            ' ' -> {
                tok = nextToken() ?: return null
            }

            ';' -> {
                tok = Token(TokenType.SEMI)
            }

            in 'a'..'z', in 'A'..'Z' -> {
                var s = c.toString()
                var cn = getChar()
                while (cn in 'a'..'z' || cn in 'A'..'Z') {
                    s += cn
                    cn = getChar()
                }
                ungetChar()
                tok = Token(TokenType.ID, s)
            }

            else -> {
                tok = Token(TokenType.ERR)
            }
        }
        return tok
    }
}

class Parser(val lexer: Tokenizer) {
    val symtab = mutableListOf<Pair<List<Token>, String>>()
    var default: String
    var default_modifiers: MutableList<Token> = mutableListOf()
    var tok: Token = lexer.nextToken()!! // it must be ID

    init {
        default = tok.name!!

        tok = lexer.nextToken()!!
        while (tok.type == TokenType.STAR || tok.type == TokenType.BRACK || tok.type == TokenType.ANDREF) {
            default_modifiers.add(tok)
            tok = lexer.nextToken()!!
        }
    }

    fun construct_symbols() {
        val stack = Stack<Token>()
        var id: String? = null
        while (tok.type != TokenType.SEMI) {
            if (tok.type == TokenType.ID) {
                if (id == null) {
                    id = tok.name
                }
            }

            if (tok.type == TokenType.STAR || tok.type == TokenType.BRACK || tok.type == TokenType.ANDREF) {
                stack.add(tok)
            }

            if (tok.type == TokenType.COMMA) {
                val mods = mutableListOf<Token>()
                default_modifiers.forEach {
                    mods.add(it)
                }
                while (stack.isNotEmpty()) {
                    mods.add(stack.pop())
                }

                symtab.add(mods to id!!)
                id = null
            }

            tok = lexer.nextToken()!!
        }

        val mods = mutableListOf<Token>()
        default_modifiers.forEach {
            mods.add(it)
        }
        while (stack.isNotEmpty()) {
            mods.add(stack.pop())
        }

        symtab.add(mods to id!!)
        id = null

    }

    fun print_symbols() {
        symtab.forEach {
            print(default)
            it.first.forEach {
                when (it.type) {
                    TokenType.STAR -> print('*')
                    TokenType.ANDREF -> print('&')
                    TokenType.BRACK -> print("[]")
                    else -> print("ERR")
                }
            }
            println(" ${it.second};")
        }
    }
}

fun main() {
    val lex = Tokenizer(readln())
    val parser = Parser(lex)
    parser.construct_symbols()

    parser.print_symbols()
}