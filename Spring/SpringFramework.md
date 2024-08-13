# Spring Framework
1. 핵심 요소
    * POJO: Plain Old Java Object (순수 자바 오브젝트. 비즈니스 복잡성 낮추기 위함.)
        - Spring Framework는 POJO를 이용한 엔터프라이즈 애플리케이션 개발을 목적으로 하는 프레임워크
        - 특정한 기술과 환경에 종속되지 않는 오브젝트이기 때문에 깔끔한 코드 제공, 객체지향적 설계 가능, 자동화된 테스트에 유리함.
    * AOP :Aspect Oriented Programming (공통 관심사를 해결하기 어려운 경우 ex. 보안, 로깅, 트랜잭션)
        - 스프링의 경우, 런타임 시 관점으로 분리된 Advice를 핵심 로직 코드에 적용(Weaving)하는, `프록시`를 이용한 구현을 채택함.
        - 중간에 프록시를 생성하여 프록시를 통해 핵심 로직을 구현한 객체에 접근하는데, 이때, 프록시의 핵심 로직을 실행하기 전 혹은 후에 공통 기능을 적용하는 방식
        - 프록시 방식의 한계? 메서드가 호출될 때만 Advice를 적용할 수 있기 때문에, 필드 값 변경과 같은 JoinPoint에 대해서는 적용 불가
    * DI/IoC :Dependency Injection and Inversion of Control (프레임워크(스프링 컨테이너)가 소스코드를 제어(ex. 프록시 빈 등)한다.)
        - 두 개의 오브젝트를 분리해서 만들고, 인터페이스를 두고 느슨하게 연결한 뒤, 실제 사용할 대상은 DI를 통해 외부에서 지정
        - DI는 OCP를 준수함.
        - 3가지 유형? 생성자, setter(스프링 권장), 초기화 인터페이스(스프링 미지원)
    * PSA :Portable Service Abstraction (인터페이스를 사용하여 기술을 일관성 있게, 소스를 적게 수정하면서 접근하는 방법)
        - Spring 패키지 외의 것들을 POJO화 시키기 위해 껍데기를 씌우는 것
        - ex. JUnit, MyBatis (일반적인 것과 스프링에서 지원하는 것이 다름.)
2. 스프링 프레임워크 모듈
    * 데이터 접근 (JDBC, Transaction, ORM etc.)
    * WEB (websocket, servlet, web etc.) -> `spring-web`, `spring-webmvc`(웹앱을 위한 REST 지원 및 MVC 구현체)
    * AOP, Aspects, Instrumentation, Messaging
    * Core Container (Bean, Context, Core, SpEL)
    * Test

## Spring Web MVC framework
- 스프링 프레임워크의 자체 웹 프레임워크 (스프링은 다른 MVC 프레임워크 사용 가능)
1. 특징
    * 핸들러에게 요청을 전달하는 DispatcherServlet를 중심으로 설계
    * 핸들러 매핑,뷰 처리,로케일,타임존,파일 업로드 지원,테마 처리 등 제공
    * 기본 핸들러(defaulthandler)는 @Controller와 @RequestMapping 애노테이션을 기반으로 하여 다양하고 유연한 핸들링 메서드를 제공
    * 스프링 3.0부터는 @Controller메커니즘을 사용하여 @PathVariable 애노테이션 및 다른 기능을 통해 RESTful웹 사이트 및 애플리케이션을 만들 수 있음
2. 환경
    * 스프링 버전에 맞는 JDK (ex. Spring4: JDK 6이상)
    * 스프링 버전에 맞는 Servlet (ex. Spring4: Servlet 3 이상)
    * 스프링 버전에 맞는 웹서버 (Tomcat etc.)
3. 구조
    * 레이어 아키텍처 (영역별로 레이어를 구분 by 패키지, 레이어별 책임)
        + 프리젠테이션 레이어
        + 서비스 레이어
        + 데이터 액세스 레이어
    * 관심사의 분리 (AOP)
        + 계층 간 느슨한 coupling
        + 유연하고 견고한 설계 지원
    * JSP 모델 2 (MVC)
        + JSP 모델에 MVC 패턴을 도입한 것
        + 요청을 바로 JSP에 전달하지 않고, 서블릿이 `컨트롤러` 역할을 함
        + JSP는 프리젠테이션만 담당하는 `뷰` 역할을 함
    * Front Controller 패턴
        + 왜? 하나의 집중적인 접근 지점이 필요해서
        + 역할? 요청을 컨트롤러에 전달하고, 결과를 받아 뷰에 전달함
        + 이 프론트 컨트롤러가 `DispatcherServlet`
            - HandlerMapping: 요청 url을 컨트롤러와 매핑
            - Controller: 요청을 실제로 처리
            - View Resolver: 처리 결과를 보여줄 뷰 정함
            - View: 처리 결과를 보여줌
            - ModelAndView: 처리 결과 데이터와 뷰를 보여줌
