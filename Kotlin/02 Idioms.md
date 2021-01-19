# and other Idioms
자주 사용되는 관용구 모음, 추가하고 싶을 경우 [Kotlin Repo](https://github.com/JetBrains/kotlin)에 PR 남기기

## DTO 만들기
```kotlin
data class Customer(val name: String, val email: String)
```
- data class? 특별히 별도의 함수가 필요하지 않고 데이터만 갖고 있는 클래스
- 컴파일러가 자동으로 각종 기본 메서드 생성
    * equals()
    * hashCode()
    * toString()
    * copy()
    * 모든 프로퍼티에 대한 getters (setters의 경우 var)


## expression을 변수에 할당할 수 있다
### 1. if-else
```kotlin
fun foo(param: Int) {
    val result = if (param == 1) {
        "one"
    } else if (param == 2) {
        "two"
    } else {
        "three"
    }
}
```

### 2. try-catch
```kotlin
fun test() {
    val result = try {
        count()
    } catch (e: ArithmeticException) {
        throw IllegalStateException(e)
    }
    // Working with result
}
```

## 패턴의 적용
### 1. Singleton

### 2. Builder
```kotlin
// Unit을 리턴하는 빌더-스타일 메서드
fun arrayOfMinusOnes(size: Int): IntArray {
    return IntArray(size).apply { fill(-1) }
}
val a = arrayOfMinusOnes(5)
println(a.asList()) // [-1, -1, -1, -1, -1]
```


## 유용한 확장 함수
- 확장 함수? 이미 존재하는 클래스에 함수 덧붙이기

### 1. let

### 2. with

### 3. run

### 4. apply
<?> This is useful for configuring properties that aren't present in the object constructor.

### 5. also
```kotlin
// 변수 스와핑
var a = 1
var b = 2
a = b.also { b = a }
```


## Generic함수에서 타입 정보 읽기
```kotlin
inline fun <reified T> function(argument: T)
```
- `inline`? 함수 안에서 함수를 호출할 때, 호출된 함수 내부의 코드를 호출한 함수 안으로 넣어주는 것
- `reified`? inline 함수에서 런타임에 타입 정보를 알고 싶을 때 활용하는 키워드


## 초기화 지연
### 1. Late initialization

### 2. Lazy initialization


## TODO로 미완성 코드 표현하기
```kotlin
fun calcTaxes(): BigDecimal = TODO("개발중: 회계팀에게 피드백 대기")
```
- 위 함수는 언제나 `NotImplementedError`를 던진다.
- 리턴 타입은 `Nothing` (함수의 리턴값으로 정의된 타입이 무엇이든 `TODO()` 사용 가능)
- 예제와 같이 이유를 설명한 문자열을 허용하는 오버로딩이 있음
- 인텔리제이에서 TODO toolwindow에 이 부분을 자동으로 추가함