# 클라우드 시스템
> 클라우드 시스템 ---(구현)--> 가상화 기술 ---(오케스트레이션 기술)--> 실제 서비스 운영



## 개념
- 클라우드란? 인터넷을 통해 액세스되는 서버와 해당 서버에서 실행되는 소프트웨어 및 데이터베이스
- 클라우드 서버는 전세계의 데이터센터에 있다.


## 특징
1. 자체적인 서버 업데이트 과정 또는 유지 관리가 불필요하다. 
2. 물리 서버를 직접 관리하지 않아도 된다.
3. 로컬 컴퓨터에서 직접 애플리케이션을 실행하지 않아도 된다. 


## 장점
> 비용, 시간 등 자원을 절약할 수 있다.
1. 자체 구축(On-premise)과 비교한 클라우드 서비스의 장점
- 온프레미스?
    * 기업의 서버를 클라우드 같은 원격 환경에서 운영하는 방식이 아닌, 자체적으로 보유한 전산실 서버에 직접 설치해 운영하는 방식
    * 클라우드 컴퓨팅 기술이 나오기 전까지 기업 인프라 구축의 일반적인 방식
    * 장점: 기업의 비즈니스 정보를 보안성 높게 관리할 수 있음
    * 단점: 많은 시간 & 많은 비용
- trade-off
    * 보안성 높은 데이터는 온프레미스 환경 & 보안성 낮은 데이터는 클라우드 환경 -> 하이브리드 IT 인프라

2. 웹 호스팅 VS 서버 호스팅 VS 클라우드
- 웹 호스팅
    * 호스팅 업체 서버 중 일부 공간을 임대하여 사용
    * 장점: 서버와 인프라를 구축할 필요가 없고 가격이 싸다.
    * 단점: 자원 사용량에 제한이 있으며, 서버 관리 권한이 없다.
    * 소규모 사이트 운영에 사용
- 서버 호스팅
    * 호스팅 업체의 물리 서버를 단독으로 임대 또는 구매하여 사용. 운영에 필요한 기술력과 인프라를 제공받는다. 
    * 장점: 관리의 직접 권한을 갖고 자원을 모두 활용할 수 있으며 단독으로 서버를 사용하므로 웹 호스팅에 비해 보안상 유리하다.
    * 단점: 초기 구축에 비용과 시간이 소모되며 가격이 다른 둘에 비해 비싸다. 
    * 고정적으로 대용량 트래픽과 DB 사용량이 발생하는 보안이 중요한 곳에서 사용
- 클라우드
    * 호스팅 업체의 가상 서버를 단독으로 사용한다.
    * 장점:
        + 단 몇 분 만에 서버를 생성하고 바로 사용할 수 있다. 
        + 서버 관리에 대한 직접 권한이 있고 필요할 때 서버를 확장하거나 축소할 수 있는 등 서버 스펙을 마음대로 조정할 수 있다.
        + 이용한 만큼만 과금되어 경제적이다. 
    * 단점: 하나의 프로그램에 이상이 생기면 다른 곳에도 영향을 받는다. -> 이중화, 백업으로 커버할 수 있음
    * 게임, 이벤트, 개발 테스트 등 접속자가 갑자기 늘었다 줄었다 하는 유동적인 서비스를 운영할 때 사용한다.


## 종류
1. Public & Private
    * 퍼블릭 클라우드
        + 불특정 다수의 일반 대상. (인터넷에 접속 가능한 모든 사용자들을 대상)
        + 인터넷으로 서비스함
        + 유지 비용 없이 사용료만 지불하면 된다.
        + 오피스, 메일 등 개인용 애플리케이션 또는 중소형 비즈니스 서비스에 적합하다.
    * 프라이빗 클라우드
        + 기업과 같은 한정된 유저 대상(제한된 네트워크 상에서 특정 기업이나 특정 사용자만을 대상)
        + 개인 사용자 인증 + VPN 등으로 네트워크 인증도 필요로 함
        + 데이터를 보호할 수 있고 개별 애플리케이션의 개발과 서비스가 가능하다. 
        + ERP와 같은 기업의 핵심 애플리케이션 또는 중대형 비즈니스 서비스에 적합하다.
2. SPI 모델
    * SaaS (Software as a Service)
        + 계정형, 구독형 서비스
        + Google Drive, Office365 등
    * PaaS (Platform as a Service)
        + IasS + 필요한 소프트웨어 (OS, JAVA 등 런타임, Spring 등 플랫폼)
        + 기본으로 제공하는 서비스: 장애 복구, 스케줄링, 로드밸런싱, 확장성, 고가용성, 미터링, 모니터링
        + 아마존 AWS EMR
        + 오픈소스: VMWare의 <CloudFoundry>, Redhat의 <OpenShift>
    * IaaS (Infrastructure as a Service)
        + 서버 자원 (CPU, 메모리, 디스크, 네트워크 등)
        + 아마존 AWS EC2
        + 오픈소스: <KVM>을 하이퍼바이저로 제공하는 <오픈 스택>