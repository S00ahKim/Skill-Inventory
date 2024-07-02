# Enum

- == 비교 가능
- 비교연산자 사용 불가, compareTo() 사용 가능
- 내부에 선언된 상수 하나하나가 타입의 객체
    * ex. Foo {BAR, CAR} 는 내부적으로 `static final Foo BAR = new Foo("BAR")`