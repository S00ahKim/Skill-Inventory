# Spring Framework
1. 핵심 요소
    * POJO: Plain Old Java Object (순수 자바 오브젝트. 비즈니스 복잡성 낮추기 위함.)
    * AOP :Aspect Oriented Programming (공통 관심사를 해결하기 어려운 경우 ex. 보안, 로깅, 트랜잭션)
    * DI/IoC :Dependency Injection and Inversion of Control (프레임워크(스프링 컨테이너)가 소스코드를 제어(ex. 프록시 빈 등)한다.)
    * PSA :Portable Service Abstraction (인터페이스를 사용하여 기술을 일관성 있게, 소스를 적게 수정하면서 접근하는 방법)
2. 스프링 프레임워크 모듈
    * 데이터 접근 (JDBC, Transaction, ORM etc.)
    * WEB (websocket, servlet, web etc.) -> `spring-web`, `spring-webmvc`(웹앱을 위한 REST 지원 및 MVC 구현체)
    * AOP, Aspects, Instrumentation, Messaging
    * Core Container (Bean, Context, Core, SpEL)
    * Test


## Spring Web MVC framework
- 스프링 프레임워크의 자체 웹 프레임워크 (스프링은 다른 MVC 프레임워크 사용 가능)
- 특징
    * 핸들러에게 요청을 전달하는 DispatcherServlet를 중심으로 설계
    * 핸들러 매핑,뷰 처리,로케일,타임존,파일 업로드 지원,테마 처리 등 제공
    * 기본 핸들러(defaulthandler)는 @Controller와 @RequestMapping 애노테이션을 기반으로 하여 다양하고 유연한 핸들링 메서드를 제공
    * 스프링 3.0부터는 @Controller메커니즘을 사용하여 @PathVariable 애노테이션 및 다른 기능을 통해 RESTful웹 사이트 및 애플리케이션을 만들 수 있음
- 환경
    * 스프링 버전에 맞는 JDK (ex. Spring4: JDK 6이상)
    * 스프링 버전에 맞는 Servlet (ex. Spring4: Servlet 3 이상)
    * 스프링 버전에 맞는 웹서버 (Tomcat etc.)
- 구조
    * 레이어 아키텍처 (영역별로 레이어를 구분 by 패키지, 레이어별 책임)
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
            * HandlerMapping: 요청 url을 컨트롤러와 매핑
            * Controller: 요청을 실제로 처리
            * View Resolver: 처리 결과를 보여줄 뷰 정함
            * View: 처리 결과를 보여줌
            * ModelAndView: 처리 결과 데이터와 뷰를 보여줌


## 주요 설정 파일
1. pom.xml
    - 라이브러리 관리를 위한 Maven 설정
    - 빌드 및 배포와 관련한 모든 설정, 필요한 라이브러리 명시 등
    - Maven? 프로젝트 생성, 테스트 빌드, 배포 등의 작업을 위한 전용 프로그램
2. web.xml
    - 웹과 관련한 설정 (Web Application의 Deployment Descriptor(환경 파일))
    - 리스너, 어플리케이션 파라미터, 서블릿 설정, 필터 설정 등
    - 클라이언트에게 받은 요청을 전달하고, 응답을 돌려주는 곳. DispatcherServlet을 여기서 설정.
3. servlet-context.xml
    - 서블릿 영역 설정 (servlet-config)
4. root-context.xml
    - 애플리케이션 영역 설정 (mvc-config)


## 참고자료
- [스프링 공식 문서](https://docs.spring.io/spring-framework/docs/current/reference/html/overview.html)