4. 기본 애노테이션
    * `@Component`: 개발자가 작성한 클래스를 Bean으로 등록하기 위함. 컨트롤러나 서비스 등 특정 목적을 가지고 있을 경우, 원활한 예외 처리를 위해 구체적으로 명시함.
    * `@Bean`: 개발자의 직접 제어가 불가능한 외부 라이브러리 등을 Bean으로 등록하기 위함.
    * `@Autowired`: Bean을 주입함. (객체에 대한 의존성 주입) 스프링이 자동으로 값 할당.
        + Bean을 주입하는 방법
            - @Autowired
            - setter
            - 생성자 (@AllArgsConstructor 사용) -> 권장
    * `@Controller` + `@ResponseBody` = `@RestController` (Spring 4~)
    * `@Service`
    * `@Repository`
5. HTTP 요청과 응답과 관련한 애노테이션
    HTTP 메시지는 요청 메서드, URL, header, body 등으로 구성됨.
    * Request
        + `@RequestMapping`: URL과 컨트롤러 연결. 요청 메서드 중 일부만 사용하는 등 파라미터로 조정 가능.
        + `@PathVariable`: 주소 경로의 일부를 파라미터로 사용할 수 있음.
        + `@RequestHeader`: Request의 header값을 가져올 수 있다. 메소드의 파라미터에 사용.
        + `@RequestBody`: 요청으로 온 데이터를 바로 Class나 model로 매핑하기 위함.
        + `@RequestPart`: Request로 온 MultipartFile을 바인딩
        + `@RequestParam`: @PathVariable과 유사. HTTP GET 요청에 대해 매칭되는 request parameter 값을 가져옴. 
        + `@ModelAttribute`: Request에서 getter/setter가 있는 Bean으로 변환할때 사용.
    * Response
        + `@ResponseStatus`: Status코드를 전달할 때 사용.
        + `@ResponseBody`: View페이지 없이 직접 객체를 JSON이나 XML로 반환할 때 사용.

## 주요 설정 파일
설정 파일의 역할과 이름은 프로젝트 별로 다를 수 있다. 삭제되는 경우도 있음. 대체적으로 사용되는 경우를 기술함.
1. pom.xml
    - 라이브러리 관리를 위한 Maven 설정
    - 빌드 및 배포와 관련한 모든 설정, 필요한 라이브러리 명시 등
    - Maven? 프로젝트 생성, 테스트 빌드, 배포 등의 작업을 위한 전용 프로그램
2. web.xml
    - 웹과 관련한 설정 (Web Application의 Deployment Descriptor(환경 파일))
    - WAS의 구동을 위한 설정. 다른 xml 파일을 인식하게 함.
    - 리스너(root-context 선언을 위함), 어플리케이션 파라미터, 서블릿 설정, 필터 설정 등
3. servlet-context.xml
    - 서블릿 영역 설정 (servlet-config)
    - 요청과 관련한 객체(Bean)를 정의
    - DispatcherServlet, url과 관련된 controller나, @(어노테이션), ViewResolver, Interceptor, MultipartResolver 등의 설정
4. root-context.xml
    - 애플리케이션 영역 설정 (mvc-config)
    - view와 관련되지 않은 객체(Bean)를 정의
    - Service, Repository(DAO), DB등 비즈니스 로직과 관련된 설정
    - 다수의 dispatcher servlet을 사용할 경우, 각각의 application context를 가지는데, 이때 spring bean을 공유하기 위함 (-> 리스너: web.xml)
5. application-context.xml
    - 애플리케이션 영역 설정
    - mvc 애노테이션 드리븐 등록, 컨트롤러와 뷰 등록을 위해 컴포넌트 스캔, 논리 주소를 절대 주소로 바꾸기 위해 인터널 뷰 리졸버 설정


### Application Context?
1. Bean: 스프링이 IoC 방식으로 관리하는 오브젝트. 생성과 제어를 담당하는 오브젝트만 해당함.
2. Bean factory: 스프링의 IoC를 담당하는 핵심 컨테이너. Bean 등록, 생성, 조회
3. Application context: 빈 팩토리를 확장한 IoC 컨테이너. 추가로 스프링의 부가 서비스 제공.
    * Root WebApplicationContext (by. ContextLoaderListener)
    * WebApplicationContext (by. DispatcherServlet): 서블릿에서만 이용됨.
        - DispatcherServlet이 직접 사용하는 컨트롤러를 포함한 웹 관련 빈을 등록하는 데 사용
        - DispatcherServlet은 독자적인 WebApplicationContext를 가지고 있고, 모두 동일한 Root WebApplicationContext를 공유
4. 컨테이너: 애플리케이션 컨텍스트보다 추상적인 표현. IoC 방식으로 빈을 관리한다는 의미에서 애플리케이션 컨텍스트나 빈 팩토리를 컨테이너 또는 IoC 컨테이너라고도 부름.


## 참고자료
- [스프링 공식 문서](https://docs.spring.io/spring-framework/docs/current/reference/html/overview.html)
- 토비의 스프링 3.1
- [스프링 애노테이션 정리](https://velog.io/@gillog/Spring-Annotation-%EC%A0%95%EB%A6%AC)
- [스프링 주요 개념 정리](https://jinpyo900.tistory.com/55)
- [web.xml 파일 및 ApplicationContext에 대하여](https://rebeccajo.tistory.com/10)