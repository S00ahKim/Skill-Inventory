# 부하 분산(로드 밸런싱, Load Balancing)
- 정의: 여러 대의 서버들로 대규모 네트워크 트래픽을 분산 처리하는 기술
- 역할: 한 곳의 End Point로 들어오는 트래픽을 각각의 컴퓨팅 장치에 분산시킨다.


## 왜?
- 한 대의 서버로 감당하기 어려운 트래픽이 발생할 경우, Scale-up 방식에 한계가 있기 때문
- Scale-up? 한 대의 서버의 성능을 높이는 것
- Sclae-out? 여러 대의 서버를 추가하여 시스템을 확장하는 것


## 기본 기능
1. Health Check
2. Tunneling
3. NAT (Network Addressing Translation)
4. DSR (Direct Server Routing)


## L4 (Classic Load Balancer)
- 계층적 특성? 라우터, 스위치 등 물리적인 하드웨어 영역을 다루므로, 데이터를 변경하거나 수정할 수 없음.
- 알고리즘
    * 라운드 로빈
    * 가중치/비율 할당
    * 최소 연결 기반
    * 응답 시간 기반
    * 해시 기반
    * 대역폭 기반


## L7 (Application Load Balancer)
- 계층적 특성? 응용 계층이기 때문에 포트나 헤더를 수정할 수 있음.
- 알고리즘
    * URL 스위칭
    * 컨텍스트 스위칭
    * 쿠키 지속성


## DNS (Domain Name Server)


## GSLB (Global Server Load Balancing)


## 성능 지표
1. 초당 연결 수 (connection per second)
2. 동시 연결 수 (concurrent connections)
3. 처리용량 (throughput)


## 기타 용어 정리
- load balancer? 로드밸런싱 기술을 제공하는 서비스/장치. 클라이언트와 서버풀/네트워크허브 사이에 존재함.
- server pool? 동일한 응용 프로그램을 구성하고 실행하는 하나 이상의 서버 군.
- network hub? 여러 대의 네트워크 장비를 연결하는 장치. 같은 허브 내의 장비는 상호간 통신이 가능함.


## 참고자료
- [Steven J. Lee - 네트워크의 부하분산, 로드밸런싱 그리고 로드밸런서](https://www.stevenjlee.net/2020/06/30/%ec%9d%b4%ed%95%b4%ed%95%98%ea%b8%b0-%eb%84%a4%ed%8a%b8%ec%9b%8c%ed%81%ac%ec%9d%98-%eb%b6%80%ed%95%98%eb%b6%84%ec%82%b0-%eb%a1%9c%eb%93%9c%eb%b0%b8%eb%9f%b0%ec%8b%b1-load-balancing-%ea%b7%b8/)
- [VMWare Docs - 로드밸런싱을 위한 서버 풀 추가](https://docs.vmware.com/kr/VMware-NSX-T-Data-Center/2.3/com.vmware.nsxt.admin.doc/GUID-E60ACBCF-C8C4-462A-9167-9BF71194FC9C.html)