import java.util.Stack

class BfInstance(val program: String) {
    var ptr: UByte = 0u
    var tape: Array<UByte> = Array(32768, { 0u })

    data class Instruct(val type: Int, var jump: Int) {
        constructor(type: Int) : this(type, 0)
    }

    var instructions = mutableListOf<Instruct>()
    var stack = Stack<Int>()

    val bw = System.out.bufferedWriter()

    fun pre(): Boolean {
        var pc = 0
        var commentFlag = false
        for (c in program) {
            if (commentFlag && c != '\n') {
                continue
            }
            when (c) {
                '>' -> {
                    instructions.add(Instruct(0))
                    pc++
                }

                '<' -> {
                    instructions.add(Instruct(1))
                    pc++
                }

                '+' -> {
                    instructions.add(Instruct(2))
                    pc++
                }

                '-' -> {
                    instructions.add(Instruct(3))
                    pc++
                }

                '.' -> {
                    instructions.add(Instruct(4))
                    pc++
                }

                '[' -> {
                    instructions.add(Instruct(5))
                    stack.add(pc)
                    pc++
                }

                ']' -> {
                    instructions.add(Instruct(6))

                    if (stack.isNotEmpty()) {
                        val jump: Int = stack.pop()
                        instructions[pc].jump = jump
                        instructions[jump].jump = pc
                    } else {
                        return false
                    }

                    pc++
                }

                '%' -> {
                    commentFlag = true
                }

                '\n' -> {
                    commentFlag = false
                }
            }
        }
        if (stack.isNotEmpty()) {
            return false
        }
        return true
    }

    fun execute() {
        var curr = 0
        while (curr < instructions.size) {
            val inst = instructions[curr]
            when (inst.type) {
                0 -> {
                    ptr++
                    curr++
                }

                1 -> {
                    ptr--
                    curr++
                }

                2 -> {
                    tape[ptr.toInt()]++
                    curr++
                }

                3 -> {
                    tape[ptr.toInt()]--
                    curr++
                }

                4 -> {
                    bw.write(tape[ptr.toInt()].toInt().toChar().toString())
                    curr++
                }

                5 -> {
                    if (tape[ptr.toInt()].toUInt() == 0u) {
                        curr = inst.jump
                    } else {
                        curr++
                    }
                }

                6 -> {
                    if (tape[ptr.toInt()].toUInt() != 0u) {
                        curr = inst.jump
                    } else {
                        curr++
                    }
                }
            }
        }
        bw.flush()
    }


}

fun main() = with(System.`in`.bufferedReader()) {
    val t = this.readLine().toInt()
    val bw = System.out.bufferedWriter()
    (1..t).forEach {
        bw.write("PROGRAM #${it}:\n")
        bw.flush()
        val sb = StringBuilder()
        var line = this.readLine()
        while (line != "end") {
            sb.append(line)
            sb.append('\n')
            line = this.readLine()
        }
        val bf = BfInstance(sb.toString())
        if (!bf.pre()) {
            bw.write("COMPILE ERROR\n")
            bw.flush()
        } else {
            bf.execute()
            bw.write("\n")
            bw.flush()
        }


    }
}