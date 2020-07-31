fun main(){
    val (a, b) = readLine()!!.split(" ")
    val a_ = a.reversed().toInt()
    val b_ = b.reversed().toInt()
    
    if (a_ > b_) {println(a_)} else {println(b_)}
}