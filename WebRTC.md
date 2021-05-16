# WebRTC (Web Real-Time Communication)
WebRTC API는 웹사이트 개발 시 사용할 수 있는 인터페이스의 하나인 WEB API(DOM 조작, 오디오 및 비디오 재생, 3D 그래픽 구현 등의 작업을 위해 사용) 중 하나다. 웹 애플리케이션과 사이트가 `중간자 없이 브라우저 간에` 오디오나 영상 미디어를 포착하고 마음대로 스트림할 뿐 아니라, 임의의 데이터도 교환할 수 있도록 하는 기술로, 플러그인이나 제 3자 소프트웨어 설치 없이 종단 간 데이터 공유와 화상 회의를 가능하게 한다.


## 통신 원리
[시그널링의 과정](images/webrtc00.png)

### P2P 절차
1. 각 브라우저가 P2P 커뮤니케이션에 동의
2. 서로의 주소를 공유 -----------------------> 브라우저는 웹 서버가 아니니까 외부 접근이 가능한 주소가 없다
3. 보안 사항 및 방화벽 우회              
4. 멀티미디어 데이터를 실시간으로 교환

### 실시간 데이터 교환을 돕는 3개의 클래스
1. MediaStream - 카메라/마이크 등 데이터 스트림 접근
2. RTCPeerConnection - 암호화 및 대역폭 관리 및 오디오 또는 비디오 연결.
      * `시그널링`: 이들이 데이터를 교환할 수 있게 처리하는 NAT 우회 과정. 즉 RTCPeerConnection 통신에 사용할 프로토콜, 채널, 미디어 코덱 및 형식, 데이터 전송 방법, 라우팅 정보와 NAT 통과 방법을 포함한 통신 규격을 교환하기 위해 두 장치의 제어 정보를 교환하는 과정. 
      * 연결을 요청한 쪽인 `콜러`, 연결을 받는 `콜리`가 있음. 이 둘의 통신을 위해 중간 역할의 서버로 SessionDescription을 주고받아야 함
3. RTCDataChannel - 일반적인 데이터 P2P통신


### References
- [MDN WebRTC API](https://developer.mozilla.org/ko/docs/Web/API/WebRTC_API)
- [[WebRTC] 웹브라우저로 화상 채팅을 만들 수 있다고?](https://velog.io/@ehdrms2034/WebRTC-%EC%9B%B9%EB%B8%8C%EB%9D%BC%EC%9A%B0%EC%A0%80%EB%A1%9C-%ED%99%94%EC%83%81-%EC%B1%84%ED%8C%85%EC%9D%84-%EB%A7%8C%EB%93%A4-%EC%88%98-%EC%9E%88%EB%8B%A4%EA%B3%A0)
- [WebRTC는 어떻게 실시간으로 데이터를 교환할 수 있을까?](https://wormwlrm.github.io/2021/01/24/Introducing-WebRTC.html)