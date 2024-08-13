# Spring Event
> Spring의 Bean과 Bean 사이의 데이터를 전달하는 방법중 하나
* 대개는 DI를 통해 이루어짐
* 이벤트는 빈이 이벤트를 ApplicationContext로 넘겨주고 이를 Listener에서 받아서 처리하는 식


## 구성
### 1. Event 모델
> 이벤트의 데이터를 담음
- `ApplicationEvent`를 상속받음
- `super`로 생성자에서 객체를 전달

### 2. Event Publisher
> 이벤트를 발생시킴
- 이벤트를 생성하는 빈은 `ApplicationEventPublisher`. 이를 주입받는다.
- `publishEvent` 메서드로 이벤트를 생성한다.

### 3. Event Listener
> 이벤트를 소비함 (처리 방법 정의)
- `ApplicationListener<{Event_Name}>`을 상속받음
    * 다중 상속이 불가능하기 때문에 제약이 있다면, `@EventListener` 어노테이션 붙이기. 그러면 Event도 상속으로 만들지 않아도 됨.
- `onApplicationEvent`를 오버라이딩하여 이벤트를 소비한다.
- cf. `@TransactionalEventListener`
    * 일반적인 이벤트 리스너는 퍼블리시 코드가 도는 시점에 바로 퍼블리시된다.
    * `@Transactional`로 하나의 트랜잭션이 된 메서드의 경우, 다른 건 롤백이 되더라도 이벤트 발생은 롤백이 안되는 경우가 있다. @TransactionEventListener는 Event의 실질적인 발생을 트랜잭션의 종료를 기준으로 삼을 수 있다. (명확한 실행 시점은 옵션으로 조정 가능)
    * 참고로 @TransactionalEventListener는 트랜잭션에 의존하여 발생하기 때문에 @Trasnactional이 없을때는 Event가 발생하지 않는다.


## 특징
- 퍼블리셔와 리스너는 기본적으로는 같은 쓰레드 위에서 돈다. 비동기로 돌리려면 ApplicationEventMulticaster 등을 사용하면 됨.
- 리스너는 여러개 등록할 수 있음.


## ref.
- https://www.blog.ecsimsw.com/entry/ApplicationEventPublisher-%EC%9D%B4%EB%B2%A4%ED%8A%B8%EB%A5%BC-%EB%8B%A4%EB%A3%A8%EB%8A%94-%EB%B0%A9%EB%B2%95
- https://sabarada.tistory.com/184
- https://sabarada.tistory.com/188