# Basic Syntax

## 패키지 정의 및 import
```kotlin
package my.demo
import kotlin.text.*
```
- 자바: 패키지가 파일 시스템의 디렉터리 구조와 동일
- 코틀린: 디렉토리와 패키지가 항상 일치하지 않아도 됨


## 프로그램 entry point
```kotlin
fun main() {
    println("Hello world!")
}
```
- Kotlin 애플리케이션의 시작점은 main 함수


## 함수 정의
```kotlin
fun 함수이름(인자이름: 인자타입, ...): 리턴타입 {
	...
}
```
1. 리턴 값이 없는 경우(void처럼) Unit으로 리턴 타입 기재
   * void와 Unit이 완전히 같은 것은 아니다. 자바와 함께 사용시 Unit 타입 사용 필요.
2. 리턴 타입이 Unit일 경우, 생략할 수 있음
3. 리턴 타입을 추론할 수 있는 경우, 생략할 수 있음

### idioms
1. 함수 파라미터에 기본값 설정 가능
```kotlin
fun foo(a: Int = 0, b: String = "") { ... }
```
2. 확장 함수 (Extension Functions)
```kotlin
fun String.spaceToCamelCase() { ... }
"Convert this to camelcase".spaceToCamelCase()
```
3. 표현식이 하나일 때, 중괄호 생략 가능
```kotlin
fun sum(a:Int, b:Int) : Int = a + b
```
4. lambda expression
- fun 키워드와 함수 이름을 생략함
- 문법은 `{ 파라미터 -> 내용 }` (파라미터 타입 선언 필요, 추론 가능할 경우 생략)
- 리턴 값은 내용의 마지막 expression
- 파라미터가 하나일 경우, it으로 표현 가능


## 변수
1. val (변경 불가능(read-only, immutable), = 자바의 final 키워드)
```kotlin
val a: Int = 1  // immediate assignment (선언부에서 초기화됨)
val c: Int      // 초기값이 없을 경우 타입을 명시해야 함
c = 3           // deferred assignment (선언부가 아니라 나중에 초기화됨)
```
2. var (변경 가능(mutable))
```kotlin
var x: Int = 5
x += 1
```


## 주석(comment) 처리
```kotlin
// 이렇게 주석을 쓸 수 있고
/*
    이렇게도 쓸 수 있으며
    /* 주석 중간에 또 다른 주석을 끼워넣을 수도 있다. */
*/
```


## 문자열 템플릿
```kotlin
val a = 2
val s1 = "a is $a" //a is 2
val s2 = "${s1.replace("is", "was")}, but now is $a" //a was 2, but now is 2
```
- `$a`와 같은 표현을 String Interpolation 이라고 부름.


## 조건문 (if)
- `statement`
    * 서술
    * 실행 가능(executable)한 최소한의 독립적인 코드 조각
    * 모든 expression은 statement
```kotlin
fun maxOf(a: Int, b: Int): Int {
    if (a > b) {
        return a
    } else {
        return b
    }
}
```
- `expression`
    * 수식
    * 하나 이상의 값으로 표현(reduce)될 수 있는 코드
    * 평가(evaluate)가 가능 -> 하나의 ‘값’으로 환원
```kotlin
// if가 expression이기 때문에 삼항 연산자를 지원하지 않는다.
fun maxOf(a: Int, b: Int) = if (a > b) a else b
```


## null Check
```kotlin
// 연산자로 null 여부를 확인할 수 있음
if (x == null) {
    println("x is null")
}

// null일 수 있다면 ? 표시 필요
fun parseInt(str: String): Int? {
    // ...
}
```

### idioms
1. if not null
```kotlin
val files = File("Test").listFiles()
println(files?.size)
```
2. if not null & else
```kotlin
val files = File("Test").listFiles()
println(files?.size ?: "empty")
```
3. if null 실행(execute)
```kotlin
val values = ...
val email = values["email"] ?: throw IllegalStateException("Email is missing!")
```
4. if not null 실행(execute)
```kotlin
val value = ...
value?.let {
    ... // null이 아닐 경우 이 블록 실행
}
```
5. null이 가능한 map value에 대한 if not null
```kotlin
val value = ...
val mapped = value?.let { transformValue(it) } ?: defaultValue 
// value가 null이거나 transformValue(it)이 null일 경우 defaultValue가 리턴됨
```


## type Check
```kotlin
println("example string" is String)
```


## Smart Casting
### 정확한 타입을 명시하지 않아도 자동으로 타입을 유추함
```kotlin
val b = 2                           // Int 타입으로 유추
val tickets = Money(100, "$")       // Money 타입으로 유추
fun sum(a:Int, b:Int) = a + b       // 리턴 타입이 Int일 것으로 유추
```

