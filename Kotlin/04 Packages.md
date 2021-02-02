# Packages
```kotlin
package org.example

fun printMessage() { /*...*/ }
class Message { /*...*/ }
```
- 코드 최상단에 기재 (선언하지 않으면 기본 패키지에 포함)
- 코드 내 모든 콘텐츠(class와 함수)가 선언된 패키지에 포함됨 ex. `org.example.printMessage`


## Default Imports
```kotlin
// 기본적으로 추가되어 있는 디폴트 패키지
kotlin.*
kotlin.annotation.*
kotlin.collections.*
kotlin.comparisons.* (since 1.1)
kotlin.io.*
kotlin.ranges.*
kotlin.sequences.*
kotlin.text.*

// JVM
java.lang.*
kotlin.jvm.*

// JS
kotlin.js.*
```


## Imports
```kotlin
import org.example.Message // Message is now accessible without qualification
import org.example.* // everything in 'org.example' becomes accessible
import org.example.Message // Message is accessible
import org.test.Message as testMessage // testMessage stands for 'org.test.Message'
```