@SpringBootApplication는 @ComponentScan과 @EnableAutoConfiguration을 포함하고 있음
- @EnableAutoConfiguration은 @ComponentScan과 함께 사용되며, auto configuration 기능을 사용하겠다는 설정
- EnableAutoConfiguration에는 상당히 많은 AutoConfigruation이 등록되어 있지만, 각 AutoConfigruation들은 필요한 상황에만 자신이 실행될 수 있도록 @Conditional, @Condition과 같은 annotation들로 설정이 되어있음
* 특정 빈이 생성되지 않은 경우에만 실행되게 -> 다른 곳에서 생성할 경우 AutoConfiguration의 빈은 또 생성되지 않음
- @Lazy 로 Spring Boot 기동시에 생성되지 않고 ThreadPoolTaskExecutor가 필요한 상황에서 bean이 생성이 요청


@AutoConfigureWebClient // 네티 내부에서 사용하는 설정들을 알아서 호출해줌. -> 테스트코드
