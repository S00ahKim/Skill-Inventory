# Basic Syntax

## 패키지 정의 및 import
```kotlin
package my.demo
import kotlin.text.*
```
- 디렉토리와 패키지가 항상 일치하지 않아도 됨

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
1. 리턴 값이 없는 경우(ex. Java의 void) Unit으로 리턴 타입 기재
2. 리턴 타입이 Unit일 경우, 생략할 수 있음

## 변수
1. val
```kotlin
val a: Int = 1  // immediate assignment (선언부에서 초기화됨)
val b = 2   // `Int` type is inferred
val c: Int  // Type required when no initializer is provided <?> lazy init?
c = 3       // deferred assignment (선언부가 아니라 나중에 초기화됨)
```
2. var
```kotlin
var x = 5 // `Int` type is inferred
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

## 조건문

## null Check

## type Check

## Smart Casting

## for

## while

## when

## range

## Collection

## Class, Instance