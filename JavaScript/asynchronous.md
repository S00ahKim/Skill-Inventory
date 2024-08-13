# 자바스크립트에서 비동기적 처리하기

## 동기(Synchronous), 비동기(Asynchronous), Promise
1. 동기: 요청을 보낸 뒤 요청의 응답을 받고 다음 작업 실행
2. 비동기: 요청을 보낸 뒤 응답과 관계 없이 다음 작업 실행
3. Promise: 비동기 작업이 맞이할 미래의 완료 또는 실패와 그 결과 값을 나타내는 객체


## 자바스크립트의 특성
1. Single Thread
    * 자바스크립트는 하나의 메인 쓰레드와 하나의 콜스택을 가짐
2. Synchronous
    * 자바스크립트 런타임 자체적으로 비동기 API를 지원하는 것이 아님
    * 비동기 처리는 런타임 환경에서 담당 (언어 코어 단보다 상위 단에서 처리함)


## 자바스크립트 실행 환경
![async_2](../images/async_2.png)
- 런타임? 자바스크립트 엔진을 구동하는 환경. 브라우저 또는 Node.js 등. 스케줄링은 여기서 처리.
    * 구성
      1. 메모리 힙 (memory heap)
          * 메모리 할당을 담당
      2. 콜 스택 (call stack)
          * 하나의 메인스레드에서 호출되는 함수들이 쌓임
          * LIFO(Last In First Out)로 실행
- 자바스크립트 엔진? 임의의 코드에 대한 on-demand 실행 환경
- 이벤트 루프? 이벤트 발생 시 호출되는 콜백 함수들을 관리하여 콜백 큐에 전달 + 콜백 큐 안의 콜백 함수를 콜 스택에 전달
- 콜백 큐? web api에서 비동기 작업들이 실행된 후 호출되는 콜백 함수들이 기다리는 공간 (순서는 이벤트 루프가 지정, FIFO로 실행)
    * 콜백 큐는 Microtask Queue, Animation Frames 등 여러개의 큐로 이루어져 있음
    * 블로킹? 콜 스택이 멈춘 상태 (어떤 작업이 실행 완료되기를 기다리며 다른 작업을 수행할 수 없는 상태)
    * 논 블로킹? 비동기 작업들을 Web API에게 넘겨줌으로써, 해당 작업이 완료될때까지 다른 코드들을 실행할 수 있는 것
- Web API? 브라우저에서 자체 지원하는 api. Dom 이벤트, Ajax (XmlHttpRequest), setTimeout 등의 비동기 작업들을 수행할 수 있도록 api를 지원


## 시도 1. 런타임에서 비동기 처리하기
- 동작 순서
    1. 콜 스택에 쌓여있던 비동기 작업 코드 실행
    2. 자바스크립트 엔진이 Web API에게 비동기 작업 위임
    3. Web API가 이벤트 루프를 통해 콜스택에 쌓여 있는 작업이 없을 경우 실행
- 단점
    * Web API로 들어오는 순서가 아니라 어떤 이벤트가 먼저 처리되느냐를 우선으로 판단
    * 실행 순서 불명확 -> 비동기 흐름 컨트롤 어려움


## 시도 2. 콜백함수 (Callback Function)
- 콜백함수? 특정 함수에 매개변수로 전달된 함수. 
- 동작 순서
    1. 콜 스택에 쌓여있던 비동기 작업 코드 실행
    2. 자바스크립트 엔진이 Web API에게 비동기 작업 위임
    3. Web API가 비동기 작업을 수행한 뒤 응답을 받음
    4. Web API가 이벤트 루프를 통해 콜스택에 쌓여 있는 작업이 없을 경우 콜백 함수를 콜백 큐로 전달
    5. 콜스택에 쌓인 콜백 함수가 실행되고, 콜스택에서 제거됨
- 특징
    * 전달된 함수 안에서 호출 및 실행됨. 
    * 전달된 함수가 콜백함수의 제어권을 가짐.
    * 처리되어야 하는 이벤트를 순차적으로 콜백 함수에 넣어주는 방식으로 구현.
- 단점
    * 콜백함수의 종료 시점 예측 불가, 모든 종속 함수의 중첩 -> 콜백 지옥
    * 코드의 가독성이 떨어짐
    * 모든 콜백함수에서 각각 에러 핸들링 필요
    * 로직 변경의 어려움
