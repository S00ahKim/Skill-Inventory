# 스파크 설정
- 애플리케이션 단위 설정 -> Spark Property
- 각 서버 단위 설정 -> 각 서버의 환경변수


## 스파크 프로퍼티
```scala
val conf = new SparkConf().setAppName("wordCount")
val sc = new SparkContext(conf)
```
- 개별 애플리케이션 실행과 관련한 설정값 정의
- SparkContext 생성시 사용한 SparkConf 인스턴스 / 자바 시스템 프로퍼티 사용
- 로직과 무관한 설정(ex. 익스큐터 메모리 설정, 코어 수 할당)이 코드에 포함됨 -> 프로그램 실행 시점에 동적으로 설정 가능
    * 실행 옵션 (ex. `--executor-cores 2`) (해당 옵션을 준 실행에 적용)
    * $SPARK_HOME/conf 하에 spark-defaults.conf 파일 생성 (해당 서버의 모든 스파크 앱에 적용)
        + [공식 문서](https://spark.apache.org/docs/latest/configuration.html)
        + 애플리케이션, 실행 환경, 셔츨, 스파크 UI, 압축과 serialization, 메모리, 익스큐터, 네트워크, 보안, 암호화 관련 설정 지원
    * 설정이 중복될 경우, SparkConf 메서드 > 실행 옵션 > spark-defaults.conf 순으로 우선순위 가짐
- [Spark2.3.2 SparkConf docs](https://spark.apache.org/docs/2.3.2/api/java/org/apache/spark/SparkConf.html)


## 환경변수
- 각 서버 단위로 적용돼야 하는 환경 정보 (ex. 자바 설치 경로)
- 클러스터 매니저의 종류에 따라 설정 방법이 다르므로 확인 필요
    * yarn에서 cluster mode 실행을 위해서는 애플리케이션 마스터가 사용할 환경변수를 설정하려면...
        + spark-defaults.conf의 `spark.yarn.appMasterEnv.환경변수이름`을 이용 (O)
        + spark-env.sh (X)


## 로깅 설정
- 사용 프레임워크: log4j
- 로깅 레벨 변경 -> $SPARK_HOME/conf/log4j.properties 설정 (없을 경우 .template이 붙은 파일 복사 후 변경)
- 로그 파일 생성 위치
    * stand-alone mode: 각 슬레이브 노드의 $SPARK_HOME/work/stdout, stderr
    * mesos cluster mode: /var/log/mesos
    * yarn
        + yarn.log-aggregation-enable:true -> yarn.nodemanager.remote-app-log-dir에 지정된 위치
        +                             미사용 -> 각 노드의 로컬 파일시스템


## 스케줄링
- 클러스터에서 앱 동작 구조에 따라 컴퓨팅 자원을 적절히 분배하는 것이 중요함

### 애플리케이션 간의 자원 스케줄링
- 동일한 클러스터에서 서로 다른 애플리케이션이 동시 실행하는 경우

1. 고정 자원 할당 방식(static resource allocation)
    * 각 애플리케이션 단위로 고정된 자원을 할당
    * 애플리케이션 실행~종료 시점까지 할당된 자원을 계속 점유 & 사용
    * standalone, yarn, mesos의 fine-grained 모드
    * 추천: 짧은 시간에 집중적으로 처리를 수행하는 애플리케이션
2. 동적 자원 할당 방식(dynamic resource allocation)
    * 애플리케이션 실행 상황에 따라 수시로 할당량 조정
    * 애플리케이션이 실제로 작업을 수행하는 경우에만 할당, 대기 중에는 회수하여 다른 곳에 할당
    * 스파크 프로퍼티 속성으로 사용
        + 공통 설정: `spark.dynamicAllocation.enabled:true` (클러스터 매니저 종류와 무관)
        + standalone 모드
            1. 각 워커 노드를 `spark.shuffle.service.enabled:true` 설정
            2. 클러스터 실행
        + mesos 모드
            1. `spark.mesos.coarse:true`, `spark.shuffle.service.enabled:true` 설정
            2. $SPARK_HOME/sbin/start-mesos-shuffle-service.sh 스크립트를 실행해 외부 셔플 프로세스 실행
        + yarn 모드
            1. 모든 노드 매니저 클래스패스에 스파크 배포본의 spark-{version}-yarn-shuffle.jar 파일을 등록
            2. 각 노드 매니저의 yarn-site.xml 파일에 spark_shuffle 속성을 yarn. odemanager.aux-services로 설정
            3. yarn.nodemanager.aux-services.spark_shuffle.class 속성을 org.apache.spark.network.yarn.YarnShuffleService로 설정
            4. spark.shuffle.service.enabled 속성을 true로 설정
            5. yarn 재실행
        + 동적 자원 할당 모드에서는 앱이 실행되고 있는 중에 익스큐터가 스케줄러에 의해 삭제될 수 있어서, 익스큐터가 삭제되어도 작업하는 데이터를 유지하고 처리할 수 있도록, standalone외의 모드들은 별도의 셔플 서비스 구동함
    * 추천: spark-shell/웹 개반 앱처럼 `장시간 동작`하거나 이벤트가 있을 때 작업을 처리하는 식으로 동작하여 `대기 시간`이 발생하는 경우

### 단일 애플리케이션 내부에서의 자원 스케줄링
- 하나의 애플리케이션에서 여러 스레드로 다수의 잡을 동시 실행하는 경우
- sparkContext는 멀티스레드 방식을 지원함 (deafult: FIFO로 잡 실행)
- 스케줄러 선택 옵션 제공
    1. FIFO 스케줄러 (default)
        + 단점: 모든 자원을 점유한 채 첫 작업이 시작되면 이후 작업이 오래 대기함
    2. Fair 스케줄러
        + 라운드 로빈 방식 사용
        + 장점: 작은 잡이 크기가 큰 잡을 기다리지 않고 빠른 처리 가능
        + 단점: 항상 올바른 것은 아니고, 경우에 따라 중요도로 자원 할당 우선순위 조절 필요 -> Pool 개념 도입 
    * 스케줄러 설정: sparkConf 설정 / spark-default.conf 설정
- Pool
    * 풀마다 스케줄링 방식, 우선순위, 자원 할당 수준을 다르게 설정하여 각 잡을 특정 풀에 할당해 풀 단위로 작업 관리
    * 장점: 우선순위 할당 가능, 정기적/고정적 작업과 일시적 작업을 분리해서 관리 가능
    * 사용 방법
        1. $SPARK_HOME/conf/fairscheduler.xml 파일로 설정 (weight로 상대적 우선 순위 부여 가능)
        2. 설정 파일 위치 등록 `conf.set("spark.scheduler.allocation.file", "설정파일경로")`
        3. 잡을 실행할 때 사용할 풀 지정 `sc.setLocalProperty("spark.scheduler.pool", "pool1")` (기본 풀 사용시 null)