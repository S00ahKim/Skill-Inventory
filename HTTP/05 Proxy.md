# 프록시

## 웹 중개자, 프록시
- 서버이자 클라이언트
- 개인 vs 공용
    * 개인
        + 흔하지는 않지만 꾸준히 사용됨. 
        + 특히 클라이언트 컴퓨터에서 직접 실행의 형태.
            - 브라우저 보조 앱 (브라우저 기능 확장, 성능 개선 등)
            - 무료 ISP 서비스를 위한 광고 운영을 위한 작은 프록시
            - cf. ISP = 인터넷 서비스 제공자(Internet service provider). 인터넷에 접속하는 수단을 제공하는 주체를 가리키는 말. KT망이면 KT. 
            - 취약점 분석 (Burp Suite) 등 
        + 하나의 클라이언트와만 작동
    * 공용
        + 대부분의 프록시
        + 캐시 프록시 등: 사용자가 많을수록 유리
- 프록시 vs 게이트웨이
    * 프록시: 같은 프로토콜을 사용하는 둘 이상의 앱을 연결
    * 게이트웨이: 다른 프로토콜을 사용하는 둘 이상의 앱을 연결
    * 차이는 모호함. 
        + HTTP 버전 문제 때문에 프록시도 프로토콜 변경이 있음. 
        + 상용 프록시는 SSL 보안, SOCKS 방화벽, FTP 접근, WAS 지원을 위해 게이트웨이 기능 구현
        + cf. SOCKS = 서버-클라이언트 간의 TCP나 UDP 통신을 프록시 서버를 거쳐 진행하도록 해 주는 프로토콜


## 프록시가 하는 일
- 실용적인 일, ex. 보안 개선, 성능 향상, 비용 절감, 트래픽 감시 및 수정...
1. 어린이 필터 (초등학교의 필터링 프록시)
2. 문서 접근 제어자 (보안. 접근 제어 & audit trail (활동 로깅)) ex. Pulse Secure
3. 보안 방화벽 (네트워크의 흐름을 한 지점에서 통제, 트래픽 후크)
    * 후킹: 소프트웨어 공학 용어로, 운영 체제나 응용 소프트웨어 등의 각종 컴퓨터 프로그램에서 소프트웨어 구성 요소 간에 발생하는 함수 호출, 메시지, 이벤트 등을 중간에서 바꾸거나 가로채는 명령, 방법, 기술이나 행위
    * 모든 데이터 흐름이 특정 프록시들을 거쳐가게 함으로써 트래픽 감시하거나 부하 컨트롤
    * 바이러스나 불량 이용자 등에 대해 트래픽을 차단하거나 수정 ex. DDoS 공격을 프록시로 차단, 웹 방화벽(WAF) 같은 앱이 SQL Injection, XSS 등의 공격을 탐지하고 차단
4. 웹 캐시
    * static 리소스처럼 캐싱하기 쉽거나 캐시할 필요가 있는 경우
    * ex. CDN: 콘텐츠를 효율적으로 전달하기 위해 여러 노드를 가진 네트워크에 데이터를 저장하여 제공하는 시스템. ISP와 직접 연결되어 병목 회피.
5. 대리 프록시, 리버스 프록시
    * 웹 서버인 것처럼 위장하는 프록시. 다른 서버의 정보/리소스를 이 프록시를 통해 받아옴.
    * 일반적인(포워드) 프록시와의 차이: 사용자가 대리 프록시를 endpoint로 여김. (='진짜' 요청)
    * 왜? HA를 위한 로드밸런싱, 보안(내부 DB 등 주요 서버와 분리) 등
    * ex. nginx, apache 웹 서버
    * 서버 가속기: 공용 콘텐츠에 대해 느린 웹 서버 성능 개선을 위해 사용되는 대리 프록시를 흔히 부르는 말.
    * 대리 프록시 + 콘텐츠 라우팅 기능 = 주문형 복제 콘텐츠의 분산 네트워크 (?)