### null 여부를 확인한 후에는 자동으로 non-nullable 타입으로 캐스팅된다.
```kotlin
fun printProduct(arg1: String, arg2: String) {
    val x = parseInt(arg1)
    val y = parseInt(arg2)

    // 여기에서 바로 `x * y` 를 호출하면 null값일 수 있기 때문에 에러 발생
    if (x != null && y != null) {
        // x 와 y 는 이 블록 안에서 non-nullable로 간주함.
        println(x * y)
    }
    else {
        println("'$arg1' or '$arg2' is not a number")
    }    
}

fun printProduct2(arg1: String, arg2: String) {
    val x = parseInt(arg1)
    val y = parseInt(arg2)

    if (x == null) {
        println("Wrong number format in arg1: '$arg1'")
        return
    }
    if (y == null) {
        println("Wrong number format in arg2: '$arg2'")
        return
    }

    // 여기까지 도달한 x와 y 역시 분기문에 걸리지 않았으므로 자동으로 non-nullable 으로 간주함.
    println(x * y)
}

// idiom: 만약 Boolean이 nullable하다면?
val b: Boolean? = ...
if (b == true) {
    ...
} else {
    // `b`는 false 또는 null
}
```

### 어떤 type인지 여부를 확인한 후에는 자동으로 해당 타입으로 여겨진다.
```kotlin
fun getStringLength(obj: Any): Int? {
    if (obj is String) {
        // `obj` 는 이 블록 안에서 자동으로 `String` 으로 간주함.
        return obj.length
    }

    // 하지만 타입 체크된 블록이 아닌, 여기에서는 `obj` 는 여전히 `Any`
    return null
}

fun getStringLength(obj: Any): Int? {
    if (obj !is String) return null

    // 위에서 문자열이 아닌 경우를 제외했기 때문에 `obj` 는 자동으로 `String` 으로 간주함.
    return obj.length
}

fun getStringLength(obj: Any): Int? {
    // && 표시는 앞에서부터 적용된다. 앞의 조건에서 이미 `String`인지 체크했기 때문에 `obj`는 자동으로 `String` 으로 간주함.
    if (obj is String && obj.length > 0) {
        return obj.length
    }

    return null
}
```


## for
```kotlin
val items = listOf("apple", "banana", "kiwifruit")
for (item in items) {
    println(item)
}

val items = listOf("apple", "banana", "kiwifruit")
for (index in items.indices) {
    println("item at $index is ${items[index]}")
}
```
- iterator를 제공하는 어떤 것이든 순회(iterate)할 수 있음
- iterator? `next()`와 Boolean을 리턴하는 `hasNext()`를 리턴 타입으로 갖는 함수 `iterator()`


## while
```kotlin
val items = listOf("apple", "banana", "kiwifruit")
var index = 0
while (index < items.size) {
    println("item at $index is ${items[index]}")
    index++
}
```


## when
```kotlin
fun describe(obj: Any): String =
    when (obj) {
        1          -> "One"
        "Hello"    -> "Greeting"
        is Long    -> "Long"
        !is String -> "Not a string"
        else       -> "Unknown"
    }
    // 자바 switch와 유사

fun main() {
    println(describe(1))        // One
    println(describe("Hello"))  // Greeting
    println(describe(1000L))    // Long
    println(describe(2))        // Not a string
    println(describe("other"))  // Unknown
}
```

### idioms
1. when 으로 리턴하기
```kotlin
fun transform(color: String): Int {
    return when (color) {
        "Red" -> 0
        "Green" -> 1
        "Blue" -> 2
        else -> throw IllegalArgumentException("허용되지 않은 색깔입니다.")
    }
}
```
2. 인스턴스인지 체크하기
```kotlin
when (x) {
    is Foo -> ...
    is Bar -> ...
    else   -> ...
}
```


## range
```kotlin
// 특정 범위 안에 포함되는가?
val x = 10
val y = 9
if (x in 1..y+1) { 
    println("x가 범위 안에 포함됩니다.")
}

// 특정 범위 안에 포함되지 않는가?
if (x !in 11..20) {
    println("x가 범위 안에 포함되지 않습니다.")
}

// 범위를 간격을 두어 표현하기
for (x in 1..10 step 2) {
    print(x) //13579
}
for (x in 9 downTo 0 step 3) {
    print(x) //9630
}

// idiom
for (i in 1..100) { ... } // 1부터 100까지 순회
for (i in 1 until 100) { ... } // 1부터 99까지 순회 
```


## Collection
- 지원하는 자료구조: List, Map, Set
    * immutable: 자료구조of
    * mutable: mutable자료구조of
