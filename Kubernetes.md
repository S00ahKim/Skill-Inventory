# Kubernetes
구글에서 만들어서 리눅스 재단에 기부
CNCF라는 리눅스재단 산하 단체에서 관리

## Docker Orchestration
여러 호스트에서 다수의 애플리케이션을 운영, 관리

## NCC
- 쿠버네티스 기반 컨테이너 클러스터
- 파스타에서 신청 -> 네임스페이스 할당 -> 앱 올려서 사용
- 공용 클러스터 사용, 엔시시 풀에 들어와서 사용 ㅇㅇ

## 쿠버네티스 오브젝트
- 기본적으로 선언적으로 동작 (선언적? 자세한 일을 기술하기보다 추상적 메서드로 작성)
- 오브젝트 생성, 업데이트, 삭제 등 사용자가 요청하면 시스템이 그에 맞춰 시스템에 반영하게끔 동작
- 장점? 이식성 있는 클라우드 플랫폼
  * 한군데에서 잘되면 오브젝트 덤프해서 다른 클러스터에서 또 구현가넝
- 기본적인 오브젝트 = pod
- yaml, json으로 스펙 정의 -> 쿠베시티엘 커맨드로 만듦

## Pod
- 컨테이너를 팟이란 단위로 묶어서 관리, 배포함
- 하나의 팟 내 컨테이너는 네트워크, 디스크 볼륨 공유
- 레이블: 이름표. 오브젝트들 간에 (서비스, 디플로이 같은 옵젝) 식별가능하게 -> 키밸류로 정의
- 스펙: 컨테이너
- pod ip: vip (ncc에서는 외부접속 가능한 아이피 할당하게함) 클러스터상에서만쓰이는 아이피 할당

## ReplicaSet
- 보통 하나의 노드에서 소프트웨어를 설치해서 하는 경우는 드물고 여러 레플리케이션을 띄우는데, 동일한 이미지 갖는 여러 팟을 동시에 띄우고 관리하는 게 리플리카셋
- 팟을 정의해서 생성하는 게 아니라 리플리카셋을 정의해서 생성하면 템플릿 안에 정의된 컨테이너들을 리프리카 개수만큼 띄우는 방식으로 띄움

## Deployment
- 리플리카셋을 직접 정의 ㄴㄴ 보통은 이걸 사용해서 팟을 띄움
- 디플로이먼트는 배포하는 오브젝트 -> 생성하면: 리플리카셋을 내부적으로 생성해서 걔네가 팟을 띄운다.
- 팟을 구동할 때 디플로이먼트라는 오브젝트를 생성하면됨다.
- 롤링 업데이트: 자동으로 업뎃버전 만들어두고 기존거 죽이고 띄우고 죽이고 띄우고 함. (선언적 동작의 장점)
- 버전 업뎃 (갈아치우기용)

## Service
- 디플로이로 띄우면 사용자한테 접근시켜야되ㅏㄴ는데 그걸 도와줌
- 어떤 팟들을 서비스에 우선으로 넣을것인가 -> selector에다가 레이블 이름 씀
- 이 레이블로 뜬 팟들을 내가 서비스할게. 로드밸런싱할게
- 팟은 쿠버가 언제 죽이고 띄울지 모르고 그때마다 아이피가 달라질수있음 그러니까 레이블 통해서 사용자한테 열어줌 그럼 서비스가 알아서 레이블에 맞는 팟들에 부하 넣어줌.
- 기본적으로 엘포 로드밸런싱 제공
- 가능한 서비스 방식 4개.

## StatefulSet
- 스테이트풀한 어플리케이션 관리
- 디플로이랑 같은 레벨의 옵젝인데, 디플로이로 뜬 팟은 아이디 없고 다 같음. 이걸 사용해서 뜬 애들은 아이디 잇음. ex. DB 샤딩 - 컨테이너별 연결 스토리지 정해짐
- 딴데서도 1번으로 떠서 1번 스토리지랑 연결해야 되는! 그런애들 띄울때.
- 카프카, 레디스 클러스터, mySQL 클러스터 등이 이걸로 떠야 함
- 팟 구동되지만 아이디 잇고, 구동도 순서에 따라, 죽을때도 역순으로 종료

## ConfigMap
- 개발망에서 어플리케이션 개발 막 끝냈고 리얼망으로 옮겨서 테스트하고 싶은데 같은 이미지 사용하면 되는데 컨피그는 바껴야댐 접속 디비 정보나 에퍄 키 같은 거.
- 컨피그맵은 이 옵젝 적용하면 바꾸니느 환경에 적응되게 설정파일 수정가능.
- 환경변수나 설정파일로 설정가능

## Secret
- ConfigMap과 비슷한데, 비밀번호나 토큰, 키 같은 민감한 데이터를 저장한다