6. 콘텐츠 라우터
    * 인터넷 트래픽 조건과 콘텐츠의 종류에 따라 요청을 특정 웹서버로 유도
    * ex. CDN, 높은 성능을 위한 유료 서비스 구독자의 요청은 복제 캐시로 전달 등
7. 트랜스코더
    * 트랜스코딩 = 데이터 표현 방식을 자연스럽게 변환하는 것
    * 클라이언트에게 전달 전에 프록시가 서버의 응답을 수정
    * ex. 파일 압축, GIF->JPG, 번역
8. 익명화 프락시
    * 신원 식별 가능한 정보(ex. 클라이언트 ip, from 헤더, 쿠키, uri 세션 아이디 등) 제거


## 어디에?
### 프록시가 네트워크에 배치되는 방법
어떻게 사용할지에 따라 어디에든 배치 가능

1. (LAN의) 출구 프록시
    * where? 로컬 네트워크의 출구
    * why? 로컬 네트워크와 인터넷의 트래픽 제어 용도
    * ex. 보안, 콘텐츠 필터링
2. (ISP의) 입구 프록시 (접근 프록시)
    * where? ISP 접근 지점
    * why? 클라이언트의 모든 요청을 종합적으로 처리하려는 용도
    * ex. 트래픽 제어, 캐시
3. 대리 프록시
    * where? 웹 서버 앞단
    * why? 웹 서버로 향하는 요청을 제어하려는 용도 
    * ex. 필요할 때만 요청, 보안, 캐시
4. 네트워크 교환 프록시
    * where? 네트워크와 네트워크 사이
    * ex. 혼잡 제어와 트래픽 감시(국가 안보 등)

### 프록시의 연쇄는 계층을 이룬다
- 프록시는 프록시 계층이라고 불리는 연쇄를 구성할 수 있음
- 프록시 계층에서 클라이언트에 가까운 프록시는 `자식`, 서버에 가까운 프록시는 `부모`
- 항상 왔던 길로만 오가는 것(정적 프록시)이 아님
- 동적 부모 선택 (로직은 설정 파일, 언어, 플러그인별로 상이)
    1. 부하 균형: 부모의 작업 수준에 따라 부모를 선택
    2. 지리적 인접성에 근거한 라우팅: 원 서버와 가까운 지역의 부모 선택
    3. 프로토콜/타입 라우팅: uri에 근거하여 어떤 uri에 대해서는 특정 부모/서버를 선택
    4. 유료 서비스 가입자를 위한 라우팅: 성능을 위해 비용을 지불한 서비스는 대형 캐시/압축 엔진으로 라우팅

### 프록시가 트래픽을 처리하는 방법
> 트래픽은 어떻게 프록시를 찾아가는가?

1. 클라이언트 수정
    * 유명 브라우저와 같은 클라이언트는 자체적으로 수동/자동 프록시 지원
    * 사용하게 설정되어 있다면, 목표 서버가 아니라 프록시로 요청 보냄
2. 네트워크 수정
    * 클라이언트 모르게 스위칭/라우팅 장치로 트래픽을 프록시로 보냄 -> `인터셉트 프록시` 
3. DNS 이름공간 수정
    * 웹 서버의 이름과 IP 주소를 리버스 프록시가 대신 사용 
4. 웹 서버 수정
    * 리버스 프록시를 사용하는 웹 서버가 305 redirect 를 내려 리버스 프록시로 다시 접근하게 함


## 클라이언트 프록시 설정
> 프록시가 트래픽을 처리하는 방법 > 클라이언트 수정
- 현대 브라우저는 다양하게 프록시 사용 설정 가능

1. 수동 설정 = 명시적 설정
    * ex. chrome: 설정 > 고급 > 시스템 > 컴퓨터 프록시 설정 열기 > 설정 창에서 프록시 서버, 포트 등 선택
    * 수동 설정은 하나의 서버만 지정 가능 & 장애시 대체 작동 지원 없음
