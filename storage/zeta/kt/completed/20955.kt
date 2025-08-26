fun find(parents: IntArray, x: Int): Int {
    var node = x
    while (parents[node] != node) {
        node = parents[node]
    }
    return node
}

fun union(parents: IntArray, a: Int, b: Int): Boolean {
    val ra = find(parents, a)
    val rb = find(parents, b)
    if (ra == rb) return false
    if (ra > rb) parents[rb] = ra else parents[ra] = rb
    return true
}

fun minimumOperations(n: Int, edges: List<Pair<Int, Int>>): Int {
    val parents = IntArray(n) { it }
    var operations = 0
    edges.forEach { (a, b) ->
        if (!union(parents, a, b)) {
            operations++
        }
    }

    val rooted_parents = parents.map { find(parents, it) }
    operations += rooted_parents.distinct().size - 1
    return operations
}


fun main() = with(System.`in`.bufferedReader()) {
    val (n, m) = this.readLine().split(" ").map { it.toInt() }
    val synapses: List<Pair<Int, Int>> = (1..m).map {
        this.readLine().split(" ").map { it.toInt() - 1 }.let { (a, b) -> a to b }
    }

    println(minimumOperations(n, synapses))
}