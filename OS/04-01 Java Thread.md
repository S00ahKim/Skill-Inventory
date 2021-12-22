# 자바의 멀티 쓰레딩
> 자바의 멀티 쓰레드는 동시성Concurrency 또는 병렬성Parallelism으로 실행된다.
- 동시성: 하나의 코어에서 다수의 쓰레드가 실행되어 동시에 도는 것처럼 보임
- 병렬성: 쓰레드마다 코어가 할당되어 각각의 코어에서 개별 쓰레드가 동시에 실행됨
    - 쓰레드 수가 코어 수보다 많을 경우에는 쓰레드 스케줄링(어떻게 동시성을 구현할 것인지를 결정) 필요
    - 자바의 쓰레드 스케줄링은 개발자가 제어 가능한 `우선 순위`방식, 제어 불가능한 `라운드 로빈` 방식 2가지 제공


# 버전별 병렬 처리 변화
## ~자바6
### 이전까지 자바에서 지원한 병렬 처리 방식
1. 데이터를 서브 파트로 분할
2. 서브 파트를 각각의 쓰레드에 할당
3. 의도하지 않은 race condition 발생을 막기 위해 동기화
4. 부분으로 작업한 결과 합치기

### 아주 기본적으로 제공하는 Thread
* 자바는 문법적으로 Thread를 지원함
* 새로운 쓰레드를 실행시키려면, 명시적으로 task를 할당해야 함
* task를 할당하기 위해 Runnable 이라는 인터페이스를 사용 (람다식 사용/구현 등)
  ```java
  public static void main(String[] args) { //메인 쓰레드에서 직접 실행
    Runnable task = () -> { //람다식으로 할 일을 지정하는 예시
    String threadName = Thread.currentThread().getName();
    System.out.println("Hello " + threadName);
    };
    
    task.run();
    
    Thread thread = new Thread(task); //명시적으로 생성 가능
    thread.start();
    
    System.out.println("Done!"); //이게 언제 실행될지는 예측 불가능
    /*
    * 즉, Hello main -> Done! -> Hello Thread-0 순으로 찍힐지
    * Hello main -> Hello Thread-0 -> Done! 순으로 찍힐지는 모른다는 것
    * 이처럼 쓰레드를 다루는 일은 고려해야 할 것이 많은 골치 아픈 일
    */
  }
  ``` 

### java.util.concurrent
* 자바5에서 추가된 패키지
* '~자바6' 항목에 포함시켰으나, 이후 꾸준히 보강되어 자바8의 람다식 역시 지원함
* 동기화가 필요한 상황에서 사용할 수 있는 다양한 유틸리티 클래스들을 제공
