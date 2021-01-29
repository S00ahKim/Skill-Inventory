# 기본 타입
- 모든 것은 object (어떤 변수에서든 멤버 함수와 프로퍼티를 호출할 수 있음)
- number, character, boolean: 런타임에서 primitive values으로 표현되나, 일반 클래스처럼 사용자에게 노출됨

# 숫자 number
- 평범한 숫자들은 Int 타입으로 추론
- Int 범위가 넘으면 Long으로 추론
- 소수로 초기화 된 변수는 Double 타입으로 추론
- 명시적으로 Float 으로 지정하고 싶다면 f 또는 F 붙여주기
- 숫자에 대한 자동 형변환 X

### 리터럴 상수
- 리터럴 상수? 변수에 할당할 상수값을 정의하는 것
- 10진법: 123 / 123L / 123.5 / 123.5f
- 16진법: 0x0F
- 2진법: 0b00001011 (자바의 경우 미지원)
- 8진법: 지원 안함, 자바의 경우 지원

### 숫자 표기 시 언더바 (가독성)
```kotlin
val a = 1_000_000
val b = 1234_5678_9012_3456L
val c = 999_99_9999L
val d = 0xFF_EC_DE_5E
val e = 0b11010010_01101001_10010100_10010010
// 자바도 지원
```

### 표현
- 기본적인 산술연산 (+ - * / %)을 제공
- 비교: `=`은 리터럴, `===`는 레퍼런스를 비교함
```kotlin
val a: Int = 100                  // 타입에 따라 할당해주는 크기에 한계 있음 (128까지는 true로 인식)
val boxedA: Int? = a
val anotherBoxedA: Int? = a
    
val b: Int = 10000
val boxedB: Int? = b
val anotherBoxedB: Int? = b
    
println(boxedA === anotherBoxedA) // true
println(boxedB === anotherBoxedB) // false
```

### 명시적 (형)변환
```kotlin
// 타입에 대해 자동으로 형변환 불가
val a: Int? = 1 
val b: Long? = a 
print(b == a) // Error!
```

### 연산

### 부동 소수점 비교


# 문자 character


# 부울 boolean


# 배열 array


# 부호 없는 정수 Unsigned Integers
- UByte, UShort, UInt, ULong
- UByteArray, UShortArray, UIntArray, ULongArray
```kotlin
val b: UByte = 1u
val s: UShort = 1u
val l: ULong = 1u
val a = 1UL
```
- 부호없는 타입은 Kotlin 1.3 이상 버전에서 가능 (현재는 베타)


# 문자열 strings
- 문자열의 문자 접근 : `[]​`
- 문자열 끼리의 연결 : `+`
- raw string : `"""`
- string template : `$`, `${expression}`