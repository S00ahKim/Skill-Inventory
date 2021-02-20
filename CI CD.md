# CI/CD
![CICD_파이프라인](images/CICD_1.png)
개발자들의 개발 환경을 사용자가 사용 가능한 서비스로 전달하는 모든 과정을 지속 가능한 형태로, 가능한 자동화하는 것 ex. 빌드, 테스트, 배포


## CI (Continuous Integration, 지속적 통합)
- Continuous Integration? 여러 개발자들의 코드 베이스를 계속해서 통합하는 것
- 핵심 목표
    * 버그를 빠르게 찾고 해결
    * 서비스 품질 개선
    * 새로운 업데이트의 검증 및 릴리즈 시간 단축
- CI가 있는 개발 과정
    1. n명의 개발자가 개발한다 -> `commit`
    2. 테스트 통과 여부 확인 -> `commit`
        + 다른 개발자가 작성한 코드와 차이가 발생하지 않는지 로컬에서 테스트
    3. 코드 베이스에 merge
        + 통과할 경우 dev branch(@git과 같은 SCM)에 커밋
- CI의 장점
    * 다수의 개발자가 협업할 때 자동으로 빌드와 테스트를 수행하여 충돌을 막음
    * MSA 환경에서 작은 기능들 간 기능 충돌을 막음


## CD (Continuous Delivery & Continuous Deployment)
- Continuous Delivery? 코드 베이스가 항상 배포 가능한 상태를 유지하는 것. (공유 repo)
- Continuous Deployment? 코드 베이스를 사용자가 사용 가능한 환경에 배포하는 것을 자동화하는 것. (production env.)
- 핵심 목표
    * 사용자에게 최대한 빠르게 신뢰 가능한 수준의 최신 서비스를 제공
    * 개발팀과 비즈니스팀(QA팀, 기획팀 등) 간 커뮤니케이션에 도움
- CD가 있는 개발 과정
    1. n명의 개발자가 개발한다
    2. merge가 성공했다면 개발자의 역할은 끝
        + 젠킨스는 dev branch의 내용을 개발 환경에 배포하기 전에 테스트/린트 등 `코드 포맷팅`을 함
        + 배포하기 위한 `빌드` 과정을 거치고, 코드를 `배포`한 뒤, `테스트`를 한다.
- CD의 장점
    * 수동으로 콘솔에서 배포하지 않아도 됨
    * 배포 스크립트 작성/베어메탈 등의 번거로움이 줄어듦
    * 서버별, 환경(dev, prod 등)별 자잘한 버그 발생 가능성이 줄어듦


## 개발 환경
1. Local: 개발자가 개발하는 환경
2. Development: 개발자들끼리 개발 내용에 대한 통합 테스트를 진행하는 환경
3. QA: 개발 완료 후 QA 엔지니어 및 내부 사용자들을 위한 환경
4. Production: 사용자들을 위한 환경 (실제 유저 사용)


## CI/CD의 핵심
여러 배포 환경의 관리
* 인프라를 모듈화하여 어떤 것이 변수인지 잘 설정하고 이를 잘 설계하는 것 
* 배포하려는 환경에 대해 각기 다른 다양한 변수들을 잘 가져다 쓰는 것 (환경변수 관리, 인프라별 키 관리)
* 예) AWS System Manager - parameter store
* 변수? 데이터베이스 PW, 서버 백엔드 url, 암호화 해시 키 등


## Jenkins
- Jenkins?
    * 자바 런타임 환경에서 동작하는 자동화 서버
    * 빌드, 테스트, 배포 등 모든 것을 자동화하는 서버
- 특징
    * 다양한 `플러그인`들을 활용하여 각종 자동화 작업을 처리할 수 있음
    * 자동화 작업을 잘 조합해서 순서 집합인 `파이프라인`을 구축할 수 있음
- 대표적인 플러그인 (초기 세팅 때 추천하는 거 깔면 웬만한 건 다 깔려 있음)
    * credentials plugin: 중요한 정보를 저장하는 데 사용
        + 젠킨스는 그냥 서버이기 때문에 배포에 필요한 각종 리소스에 접근하기 위해 토큰 같은 걸 저장해야 함
        + RSA(고유아이디) 기반 공개키 방식으로 암호화하여 안전함
    * git plugin: 젠킨스가 소스코드를 긁어와서 빌드할 때 사용
    * `pipeline`: 일련의 플러그인 집합 구성.
        + 어떤 순서대로 일을 처리할 것인가. 일종의 작업 명세.
        + 여러 플러그인을 파이프라인에서 용도에 맞게 사용하고 정의함
        + Pipeline DSL(Domain Specific Language)로 작성
        + syntax는 선언적(`declarative`, 최신, 가독성 좋음), 스크립트(scripted)의 두 가지 파이프라인이 존재함
    * docker plugin: 도커 다운로드 및 사용을 위함


### 젠킨스 파이프라인 구성
```
pipeline {
    agent any
    stages {
      stage('test1 : Using when_01') {
        # when에서 설정한 조건이 True일 경우에만 steps이 실행된다.
        when { branch 'master' }  # branch가 master인 경우 다음 steps이 실행됨
        steps {
          echo 'It is executed'
        }
      }
...
```
1. Section
    - Agent Section: 젠킨스 서버가 여러대일 경우, 어떤 노드에게 일을 시킬 것인지 지정 가능. 새로 노드를 띄우거나 도커 이미지로 환경 구성 등.
    - Post Section: 각 순서가 끝났을 때, 성공했으면 할 일, 실패했으면 할 일을 명시
    - Stages Section: 어떤 일을 처리할 건지 순서(스테이지) 카테고리를 정의함
    - Steps Section: 한 스테이지 안에서의 단계로, 일련의 스텝을 보여줌
        + steps는 여러 스텝들로 구성됨
        + 플러그인을 설치하면 사용할 수 있는 스텝이 생김
        + 여러 작업 실행 가능
2. Declarative
    - 각 스테이지 안에서 어떤 일을 하는지 정의함
    - Environment: 어떤 파이프라인이나 스테이지 스코프의 환경 변수 설정
    - Parameter: 파이프라인 실행 시 파라미터 받음
    - Triggers: 어떤 형태로 트리거되는가
    - When: 언제 실행되는가


## References
- [[토크ON세미나] Jenkins를 활용한 CI/CD | T아카데미](https://www.youtube.com/watch?v=JPDKLgX5bRg&ab_channel=SKplanetTacademy)