2. 브라우저 기본 설정 = 배포자가 임의로 미리 설정 가능
3. 프록시 자동 설정
    * Proxy auto configuration (PAC)
    * 웹 브라우저 및 기타 사용자 에이전트가 자동으로 프록시 사용 여부와 어떤 프록시를 사용해야 하는지 판단하기 위해 실행하는 JS 파일
    * PAC은 `상황에 맞게` 유연하게 대처함. 수동 설정보다 동적인 해결책.
    * PAC 파일 URI를 브라우저에 설정 > 브라우저는 매 접근마다 적절한 프록시 서버 계산 > 스크립트의 반환값에 따라 연결
4. 웹 프록시 자동 발견
    * WPAD (Web Proxy Auto-Discovery)
    * PAC 파일을 다운 받을 수 있는 설정 서버를 자동으로 찾는 기법
        + 올바른 발견을 위해 성공할 때까지 여러 기법을 시도함. 
    * 대부분의 브라우저가 제공


## 프록시 요청의 미묘한 특징들

### 프록시 URI는 서버 URI와 다르다
- [클라이언트 -> 프록시] `완전한 URI`
    * 목적지 서버와의 커넥션을 위해 서버 이름을 알아야 함 (L7단)
    * 클라이언트가 명시적으로 프록시 사용 설정을 했다면 URI를 완전하게 보냄
- [클라이언트 -> 서버] 스킴, 호스트, 포트번호 없는 `부분 URI`
    * 패킷과 중복되므로 불필요한 정보 전송을 피함 

### 프록시는 프록시 요청과 서버 요청을 모두 다룰 수 있다
- 여러 웹서비스가 동일한 물리 서버를 사용하는 `가상 호스팅된 서버`에서도 부분 URI 문제가 있었는데, 이 경우 **Host 헤더**를 요구한다.
- 클라이언트가 명시적으로 프록시를 사용한다고 설정되어 있지 않아도, `대리 프록시와 인터셉트 프록시`는 지날 수 있다. 이런 프록시는 부분 URI를 받을 수 있다.
- 따라서 다목적 프록시 서버는 **완전한 URI와 부분 URI 모두를 지원**해야 한다.
- 부분 URI를 받은 프록시는 일단 Host 헤더를 확인하는데, 없다면...
    1. 자신 안에 주소+포트가 있는지 확인 (대리 프록시)
    2. 이전에 가로챈 트래픽에서 주소+포트 확인 (인터셉터 프록시)
    3. 에러 메시지 반환 (Host 없는 요청을 받은 가상 호스팅 서버처럼, 400을 내려줌. HTTP/1.1~ 모두 포함)

### 전송 중 URI 변경
- 별 거 아닌 것 같아도 상호운용성에 문제가 생길 수 있음
- 프록시는 가능한 한 관대해야 함
- 인터셉트 프록시는 공백->/ 을 제외하고 경로 변경 엄격히 금지

### URI 클라이언트 자동확장과 호스트명 분석
- 사용자가 입력한 걸로 호스트 발견이 안 되면, 약어가 입력된 것으로 보고 확장을 시도함
- 자동으로 홈페이지를 완성 / 오타 교정 사이트로 보냄 / 호스트 명의 앞부분으로 뒷부분 유추 시도
- (A) 프록시가 없는 URI 분석: 홈페이지 자동완성 지원 (yahoo -> www.yahoo.com)
- (b) 명시적인 프록시를 사용할 때 URI 분석: 확장 미지원. 목적지는 무조건 프록시를 향하기 때문.
- (C) 인터셉트 프록시를 이용한 URI 분석: 클라이언트가 목적지가 프록시인지 모르기 때문에 프록시가 만약 죽은 서버에 요청을 보내고 OK하면 서버와 커넥션이 잘 된 것으로 이해할 수 있음
- 경우별로 자동확장 과정에 도달하는 것 정리
    1. 약어로 DNS Lookup          ---------> A, C
    2. 실패한 뒤 확장해서 DNS Lookup ---------> A, C
    3. 목적지 서버와 커넥션 맺음       ---------> A, B(목적지가 프록시), C는 여기에서 실패 가능성 있음
        * B는 실패했고, C는 실패 가능성이 있으므로, 프록시는 호스트명을 다시 분석하거나, 역방향 DNS Lookup을 하는 등 Fault tolerant를 지원해야 함.


