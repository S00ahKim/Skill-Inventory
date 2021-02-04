# Control Flow
## If Expression
```kotlin
// Traditional usage 
var max = a 
if (a < b) max = b

// With else 
var max: Int
if (a > b) {
    max = a
    // 이렇게 블록을 가질 수 있고, 이 블록의 마지막 expression이 리턴값
} else { // statement가 아니라 expression으로 쓸 경우 else 필수
    max = b
}
 
/* As expression 
    - 어떤 값을 반환함
    - 삼항 연산자 불필요함
*/
val max = if (a > b) a else b
```


## When Expression
```kotlin
when (x) {
    1 -> print("x == 1")
    2 -> print("x == 2")
    else -> { // if 처럼 블록을 가질 수 있고, 역시 마지막 expression이 리턴값
        print("모든 조건을 만족하지 않는 경우 else로 들어온다")
    }
}
```
- like... C `switch`
- 조건 만족할 때까지 순차적 순회
- as statement: 개별 브랜치 무시
- as expression: 조건이 맞는 값이 전체 when 문의 리턴값


## For Loops
```kotlin
for (item: Int in ints) {
    // ...
}
```
- like... C# `foreach`
- 반복자를 제공하는 모든 대상을 반복 가능


## While Loops
```kotlin
while (x > 0) {
    x--
}

do {
    val y = retrieveData()
} while (y != null) // y is visible here!
```
- like... 다른 언어들의 `while`, `do-while`