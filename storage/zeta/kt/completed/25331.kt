import java.io.StreamTokenizer
import kotlin.math.min

const val SIDE = 7
const val SIZE = SIDE * SIDE

class Drop7Simulator(val initState: IntArray) {
    var state: IntArray = initState.clone()

    fun init() {
        state = initState.clone()
    }

    fun getPos(row: Int, col: Int): Int = row * SIDE + col
    fun getRC(pos: Int) = Pair(pos / SIDE, pos % SIDE)

    fun getAt(pos: Int) = state[pos]
    fun getAt(row: Int, col: Int): Int = state[row * SIDE + col]
    fun setAt(row: Int, col: Int, value: Int) {
        state[row * SIDE + col] = value
    }

    fun swapAt(r1: Int, c1: Int, r2: Int, c2: Int) {
        val v1 = getAt(r1, c1)
        setAt(r1, c1, getAt(r2, c2))
        setAt(r2, c2, v1)
    }

    fun countBalls(): Int {
        return state.count {
            it != 0
        }
    }

    fun print() {
        for (i in 0 until SIDE) {
            for (j in 0 until SIDE) {
                print(getAt(i, j))
                print(" ")
            }
            println()
        }
    }

    fun start(ball: Int): Int {
        var minimum = Integer.MAX_VALUE
        for (j in 0 until SIDE) {
            if (getAt(0, j) != 0) {
                continue
            }
            init()
            var falling = SIDE - 1
            for (i in 1 until SIDE) {
                if (getAt(i, j) != 0) {
                    falling = i - 1
                    break
                }
            }
            setAt(falling, j, ball)

            while (update());
            minimum = min(minimum, countBalls())
        }

        return minimum
    }

    fun update(): Boolean {
        // 삭제 작업
        val candidate = mutableListOf<Int>() // List<Pos>
        val groups = mutableListOf<Pair<Int, Int>>() //
        for (i in 0 until SIDE) {
            for (j in 0 until SIDE) {
                val pos = getPos(i, j)
                val ele = getAt(pos)

                if (ele == 0) {
                    groups.filter {
                        it.first == groups.size
                    }.forEach {
                        candidate.add(it.second)
                    }

                    groups.clear()
                } else {
                    groups.add(Pair(ele, pos))
                }
            }
            groups.filter {
                it.first == groups.size
            }.forEach {
                candidate.add(it.second)
            }
            groups.clear()

        }

        for (j in 0 until SIDE) {
            for (i in 0 until SIDE) {
                val pos = getPos(i, j)
                val ele = getAt(pos)

                if (ele == 0) {
                    groups.filter {
                        it.first == groups.size
                    }.forEach {
                        candidate.add(it.second)
                    }

                    groups.clear()
                } else {
                    groups.add(Pair(ele, pos))
                }
            }

            groups.filter {
                it.first == groups.size
            }.forEach {
                candidate.add(it.second)
            }
            groups.clear()
        }


        if (candidate.isEmpty()) {
            return false
        }

        candidate.forEach {
            state[it] = 0
        }

        // 중력 업데이트

        for (j in 0 until SIDE) {
            for (i in (0 until SIDE - 1).reversed()) {
                var now = i
                if (getAt(now, j) == 0) {
                    continue
                }
                while (now < SIDE - 1 && getAt(now + 1, j) == 0) {
                    swapAt(now, j, now + 1, j)
                    now++
                }
            }
        }
        return true
    }
}


fun main() = with(StreamTokenizer(System.`in`.bufferedReader())) {
    val state = IntArray(SIZE)

    for (i in 0 until SIZE) {
        nextToken()
        state[i] = nval.toInt()
    }

    val simulator = Drop7Simulator(state)

    nextToken()
    val ball = nval.toInt()

    println(simulator.start(ball))
}