```kotlin
val fruits = listOf("avocado", "banana", "apple")
val veggies = setOf("tomato", "carrot", "lettuce")
val fruitsCode = mutableMapOf<String, String>(
    "avocado" to "AVO", "banana" to "BAN", "apple" to "APP")
```
- 지원하는 기능은 아주 많으며, 그룹으로 묶어보면 아래와 같음
    1. Creation — 컬렉션을 생성하는 함수 (ex : listOf)
    2. Convert — 다른 유형으로 캐스팅하는 함수 (ex: asMap)
    3. Change — 내용을 변환하는 함수 (ex: map)
    4. Choose — 항목 중 하나에 접근하는 함수(ex : get)
    5. Conclude — 항목에서 무언가를 생성하는 함수(ex: sum)

### 값에 접근하기 (... etc.)
```kotlin
println(fruits.get(0))          // avocado
println(veggies.elementAt(0))   // tomato
println(fruitsCode["avocado"])  // AVO
fruitsCode["avocado"] = "AVC"
```

### 콜렉션 안에 어떤 오브젝트가 포함되어 있는지 확인하기 -> in
```kotlin
when {
    "orange" in fruits -> println("냠냠")
    "apple" in fruits -> println("사과 냠냠")
}
// 사과 냠냠
```

### 콜렉션 filter와 map에서 lambda expression 사용 가능
```kotlin
/* 
    !!주의!!
    it은 임시로 붙인 변수 이름이 아니라 
    lambda expression에 파라미터가 하나일 때 사용하는 "키워드"
    (= 다른 이름을 쓰면 에러 발생)
*/
fruits
  .filter { it.startsWith("a") }
  .sortedBy { it }
  .map { it.toUpperCase() }
  .forEach { println(it) }
/*
APPLE
AVOCADO
*/
```

### idioms
1. 리스트 필터링
```kotlin
val positives = list.filter { x -> x > 0 }
val positives = list.filter { it > 0 }
```
- [it-implicit-name-of-a-single-parameter](https://kotlinlang.org/docs/reference/lambdas.html#it-implicit-name-of-a-single-parameter)
2. 특정 엘리먼트가 존재하는지 확인
```kotlin
if ("john@example.com" in emailsList) { ... }   // in
if ("jane@example.com" !in emailsList) { ... }  // not in
```
3. map/list pair traversing(순회)
```kotlin
// 자바의 방법들보다 간결함
for ((k, v) in map) {
    println("$k -> $v")
}
```
```java
// 방법1
Iterator<String> keys = map.keySet().iterator();
while( keys.hasNext() ){
    String key = keys.next();
    System.out.println( String.format("키 : %s, 값 : %s", key, map.get(key)) );
}
 
// 방법2
for( Map.Entry<String, String> elem : map.entrySet() ){
    System.out.println( String.format("키 : %s, 값 : %s", elem.getKey(), elem.getValue()) );
}
 
// 방법3
for( String key : map.keySet() ){
    System.out.println( String.format("키 : %s, 값 : %s", key, map.get(key)) );
}
```


## Class, Instance
```kotlin
abstract class Shape(val sides: List<Double>) {
    val perimeter: Double get() = sides.sum()
    abstract fun calculateArea(): Double
}

interface RectangleProperties {
    val isSquare: Boolean
}

// 추상 클래스 initialize
class Rectangle(
    var height: Double,
    var length: Double
) : Shape(listOf(height, length, height, length)), RectangleProperties {
    override val isSquare: Boolean get() = length == height
    override fun calculateArea(): Double = height * length
}

class Triangle(
    var sideA: Double,
    var sideB: Double,
    var sideC: Double
) : Shape(listOf(sideA, sideB, sideC)) {
    override fun calculateArea(): Double {
        val s = perimeter / 2
        return Math.sqrt(s * (s - sideA) * (s - sideB) * (s - sideC))
    }
}

fun main() {
    val rectangle = Rectangle(5.0, 2.0)     // new 불필요
    val triangle = Triangle(3.0, 4.0, 5.0)

    println("Area of rectangle is ${rectangle.calculateArea()}, its perimeter is ${rectangle.perimeter}")
    println("Area of triangle is ${triangle.calculateArea()}, its perimeter is ${triangle.perimeter}")
}

/*
Area of rectangle is 10.0, its perimeter is 14.0
Area of triangle is 6.0, its perimeter is 12.0
*/
```


# Reference & Learn More
- [코드 단위인 Expression과 Statement의 차이를 알아보자](https://shoark7.github.io/programming/knowledge/expression-vs-statement)
- [Kotlin Collection Functions Cheat Sheet](https://medium.com/mobile-app-development-publication/kotlin-collection-functions-cheat-sheet-975371a96c4b)
- [Kotlin의 Collection 함수 (번역)](https://medium.com/hongbeomi-dev/kotlin-collection-%ED%95%A8%EC%88%98-7a4b1290bce4)