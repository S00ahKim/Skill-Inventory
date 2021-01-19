# Introduction
## Kotlin이란?
- 정의: Statically typed programming language for modern multiplatform applications
- 정적 타입? 컴파일 시점에 객체의 타입이 결정되기 때문에 실행 시점에 문제가 발생할 확률이 동적 타입 언어보다 적음

## Why Kotlin?
1. 문법의 간결함
    - 직관적, 간결함, 적은 중복
2. Null Safety
    - 자바: 런타임에 NPE
    - 코틀린: 컴파일 시점에 null 관련 문제를 처리하게 함
        * 변수나 파라미터에 null을 할당하려면 해당 변수나 파라미터가 null일 수 있다는 것을 명시적으로 표현
        * ex. `null일수도있는변수명?`
3. 기존의 자바 코드와 호환 가능
    - Java <-> Kotlin 쌍방 호출 가능
        * 코틀린 역시 JVM 기반
        * 자바 코드 -(`javac`)-> 컴파일된 바이트 코드 (class) -(`JVM`)-> 기계어
        * 코틀린 코드 -(`kotlinc`)-> 컴파일된 바이트 코드 (class) -(`JVM`)-> 기계어
    - 기존 자바 라이브러리 활용 가능
    - (완벽하지는 않으나) 자바 코드로 변환 가능
    - etc: 자바스크립트 컴파일 지원
4. 강력한 기능
    - 확장 함수
    - 연산자 오버로딩: 클래스에 사용할 수 없는 기본 연산자(+, -, *, /, ++, -- 등)를 클래스에 사용할 수 있게 연산자를 정의
    - 늦은(레이지) 필드 초기화 (객체 필드를 항상 초기화하고, 바로 사용하지 않음)
    - DSL (Domain-Specific Language)
    - 코루틴 (비동기, Actor)
    - Delegate
    - Reified
    - Smart Cast
5. 툴 친화적
    - 개발사: JetBrains
    - 지원 IDE: IntelliJ, Eclipse, ...
6. 안드로이드만...?
    - 구글이 안드로이드 공식 언어로 채택
    - 카카오톡 채팅 서버: 코드량 감소, 생산성 향상


# IntelliJ에서 코틀린 사용하기
(시연 필요)

## 새로운 프로젝트

## 기존 프로젝트

### 자바-코틀린 코드 링크

### 자바 코드 → 코틀린 코드


# Reference
- [자바 개발자 관점의 - 왜 코틀린인가? by 강현식](https://www.youtube.com/watch?v=HhifPEExguA&ab_channel=JetBrainsTV)
- [Google I/O 2017 참관기 - Kotlin](https://d2.naver.com/helloworld/7543578)
- [노마드 코더 - 코틀린이 자바를 대체할 수 있을까?](https://www.youtube.com/watch?v=8gseVzeMOzk&ab_channel=%EB%85%B8%EB%A7%88%EB%93%9C%EC%BD%94%EB%8D%94NomadCoders)