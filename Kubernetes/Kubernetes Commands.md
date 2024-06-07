# 쿠버네티스 명령어

## 기본
- 컨테이너 실행 `kubectl run {컨테이너(파드)_이름} --image {이미지_이름}`
    * 도커 명령어 `docker run`과 유사
- 컨테이너 조회 `kubectl get pods`
    * 쿠버네티스의 최소 실행 단위가 Pod(파드)이기 때문
    * 결과의 STATUS는 파드의 상태를 말함
        + Pending 쿠버네티스 마스터에 명령이 전달되었으나 실행되지 않음
        + ContainerCreating 노드에 스케줄링되어 컨테이너 생성 중
        + Running 파드가 정상적으로 실행 중
        + Completed 작업 후 종료되도록 정의된 파드가 정상 완료됨
        + Error 파드에 문제가 생겨 에러가 발생함
        + CrashLoopBackOff 지속적으로 에러 상태가 있어 크래시가 반복됨
    * 파드의 상태 정보를 자세히 보기 `kubecrl get pod {파드_이름} -o yaml`
    * 파드의 IP 확인 `kubectl get pod -o wide`
    * 도커 명령어 `docker ps`와 유사
- 컨테이너 상세 정보 확인 `kubectl describe pod {파드_이름}`
    * get과 유사하나, 파드의 이벤트 기록까지 확인 가능하여 디버깅 용도로 사용
    * 도커 명령어 `docker inspect`와 유사
- 컨테이너 로깅 `kubectl logs {파드_이름}`
    * `-f` 지속적으로 로그 출력하는 옵션
    * 도커 명령어 `docker logs`와 유사
- 컨테이너 명령 전달 `kubectl exec {파드_이름} -- {명령어}`
    * 실행 중인 컨테이너에 명령을 전달함
    * 명령은 `--` 구분자로 구분함
    * `-it` 컨테이너 내부 진입 가능한 옵션
    * 도커 명령어 `docker exec`과 유사
- 컨테이너/호스트 간 파일 복사 `kubectl cp {타깃_경로} {출처_경로}`
    * 도커 명령어 `docker cp`와 유사
- 컨테이너 정보 수정 `kubectl edit pod {파드_이름}`
    * 실행된 컨테이너의 정보 수정
    * 텍스트 에디터가 열리고, 여기서 수정하면 컨테이너에 반영됨
    * 도커 명령어 `docker update`와 유사
- 컨테이너 삭제 `kubectl delete pod {파드_이름}`
    * 도커 명령어 `docker rm`과 유사
- 선언형 명령 정의서 YAML 기반 컨테이너 생성 `kubectl apply -f {정의서_파일_이름}`
    * 컨테이너 이름과 이미지 주소 같은 기본적 정보만 작성해도 나머지는 디폴트 값으로 쿠버네티스가 채움
    * 명령형 스타일의 컨테이너 실행 커맨드가 run이라면, 선언형 스타일 커맨드가 apply
    * 대부분의 경우, 선언형 커맨드로 리소스 생성함
    * apply 명령어는 멱등성 (동일한 연산을 여러번 실행해도 최종 결과가 한번 실행한 것과 같음) 을 보장함

## 고급
- 기본에서 다룬 명령어의 pod 자리에 다른 리소스 이름을 넣으면 그 리소스를 제어할 수 있음
- 네임스페이스: 쿠버네티스 클러스터를 논리적으로 구분
    * default 기본 네임스페이스, 옵션 없이 컨테이너를 만들 때 생성되는 곳
        + `-n` 특정 네임스페이스에 리소스를 생성하는 옵션 ex. `kubectl run foo --image nginx --namespace bar-space`
    * kube-system 쿠버네티스의 핵심 컴포넌트가 들어있는 네임스페이스 ex. 네트워크 설정, DNS 서버 등
    * kube-public 외부로 공개 가능한 리소스를 담은 네임스페이스
    * kube-node-lease 노드가 살아있는지 마스터에 알리는 용도로 사용되는 네임스페이스
- 자동완성
    * [OS와 shell에 맞게 스크립트 세팅](https://kubernetes.io/docs/tasks/tools/install-kubectl-macos/#enable-shell-autocompletion)
    * 명령어를 입력하고 TAB 키를 눌러서 후보를 받을 수 있음
- 즉석 리소스 생성
    * 빠른 리소스 생성을 위해 YAML 정의서를 파일이 아닌 문자열로 넘기는 방식
    * ex. 
        ```bash
          cat << EOF | kubectl apply -f -
          apiVersion: v1
          kind: Pod
          metadata:
            name: cat-nginx
          spec:
            containers:
            - image: nginx
              name: cat-nginx
          EOF
          ```
- 리소스 특정 정보 추출
    * ex. `kubectl get node master -o jsonpath="{.status.address[0].address}"`
- 모든 리소스 조회 `kubectl api-resources`
    * NAMESPACED 칼럼은 이 리소스가 네임스페이스 레벨인지 여부를 뜻함
        + 네임스페이스 레벨 리소스 예: Pod
        + 클러스터 레벨 리소스 예: Node
    * 네임스페이스 레벨의 리소스만 탐색하기 위한 명령어 `kubectl api-resources --namespaced=true`
- 리소스 정의 설명 `kubectl explain pods`
- 클러스터 상태 확인 (health check)
    * 쿠버네티스 API 서버 작동 여부 확인 `kubectl cluster-info`
    * 전체 노드 상태 확인 `kubectl get node`
    * 쿠버네티스 핵심 컴포넌트의 파드 상태 확인 `kubectl get pod -n kube-system`
- 클라이언트 설정 파일 조회 `kubectl config {추가_명령어}`
    * kubectl은 내부적으로 KUBECONFIG 설정 파일을 참조하여 정보를 관리함
    * 설정값을 변경하려면 파일을 직접 수정하거나, kubectl config 명령어를 사용할 수 있음
    * 추가 명령어 예
        + `view` 클라이언트 설정 파일 값을 확인
    * KUBECONFIG 영역
        + clusters kubectl이 바라보는 클러스터 정보
        + users 쿠버네티스 클러스터에 접속하는 사용자
        + contexts 클러스터와 유저를 연결해주는 것
- cf. [치트시트](https://kubernetes.io/ko/docs/reference/kubectl/cheatsheet/)