# WebRTC (Web Real-Time Communication)
WebRTC API는 웹사이트 개발 시 사용할 수 있는 인터페이스의 하나인 WEB API(DOM 조작, 오디오 및 비디오 재생, 3D 그래픽 구현 등의 작업을 위해 사용) 중 하나다. 웹 애플리케이션과 사이트가 `중간자 없이 브라우저 간에` 오디오나 영상 미디어를 포착하고 마음대로 스트림할 뿐 아니라, 임의의 데이터도 교환할 수 있도록 하는 기술로, 플러그인이나 제 3자 소프트웨어 설치 없이 종단 간 데이터 공유와 화상 회의를 가능하게 한다.


## 특징
- SDP format (세션 기술 프로토콜) 에 맞추어 영상, 음성 데이터를 교환
- 서버 없이 클라이언트-클라이언트 간 p2p 영상/음성/데이터 통신에 활용 가능
- 오픈 프로젝트. 구글 크롬 주도 + 모질라, 오페라 지원
- WebRTC API 최신 스펙 공개 -> 크롬에 구현하여 릴리즈


## 주의
- 피어간의 연결이 끊겼을 때, 적절한 retry 로직으로 다시 연결을 맺어줘야 함
- 안정적인 Signaling Server 를 구축해야 함
- 다양한 플랫폼에서 표준화가 완전히 구현되지 않음. 각 브라우저의 WebRTC API에는 moz, webkit 같은 벤더 프리픽스(vendor prefix)가 붙어있음.
    * 단일 브라우저 벤더에서 제공하는 API가 아니며, 브라우저와 운영체제별로 개발되는 속도와 지원되는 버전이 다름
    * 크로스 브라우징 이슈(브라우저 간 호환성 문제): adapter.js 라이브러리 사용


## 통신 원리
[시그널링의 과정](images/webrtc00.png)

### P2P 절차
1. 각 브라우저가 P2P 커뮤니케이션에 동의
2. 서로의 주소를 공유 -----------------------> 브라우저는 웹 서버가 아니니까 외부 접근이 가능한 주소가 없다
3. 보안 사항 및 방화벽 우회 ------------------> 개개인의 컴퓨터는 방화벽 등 여러 보호 장치가 있다
      * Stun/Turn Server: Peer 간 연결이 쉽지 않은데, 연결을 위한 정보를 공유하여 P2P 통신을 만들게 해 줌
          + `STUN`(Session Traversal Utilities for NAT)
              - NAT 내부에서 피어 간 연결을 도움
              - P2P IP 연결을 위한 정보 제공 (불가능할 경우, TURN)
              - 퍼블릭한 관점에서 엔드포인트에 액세스할 수 있는 IP:port를 발견함
              - NAT/방화벽 뒤 엔드포인트 판단 + 엔드포인트의 공인 아이피와 NAT/방화벽 유형에 대해 알림
          + `TURN`(Traversal Using Relays around NAT)
              - Peer 간 미디어 스트리밍을 릴레이(중계)하기 위해 사용
              - Peer 간 직접 통신이 실패할 경우 데이터 릴레이를 수행함
      * `Ice`(Interactive Connectivity Establishment)
          + NAT환경에서 자신의 Public IP를 파악하고 상대방에게 데이터를 전송하기 위한 Peer간의 응답 프로토콜. 
          + 클라이언트의 모든 통신 가능한 주소를 식별하며, 이를 위해 로컬 주소, STUN이 파악한 클라의 공인망 주소인 server reflexive 주소, TURN이 패킷 릴레이를 위해 할당하는 relayed 주소를 사용한다.
          + 일반적으로 Stun/Turn을 사용해서 구축. 
          + 한쪽이 offer를 보내면 다른 쪽이 answer해서 연결이 설정됨
      * `NAT`: 외부망과 분리하여 공인망, 내부망의 IP:port를 매핑해줌           
4. 멀티미디어 데이터를 실시간으로 교환

### WebRTC 애플리케이션 작업 절차
1. Fetching
      * 상대 peer 에게 보낼 사용자의 음성 및 영상 데이터를 수집 
      * with: MediaStream, getUserMedia
2. Signaling
      * 이 세상 어딘가에 있는 상대 peer 와 연결을 맺기 위해서, 상대 peer 의 정보를 탐색
      * 피어가 서로를 찾기 위한 중간 매개자인 시그널링 서버가 필요 없음 (구현 방법 다양)
      * with RTCPeerConnection, setRemoteDescription
      * 시그널링의 단계
          1. 네트워크 정보 교환 (ICE 프레임워크로 후보 ip:port 찾음) 
          2. Media Capability 를 교환 (offer <-> answer)
          3. Session Control Messages 교환
3. Connection
      * 발견한 peer 와 p2p connection 을 맺습니다. channel 을 개방
4. Communication
      * 개방해놓은 채널을 통해 음성/영상/텍스트 데이터를 주고 받음


## 실시간 데이터 교환을 돕는 3개의 클래스
1. MediaStream
      * 카메라/마이크 등 데이터 스트림 접근
      * 애플리케이션이 사용자의 음성, 영상 데이터 수집 
2. RTCPeerConnection
      * 암호화 및 대역폭 관리 및 오디오 또는 비디오 연결
      * 애플리케이션이 수집한 음성, 영상 데이터를 주고받는 채널 추상화
      * `시그널링`: 이들이 데이터를 교환할 수 있게 처리하는 NAT 우회 과정. 즉 RTCPeerConnection 통신에 사용할 프로토콜, 채널, 미디어 코덱 및 형식, 데이터 전송 방법, 라우팅 정보와 NAT 통과 방법을 포함한 통신 규격을 교환하기 위해 두 장치의 제어 정보를 교환하는 과정. 
      * 연결을 요청한 쪽인 `콜러`, 연결을 받는 `콜리`가 있음. 이 둘의 통신을 위해 중간 역할의 서버로 SessionDescription을 주고받아야 함
3. RTCDataChannel
      * 일반적인 데이터 P2P통신
      * 음성 및 영상 데이터가 아닌 json, text 데이터를 주고받는 채널 추상화


### References
- [MDN WebRTC API](https://developer.mozilla.org/ko/docs/Web/API/WebRTC_API)
- [Get Started with WebRTC](https://www.html5rocks.com/ko/tutorials/webrtc/basics/)
- [[WebRTC] 웹브라우저로 화상 채팅을 만들 수 있다고?](https://velog.io/@ehdrms2034/WebRTC-%EC%9B%B9%EB%B8%8C%EB%9D%BC%EC%9A%B0%EC%A0%80%EB%A1%9C-%ED%99%94%EC%83%81-%EC%B1%84%ED%8C%85%EC%9D%84-%EB%A7%8C%EB%93%A4-%EC%88%98-%EC%9E%88%EB%8B%A4%EA%B3%A0)
- [WebRTC는 어떻게 실시간으로 데이터를 교환할 수 있을까?](https://wormwlrm.github.io/2021/01/24/Introducing-WebRTC.html)
- [WebRTC - STUN과 TURN에 대하여](https://alnova2.tistory.com/1110)
- [webRTC 정리](http://jaynewho.com/post/36)