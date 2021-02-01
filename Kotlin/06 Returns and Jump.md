# 반환과 점프
- 코틀린의 점프 표현 (Nothing 타입)
  * `return` 인접한 함수 또는 anonymous function로 부터 가장 기본적인 반환.
  * `break` 인접한 루프를 종료.
  * `continue` 인접한 루프의 다음 스텝 진행.


## Break and Continue Labels
```kotlin
loop@ for (i in 1..100) {
    for (j in 1..100) {
        if (...) break@loop // j의 for가 아니라 i의 for문을 중지
    }
}
```


## Return at Labels
```kotlin
fun foo1() {
    listOf(1, 2, 3, 4, 5).forEach {
        if (it == 3) return // 외부 함수에 리턴
        print(it)
    }
    println("여기는 닿지 못하는 곳")
}

fun foo2() {
    listOf(1, 2, 3, 4, 5).forEach lit@{
        if (it == 3) return@lit // 람다에서 리턴하려면 라벨 필요 (lit에 리턴)
        print(it)
    }
    print("명시적으로 라벨링해서 끝냄")
}
```