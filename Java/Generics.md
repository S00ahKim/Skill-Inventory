# 제네릭

- 다양한 타입의 객체들을 다루는 메서드나 컬렉션 클래스에 **컴파일 시**의 타입 체크를 해주는 기능
- 장점: 타입 안정성 제공, 타입체크&형변환 생략 가능

## 표현
- 클래스, 메서드 등에 적용 가능
    * cf. `class Box<T>`는 타입 변수 T를 갖는 원시 타입 Box의 제네릭 클래스이며, 줄여서 `T의 Box` `T Box` 라고 함
- `<? extends T>` 와일드 카드의 상한 제한 (T와 자손만 가능)
- `<? super T>` 와일드 카드의 하한 제한 (T와 조상만 가능)
- `<?>` 제한 없음, 모든 타입 가능 (=`<? extends Object>`)

## 형변환
- 제네릭-넌제네릭 타입 간 형변환은 경고가 발생하지만 가능함
- 제네릭-제네릭 타입 간 형변환은 불가능 (단, 와일드카드 사용하는 경우는 가능)