- 개선 방안
    * 코딩 패턴 (각 콜백 함수 분리)
    * 또는... Promise


## 시도 3. 프로미스 (Promise)
- 프로미스? 비동기 작업이 갖게 될 미래의 완료/실패 + 결과값
- 동작 순서
    1. `new Promise()` 메소드 호출 시 바로 `Pending`
    2. 콜백함수 인자 `resolve` 호출시 -> `Fulfilled`(Settled) -> `then()`으로 결과값 받음
    3. 콜백함수 인자 `reject` 호출시 -> `Rejected`(Settled) -> `catch()`로 실패 결과값 받음
- 사용예
    ```javascript
    new Promise() // new 키워드로 메소드 호출
    new Promise(function(resolve, reject) {...}) // 인자로 resolve(비동기 처리 성공), reject(비동기 처리 실패)를 받는 콜백 함수를 넘겨줌
    ```
- 특징
    * 결과값을 받기 때문에 응답(결과)에 대한 사후 처리를 쉽게 제어할 수 있음.
    * 한 블록 내에 많은 중첩 함수를 작성할 필요 없음.
    * 코드의 가독성이 좋아짐.
    * then은 연속적으로 사용할 수 있음. (Promise Chaining)
    * 프로미스의 처리 과정을 상태값으로 가짐
      * ![async_1](../images/async_1.png)
      * `Pending`: 대기, 비동기 처리 작업이 아직 완료되지 않음.
      * `Fulfilled`: 성공, 비동기 처리 작업이 완료되어 프로미스가 결과값을 리턴함.
      * `Settled`: 결과 값이 성공/실패로 반환된 상태. 이 상태가 된 값은 재실행되지 않음.
      * `Rejected`: 실패, 비동기 처리 작업이 실패하거나 오류가 발생함.
- 단점
    * 패턴을 잘못 사용할 경우, 체이닝 과정에서 에러 핸들링에 어려움을 겪을 수 있음
    * 여전히 코드가 장황함


## 시도 4. async-await
- async-await? 비동기 함수를 만들 수 있는 키워드. async로 함수를 선언하면 AsyncFunction 객체를 반환하는 함수를 정의하는 것.
- 특징
    * 비동기 함수는 암시적으로 프로미스를 사용하여 결과를 리턴함.
    * 프로미스를 대체하는 개념이 아니라, 콜백과 프로미스의 단점을 보완함.
    * 프로미스를 사용하지만, then, catch를 컨트롤하지 않고 동기적 코드처럼 리턴값을 변수에 할당 가능. -> 코드 작성을 동기적 관점에서 할 수 있음
    * 코드의 가독성이 훨씬 나음. (여러개의 비동기 처리를 다루는 등)
    * async가 붙은 함수 내에서만 await를 사용 가능.
    * 프로미스 객체를 리턴하는 함수에 사용 가능.
    * 에러 핸들링에 try-catch문 사용 가능.


# References
- [MDN Web Docs - Promise](https://developer.mozilla.org/ko/docs/Web/JavaScript/Reference/Global_Objects/Promise)
- [Javascript 동작원리 (Single thread, Event loop, Asynchronous)](https://medium.com/@vdongbin/javascript-%EC%9E%91%EB%8F%99%EC%9B%90%EB%A6%AC-single-thread-event-loop-asynchronous-e47e07b24d1c)
- [어쨌든 이벤트 루프는 무엇입니까? | Philip Roberts | JSConf EU](https://www.youtube.com/watch?v=8aGhZQkoFbQ&ab_channel=JSConf)
- [자바스크립트 런타임](https://beomy.github.io/tech/javascript/javascript-runtime/)
- [자바스크립트 콜백의 문제점과 프로미스 쓰는 이유](https://yuddomack.tistory.com/entry/%EC%9E%90%EB%B0%94%EC%8A%A4%ED%81%AC%EB%A6%BD%ED%8A%B8-%EC%BD%9C%EB%B0%B1%EC%9D%98-%EB%AC%B8%EC%A0%9C%EC%A0%90%EA%B3%BC-%ED%94%84%EB%A1%9C%EB%AF%B8%EC%8A%A4-%EC%93%B0%EB%8A%94-%EC%9D%B4%EC%9C%A0)