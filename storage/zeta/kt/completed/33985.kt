/**
평문의 모든 문자는 X이므로 Encrypted Message가 모두 `A` 또는 `B`로 교체되려면,
처음과 마지막은 각각 `A`, `B`이어야만 한다.
나머지는 구역은 모두 A 또는 B로 바꿀수 있는 방법이 항상 존재한다.
```
// 가운데 문자를 i번째 문자(S[i])라고 할경우:
XXX -> ABX -> AAB (S[i]를 A로 바꾸는 경우)
XXX -> ABX -> AAB -> ABB (S[i]를 B로 바꾸는 경우)
```

 */
fun isValidEncryption(s: String): Boolean {
    return s.startsWith('A') && s.endsWith('B')
}


fun main() {
    readln()
    isValidEncryption(readln().trim()).run {
        println(
            if (this) {
                "Yes"
            } else {
                "No"
            }
        )
    }
}