# WebClient
> Spring WebFlux가 갖고 있는 HTTP 요청을 처리하는 클라이언트

## (과거) RestTemplate
- 스프링 3.0~
- blocking 방식의 HttpClient 모듈
- 멀티쓰레드 방식: PoolingHttpClientConnectionManager 등으로 쓰레드 풀을 미리 만들어두고, 요청을 큐에 쌓는다. 1요청은 1쓰레드에 할당되며, 각 쓰레드는 블로킹으로 처리되어 응답이 올 때까지 다른 요청에 할당될 수 없다.
- [참고로 동기와 blocking은 다른 개념이다!](https://musma.github.io/2019/04/17/blocking-and-synchronous.html) 비동기 restTemplate은 AsyncRestTemplate(현재 deprecated)을 사용한다.
- [동시 사용자 1000명까지는 webClient와 큰 성능 차이를 보이지 않으나, 이후 급격하게 느려진다.](https://alwayspr.tistory.com/44) 그래서 Spring커뮤니티에서는 RestTemplate을 deprecated시키고 WebClient를 사용할것을 강력히 권고한다.


## WebClient
- 스프링 5.0~
- fully `non-blocking`, supports `streaming`
- 싱글쓰레드 방식: 코어당 한 개의 쓰레드를 이용하며 요청은 event loop 내의 job으로 등록된다. 이벤트 루프는 잡을 제공자(worker)에게 요청을 주고, 결과는 기다리지 않고 다른 잡을 처리한다. 제공자로부터 콜백이 오면 그 결과를 요청자에게 제공한다. (=이벤트 반응형)
- Reactor 기반의 함수형, fluent(메서드 체이닝 지원) API
- 쓰레드나 동시성 문제를 고려하지 않고 짤 수 있는 비동기 로직 작성을 가능하게 함


## 사용예
```java
    public Mono<String> doTest2() {
        HttpClient httpClient = HttpClient.create()
            .tcpConfiguration(
                client -> client.option(ChannelOption.CONNECT_TIMEOUT_MILLIS, 3000) //miliseconds
                    .doOnConnected(
                        conn -> conn.addHandlerLast(new ReadTimeoutHandler(5))  //sec
                            .addHandlerLast(new WriteTimeoutHandler(60)) //sec
                    )
            );    

        WebClient client = WebClient.builder()
            .baseUrl("http://localhost:5011")
            .clientConnector(new ReactorClientHttpConnector(httpClient))
            .build();
        
        return client.get() 
            .uri("/webclient/test-builder")
            .retrieve()
            .bodyToMono(String.class);          
    }
```
- create() 또는 builder()로 생성 가능. 다양한 옵션 설정을 체이닝으로 지원하기 때문에 builder가 좋다.
- timeout 지정: HttpClient를 생성하면서 timeout을 지정하고, WebClient.clientConnector를 이용하여 적용
    * WebClient에서 Reactor Netty를 사용하는데, 이것의 세부 설정을 변경하기 위해 커스텀한 httpClient를 제공하는 것
    * Read/Write, Response time, Response time for a specific request 등 상세한 타임아웃 설정 가능 
- retrieve() 메서드를 사용하여 응답을 추출할 수 있다. 하나의 값을 리턴할 때에는 bodyToMono, 복수의 값을 리턴할 때는 bodyToFlux를 사용.
- onStatus() 핸들러로 응답 상태에 따른 처리를 지정할 수 있음
- exchangeToMono(), exchangeToFlux()로 응답 결과에 따라 다르게 디코딩할 수 있음


### Mono와 Flux
- [리액티브 스트림즈](http://www.reactive-streams.org/)는 비동기 스트림 처리를 위한 표준이다.
    * 스트림은 시간의 흐름에 따라 생성되는 일련의 `signal`(=event/data)으로 정의됨
    * signal의 종류는 실제 데이터를 담는 `next`, 끝났음을 의미하는 `complete`, 에러를 의미하는 `error`가 있다.
    * Publisher는 스트림을 정의하며, Subscriber는 signal을 처리한다.
        + Publisher
            * `subscribe(Consumer<? super T> consumer`) 구독
        + Subscriber
            * `onSubscribe(Subscription s)` 구독을 하면 Publisher와 연동된 Subscription을 받고, 전달받은 Subscription을 이용해서 Publisher에 데이터를 요청
            * `onNext(T t)` Publisher가 next 신호를 보내면 호출
            * `onError(Throwable t)` Publisher가 error 신호를 보내면 호출
            * `onComplete()` Publisher가 complete 신호를 보내면 호출된다.
    * Subscriber가 Publisher로부터 signal을 받는 것을 구독이라고 한다. 실제 값의 발생은 구독 시점에 이루어진다.
- Reactor는 리액티브 스트림즈의 구현으로, 스프링이 리액티브 프로그래밍 지원을 하기 위한 핵심 모듈이다.
    * 리액티브 프로그래밍을 하는 이유?
        + 탄력적(resilience), 성능, 확장성, 메시징 패턴 등
        + 비동기 + 논블로킹한 성질을 이용해 더 적은 자원으로 더 많은 트래픽을 처리하기 위함
    * 리액터에서는 스트림이란 용어 대신 시퀀스라는 용어를 사용함
    * 콜드 시퀀스는 구독 시점에 데이터를 새로 생성하며, 핫 시퀀스는 구독 여부에 관계 없이 데이터가 생성되고 구독 시점부터 signal을 받게 된다. **WebClient의 요청은 콜드 시퀀스**에 해당한다.
- Mono와 Flux는 Reactor의 주요 객체.
    * 둘 다 Publisher
    * Flux는 0-N 개의 데이터를, Mono는 0-1 개의 데이터를 발생시킨다
    * **WebClient 클래스를 사용할 때에는 웹클라이언트가 생성하는 Mono를 이용해서 데이터를 처리하는 형태**


## tips
- `org.springframework.core.io.buffer.DataBufferLimitException: Exceeded limit on max bytes to buffer` : Request Data를 버퍼링하기 위한 메모리의 기본값은 256KB라서 이런 에러가 발생하면 버퍼 메모리를 늘려야 함. ExchangeStrategies로 메모리를 지정하고, WebClient.exchangeStragegies로 적용.
- webClient를 만드는 시간이 필요하므로 필요할 때마다 만들기보다 Bean으로 등록하는 것이 좋다


## References
- [Spring WebFlux 2. WebClient](https://docs.spring.io/spring-framework/docs/current/reference/html/web-reactive.html#webflux-client)
- [Spring WebClient 쉽게 이해하기](https://happycloud-lee.tistory.com/220)
- [스프링 리액터 시작하기 1 - 리액티브 스트림 Flux Mono Subscriber](https://javacan.tistory.com/entry/Reactor-Start-1-RS-Flux-Mono-Subscriber)
- [[Spring Reactive] WebClient](https://binux.tistory.com/56)
- [웹플럭스?](https://devuna.tistory.com/m/108)