## 메시지 추적
- 둘 이상의 프록시가 트랜잭션 안에 있는 경우 흔함
- 성능 문제를 위해 캐시 저장고 사용
- 프록시를 개발하는 벤더가 많고, 각 프록시의 기능/버그가 상이하므로, 스위치&라우터 흐름 추적처럼 프록시 흐름 추적도 필요해짐

### Via 헤더
- 메시지가 지나는 중간 노드(프록시/게이트웨이)의 정보 나열
- 메시지가 노드를 지날 때마다 via 목록의 끝에 중간 노드가 추가됨 (쉼표로 구분)
- 요청의 via 헤더와 응답의 via 헤드는 보통 순서가 반대다
- 추가되는 정보: 프로토콜 이름(opt.), 프로토콜 버전, 노드 이름(주소), 코멘트(opt.)
- 용도
    * 메시지 전달 추적
    * 루프 진단
    * 트랜잭션에 관여하는 발송자들의 프로토콜을 다루는 능력 파악
    * 네트워크 라우팅 루프 탐지 (by 프록시): 자기를 상징하는 유일한 문자열을 포함시키고 요청 중 그 문자열이 있는지 검사
- 어떤 프록시는 게이트웨이 기능을 제공하는데, via 헤더는 프로토콜 기록도 남으므로 앱은 프록시 연쇄 중 변환이 있었는지 알 수 있다.
- server 헤더는 원 서버가 사용하는 소프트웨어를 알려주는데, 프록시는 이 헤더를 수정하면 안 된다.
- 프록시는 via에 가명을 추가할 수 있다.
- 같은 프로토콜, 같은 조직 통제하에 있고, 가명으로 교체되어 있는 경우에만 하나의 그룹으로 묶을 수 있다.
- 기본적으로는 모든 프록시는 via 헤더(경유지 목록)를 유지하고자 노력해야 한다.

### TRACE 메서드
- 프록시가 메시지를 수정할 수 있으므로 홉을 지나면서 어떻게 수정되는지 관찰할 수 있는 메서드
- 서버는 메시지가 도착하면 자신이 수신한 요청 메시지를 본문에 담아 다시 돌려보낸다.
- 지날 수 있는 최대 홉의 개수를 지정한 Max-Fowards 헤더를 사용할 수 있다.
    * 무한 루프 여부 등 프록시 연쇄 테스트
    * 연쇄 중간의 특정 프록시의 효과 체크
    * 반드시 지원해야 하는 필드 & 이번 홉을 지나갈 때 반드시 값 감소
- 널리 구현되어 있지 않다.


## 인증
- 프록시는 접근 제어 장치 기능 제공 가능
- 특정한 접근 자격이 없다면 407 + Proxy-Authenticate(자격 제출 방법) 응답


## 상호운용성
- 프록시는 클라이언트, 서버, 프록시 등등과 연결되며, 이들은 각기 다른 구현이 되어 있음에도, 이들을 중개해야 함

### 미지원 헤더와 메서드 다루기
- 이해할 수 없는 헤더는 그대로 전달
- 같은 이름의 헤더 필드 값의 순서 유지
- 미지원 메서드로 들어온 메시지는 최대한 다음 홉으로 전달하려 시도

### OPTIONS 메서드, Allow 헤더
- 보다 쉽게 상호작용할 수 있게 클라이언트는 어떤 기능을 지원하는지 OPTIONS 메서드로 질의
- 서버는 Allow 헤더를 통해 가능한 기능을 응답