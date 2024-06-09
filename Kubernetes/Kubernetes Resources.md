# 쿠버네티스 리소스
- 쿠버네티스 리소스: 시스템 내에서 관리되는 모든 엔티티 (추상적인 개념, 오브젝트 정의에 사용되는 개념)
  - 종류가 매우 많아 자주 사용되는 것만 메모함 (모두 보기: `kubectl api-resources`)
  - 작은 단위의 리소스를 조합하여 큰 단위 리소스를 만듦 (컨테이너 > 파드 > 서비스, 디플로이먼트 > 애플리케이션)
  - [Pod](#pod)
  - [ConfigMap](#configmap)
  - [Secret](#secret)
  - [Service](#service)
  - [ReplicaSet](#replicaset)
  - [Job](#job)
  - [CronJob](#cronjob)
  - [DaemonSet](#daemonset)
  - [StatefulSet](#statefulset)
- 쿠버네티스 오브젝트: 특정 리소스를 기반으로 생성된 인스턴스 (실제로 클러스터 내에서 관리되고 동작하는 구체적인 개체)
- 쿠버네티스 컴포넌트: 시스템 안에서 운영을 담당하는 다양한 오브젝트
  - ex. 노드가 배정되지 않은 새로 생성된 파드를 감지하고, 실행할 노드를 선택하는 컨트롤 플레인 컴포넌트인 kube-scheduler는 Pod 오브젝트

## Pod
- 쿠버네티스의 최소 실행 단위
    * 프로세스의 크기나 복잡도와 무관
    * 쿠버네티스는 파드로 기본 가상환경을 제공함
- 특징
    * 1개 이상의 컨테이너 (대개 최대 3개 정도)
    * 동일 파드의 컨테이너는 동일 노드에 할당되며 동일 생명주기를 가짐
    * 파드 리소스는 노드IP와 별개로 고유한 파드IP를 할당받으며, 파드 안 컨테이너끼리 IP를 공유함
        + 쿠버네티스에서는 다른 노드에 있는 파드와도 별도 설정 없이 파드 고유 IP를 이용해 접근 가능
            - 별도 설정? NAT(Network Address Translation) 통신
            - NAT? 여러 개의 내부 IP를 1개의 외부 IP와 연결하는 기술 ex. 포트 포워딩 
        + ex. 도커 컨테이너가 다른 노드의 컨테이너와 통신하는 방법
            1. 노드IP와 포트 포워딩시켜둠
            2. 포워딩 포트를 이용하여 타 노드의 컨테이너와 통신
        + ex. 도커 컨테이너가 같은 파드의 다른 컨테이너와 통신하는 방법
            1. localhost를 통해 네트워크 접근
            2. 서로 다른 포트로 각자를 구분함
        + 다른 클러스터에 있는 리소스와 통신하려면 별도 설정 필요 (게이트웨이 등)
    * 파드 리소스는 동일한 볼륨과 연결할 수 있고, 파일 시스템을 기반으로 파일 주고받기 가능
- 필수 프로퍼티
    ```yaml
    apiVersion: v1              # 리소스의 이름이 동일할 경우 충돌을 피하기 위한 목적으로 정의한 스코프
    kind: Pod                   # 리소스 타입
    metadata:                   # 메타 정보
        name: nginx             # 리소스 이름
        labels:                 # 리소스 라벨 정보 
            run: nginx
    spec:                       # 리소스 스펙
        containers:             # 1개 이상의 컨테이너를 정의
        - name: nginx           # 컨테이너 이름
          image: nginx:1.14.2   # 컨테이너 이미지 주소
          ports:
          - containerPort: 80
    ```
- 라벨링 시스템
    * 사용예: 특정 리소스 참조, 파드에 트래픽 전달, ...
    * 라벨 부여 = key,value 형태의 문자열이 파드의 메타데이터 프로퍼티에 추가
        + `kubectl label pod {파드_이름} {KEY}={VALUE}`
        + YAML 파일에 작성 (복수개 가능, labels 하위에 키:값 형태로 기술)
        + `kubectl run {파드_이름} --image {이미지_이름}` 이 경우, run={파드_이름} 라벨이 자동 추가됨
    * 라벨 확인 
        + `-L` 파드에 부여된 라벨을 확인하기 위한 옵션 ex. `kubectl get pod {파드_이름} -L {KEY}`
        + `--show-labels` 특정 라벨 말고 전체 라벨 확인
    * 라벨 조건 필터링
        + `-l` 특정키, 특정키의 특정값 을 갖는 파드를 필터링 가능 ex. `kubectl get -l foo=bar`
    * nodeSelector를 이용하여 파드가 특정 노드에 할당되게 스케줄링
        1. 노드에 기본적으로 설정된 라벨들이 있는데, 여기에 라벨을 추가로 부여한다. 부여 방법은 파드와 같다.
            ex. `kubectl label node worker foo=bar`
        2. 실행하려는 파드 YAML 파일에 nodeSelector 프로퍼티 추가
            ```yaml
            (...)
            spec:
                containers:            
                - name: nginx          
                  image: nginx:1.14.2 
                nodeSelector:
                  foo: bar
            ```
        3. 파드가 실행되는 노드를 확인해본다
            ex. `kubectl get pod pod_name -o wide`
        - cf. 만약 2개 이상의 노드에 동일한 라벨이 부여되어 있는 경우, 쿠버네티스가 노드 상태를 확인하여 최적의 노드를 선택한다
- 실행 명령, 파라미터 지정
    * YAML 파일의 컨테이너 정의 하위에 정의할 수 있음
      ```yaml
      (...)
      spec:
        restartPolicy: OnFailure    # 파드 재시작 정책 / Always(디폴트, 종료시 항상 재시작), Never(시도안함), OnFailure(실패시에만)
        containers:
        - name: nginx
          image: nginx:1.14.2
          command: ["/bin/echo"]    # 도커의 ENTRYPOINT에 대응 / echo가 실행 후 바로 종료되어 재시작 정책을 OnFailure로 설정한 것
          args: ["hello"]           # 도커의 CMD에 대응
      ```
- 환경변수 설정
    * YAML 파일의 컨테이너 정의 하위에 정의할 수 있음
      ```yaml
      (...)
      spec:
        containers:
        - name: mycontainer1
          image: myimage1
          env:                      # 각 컨테이너는 자신만의 환경 변수 공간을 가진다
          - name: ENV_VAR1
            value: "value1"
      ``` 
- 볼륨 연결
    * 파드 내부 스토리지의 생명주기는 파드와 동일하여 파드가 사라지면 저장된 데이터도 삭제됨
    * 데이터를 지속적으로 저장하려면 볼륨을 연결해야 함
    * ex. 호스트 서버의 볼륨 공간을 마운트
      ```yaml
      (...)
      spec:
        containers:
        - name: mycontainer1
          image: myimage1
          volumeMounts:                         # 컨테이너 내부의 연결 위치 지정
          - name: shared-data                   # 볼륨을 연결하는 식별자
            mountPath: /usr/share/nginx/html    # 컨테이너 내부에 볼륨이 연결될 위치
        volumes:                                # 파드에서 사용할 볼륨을 지정
        - name: shared-data
          hostPath:                             # 호스트 서버의 연결 위치 지정
            path: /home
      ```
    * hostPath 외에 파드 내에서 임시로 생성하는 `emptyDir` 프로퍼티도 있다.
        + 컨테이너끼리 파일을 주고받을 때 자주 사용한다.
        + 파드의 생명주기를 따라가는 임시 볼륨이라 파드가 사라지면 삭제되는 데이터이나 2개 이상 컨테이너가 서로 공유 가능한 디렉터리 공간
- 컴퓨팅 자원 관리
    * 프로퍼티 이름: `resources`
      ```yaml
      (...)
      spec:
        containers:
        - name: resource-demo-container
          image: nginx
          resources:
            requests:                       # 최소 자원 사용량 보장
              memory: "64Mi"                # cf. Mi = MiB = 메비바이트(2^20 바이트) / 메가바이트(10^6 바이트)
              cpu: "250m"                   # cf. 1000m = 1core
            limits:                         # 최대 자원 사용량 제한
              memory: "128Mi"
              cpu: "500m"
      ```
    * 컨테이너가 최대 사용량을 넘기면 CPU는 쓰로틀링(성능다운/강제중단) 발생, 메모리는 OOM 에러 발생
- 헬스 체크
    * YAML 파일의 컨테이너 정의 하위에 정의할 수 있음
    * 컨테이너가 살아있는지 체크할 목적의 프로퍼티 이름: `livenessProbe`
        ```yaml
        (...)
        livenessProbe:
            httpGet:            # HTTP GET 메서드로 헬스 체크
                path: /live     # 체크할 경로
                port: 80        # HTTP 포트
        ``` 
        + 정상 동작 확인
        + 자가치유를 위한 판단 기준으로 활용
        + httpGet은 200~300번대 응답을 정상으로 판단, 그 외의 경우 컨테이너 종료 후 재실행
        + kubectl get pod 명령의 응답값에서 RESTARTS 수를 모니터링하면서 잘 동작하는지 확인 가능
    * 트래픽을 받을 준비가 되었는지 체크할 목적의 프로퍼티 이름: `readinessProbe`   
        ```yaml
        (...)
        readinessProbe:
            httpGet:
                path: /ready
                port: 80
        ```
        + ex. 초기 구동에 시간이 오래 걸리는 젠킨스 서버 등
        + 초기화가 완료됨을 쿠버네티스에 알리는 용도
        + kubectl get pod 명령의 응답값에서 READY 수를 모니터링하면서 잘 동작하는지 확인 가능
    * httpGet 외에도 특정 명령이 성공(응답=0)하는지 확인할 수 있는 프로퍼티 존재함 `exec`
        ```yaml
        readinessProbe:
            exec:
                command:
                - cat
                - /tmp/ready
        ```
- 2개 이상의 컨테이너
    * YAML 파일의 컨테이너 정의 하위에 생성하려는 컨테이너 스펙을 나열하면 됨
    * 2개 이상의 컨테이너가 있으면 `logs` 같은 커맨드 실행시 무슨 컨테이너에 대한 명령인지 `-c` 옵션으로 명시해야 함
    * 2개 이상의 컨테이너를 실행하는 이유에는 여러가지가 있는데, 대표적으로 **사이드카 패턴**을 사용하기 위함이 있다. 메인 컨테이너를 보조하는 컨테이너(로그 전송 컨테이너 등)를 사이드카라고 부른다.
    * 쿠버네티스는 기본적으로 파드 내부 컨테이너끼리 실행 순서를 보장하지 않음
- 초기화 컨테이너
    * 메인 컨테이너 실행에 앞서 초기화를 위해 먼저 실행되는 컨테이너를 정의하는 프로퍼티 이름: `initContainers`
        + 메인 컨테이너가 실행되기 전에 초기화가 필요할 경우 사용
    * YAML 파일의 컨테이너 정의 동위에 정의할 수 있음
        ```yaml
        containers:
        (...)
        initContainers:
        - name: git
          image: alpine/git
        (...)
        ```
- Downward API
    * 메타데이터를 컨테이너에게 전달할 수 있는 매커니즘
    * 실행되는 파드의 정보를 컨테이너에 노출하고 싶을 때 사용함
    * 환경변수/볼륨 연결을 통해 컨테이너에 정보 전달 가능
        + (1) 환경변수
            ```yaml
            containers:
            - name: foo
              image: bar
              env:
              - name: NODE_NAME
                valueFrom:
                  fieldRef: 
                    fieldPath: spec.nodeName
              - name: POD_NAME
                valueFrom:
                  fieldRef: 
                    fieldPath: metadata.name
              - name: POD_NAMESPACE
                valueFrom:
                  fieldRef: 
                    fieldPath: metadata.namespace
              - name: POD_IP
                valueFrom:
                  fieldRef: 
                    fieldPath: status.podIP
             ```
        + (2) 볼륨 연결
            ```yaml
            volumes:
            - name: podinfo
            downwardAPI:
              items:                          # 메타데이터로 사용할 아이템 리스트 지정
              - path: "labels"                # 볼륨과 연결될 컨테이너 내부 경로
              fieldRef:                     
                fieldPath: metadata.labels    # 파드의 메타데이터 필드 경로
            ``` 

## ConfigMap
- 메타데이터(설정값)을 따로 모아두는 리소스
- 파드에 직접 설정값을 기입할 수도 있지만, ConfigMap에 저장된 설정값을 파드에서 불러와서 사용 가능
- 생성
    * (1) `kubectl create configmap {컨피그맵_이름} {data_source}`
        - data_source 옵션(1) `--from-file` 작성해둔 프로퍼티 파일을 컨피그맵_이름 이라는 이름의 컨피그맵으로 만듦
        - data_source 옵션(2) `--from-literal` 사용자가 커맨드 뒤에 설정값을 쭉 입력함 ex. `kubectl create configmap ex-config --from-literal=foo=bar --from-literal=foo2=bar2`
    * (2) YAML 파일 작성 후 생성
        - `kind: ConfigMap`
          ```yaml
          apiVersion: v1
          kind: ConfigMap
          metadata:
            name: my-configmap
          data:
            key1: value1
            key2: value2
          ```
- 파드에서 활용
    * 볼륨 연결: 컨피그맵을 볼륨으로 마운트해서 파일처럼 사용
        ```yaml
        volumes:
        - name: ex-volume
          configMap:
            name: ex-config
        ``` 
    * 컨피그맵의 특정 값을 환경변수로 사용
        ```yaml
        containers:
        - name: ex-env
          image: foo/bar
          env:
          - name: foo-env
            valueFrom:
              configMapKeyRef:
                name: ex-config
                key: ex.ample
        ```
    * 컨피그맵의 모든 값을 환경변수로 사용
        ```yaml
        containers:
        - name: ex-env
          image: foo/bar
          envFrom:
          - name: foo-env
            valueFrom:
              configMapRef:
                name: ex-config
        ```

## Secret
- 컨피그맵과 유사하지만, 민감 데이터를 저장하는 데 사용
- 강한 보안: 노드에서 사용될 때 디스크에 저장되지 않고, tmpfs라는 메모리 기반 파일시스템 사용
- 사용자 조회 시 인코딩된 값 노출: 평문으로 바로 보이지 않고 base64 인코딩된 값. 암호화는 아님.
- 생성
    * (1) `kubectl create secret {시크릿_이름} {data_source}`
        - 데이터소스 옵션은 컨피그맵과 마찬가지로 `--from-env-file`, `--from-file`, `--from-literal` 등을 지원
    * (2) YAML 파일 작성 후 생성
        - `kind: Secret`
          ```yaml
          apiVersion: v1
          kind: Secret
          metadata:
            name: mysecret
          type: Opaque
          data:
            username: YWRtaW4=       # "admin"을 base64로 인코딩한 값
            password: cGFzc3dvcmQ=   # "password"을 base64로 인코딩한 값
          ```
        - type은 시크릿의 타입을 설정하는 프로퍼티. 필요할 경우 다른 값을 사용할 수 있다.
        - 특정한 문자열을 base64로 인코딩한 값을 얻는 커맨드 `echo -ne {문자열} | base64`
        - 인코딩해서 넣지 않고 쿠버네티스가 인코딩하기 원한다면 `data` -> `stringData` & 원본 문자열을 값으로 입력
- 파드에서 활용
    * 볼륨 연결: 시크릿을 볼륨으로 마운트해서 파일처럼 사용
        ```yaml
        volumes:
        - name: ex-volume
          secret:
            secretName: ex-secret
        ``` 
    * 시크릿의 특정 값을 환경변수로 사용
        ```yaml
        containers:
        - name: ex-env
          image: foo/bar
          env:
          - name: foo-env
            valueFrom:
              secretKeyRef:
                name: ex-secret
                key: example
        ```
    * 시크릿의 모든 값을 환경변수로 사용
        ```yaml
        containers:
        - name: ex-env
          image: foo/bar
          envFrom:
          - name: foo-env
            valueFrom:
              secretRef:
                name: ex-secret
        ```

## Service
- 쿠버네티스의 네트워크 담당
- 파드 IP가 부여되는데 왜 필요한가? 안정성 때문
    * 파드 = 불안정한ephemeral 자원 / 언제든지 종료될 수 있음 / 엔드포인트가 불안정하고 계속 추적해야함
    * 서비스 = 파드 생명주기와 무관한 엔드포인트 제공 / 파드 앞단에서 트래픽을 파드로 전달하는 리버스 프록시 역할
        + cf. 프록시 = 클라숨김, 리버스 프록시 = 서버숨김
- 생성
    ```yaml
    apiVersion: v1
    kind: Service
    metadata:
      labels:
        hello: world
      name: ex-svc
    spec:
      ports:                # 서비스의 포트들
      - port: 8080          # 서비스로 들어오는 포트
        protocol: TCP       # 사용하는 프로토콜 (ex. TCP, UDP, HTTP)
        targetPort: 80      # 트래픽을 전달할 컨테이너의 포트
      selector:             # 트래픽을 전달할 컨테이너의 라벨
        run: ex-nginx
    ```
- 서비스 탐색
    * 서비스 리소스의 이름 기반 DNS 참조 가능
        + 이름 법칙: `{서비스_이름}.{네임스페이스}.svc.cluster.local`
        + svc.cluster.local 부분은 쿠버네티스 클러스터 도메인으로, 변경할 수 있지만 보통 기본값으로 사용함
        + 클러스터 도메인 주소는 생략 가능하며, 생략된 부분은 디폴트로 채움
    * 라벨 셀렉터를 이용하여 파드를 선택
        + 파드의 이름을 직접 참조할 경우, 파드 생명주기에 따라 매번 서비스 내용이 변경되어야 함
        + 라벨링으로 간접 참조(느슨한 연결)할 경우, 그 라벨을 가진 어떤 파드에든 트래픽 전달 가능하므로 서비스를 매번 변경해줄 필요 없음
    * 탐색 순서
        1. 도메인 호출
        2. DNS Resolver가 도메인에 맞는 클러스터IP 호출 (= kube-dns 서비스의 IP)
        3. kube-dns가 coredns를 사용하여 요청된 도메인에 맞는 IP 주소 얻음
        4. 사용자에게 응답이 돌아감
    * 모든 파드들은 클러스터 내부, 외부 DNS 질의를 coredns를 통해 수행함
- 서비스 타입
    * YAML 파일의 spec 하위 프로퍼티
    * ClusterIP
        + 디폴트
        + 이 타입의 서비스 엔드포인트는 클러스터 내부에서만 접근 가능
        + 네트워크 보안 및 관리를 위해 직접 트래픽을 받는 엔드포인트 최소화
        + 다른 복잡한 네트워크 기능의 뼈대가 되어줌
    * NodePort
        + 도커 컨테이너 포트 매핑과 유사
        + 호스트(노드)의 특정 포트 - 서비스의 특정 포트 연결
        + 노드포트는 모든 노드에서 동일한 엔드포인트 제공함 (단, externalTrafficPolicy 옵션이 Cluster인 경우)
        + 외부 트래픽을 서비스까지 전달할 수 있음
    * LoadBalancer
        + 노드 앞단에 로드밸런서를 두고 각 노드로 트래픽을 분산할 수 있게 함
        + 퍼블릭 클라우드 플랫폼이 제공하는 로드밸런서를 연결할 수 있음 ex. ELB
        + NodePort 타입은 노드포트 대역을 열어두고 이를 외부에 공개하는 방법이라면, LoadBalancer 타입은 서버를 내부 네트워크에 두고 로드밸런서만 외부 네트워크에 엔드포인트(웰노운포트)를 제공하는 식으로 열어두어 네트워크 보안성을 높임
        + 로드밸런서가 클러스터 앞단에 있으면 사용자는 각각의 서버IP를 몰라도 로드밸런서의 IP/도메인 주소만 갖고도 요청을 보낼 수 있음 (쿠버네티스는 노드 역시 자유롭게 종료시킬 수 있어서 그때마다 노드IP 추적은 번거로움)
            - 즉 ClusterIp가 파드 레벨에서 안정적 엔드포인트를 제공한다면, LoadBalancer 타입 서비스는 노드 레벨에서 안정적 엔드포인트를 제공
    * ExternalName
        ```yaml
        apiVersion: v1
        kind: Service
        metadata:
          name: my-external-service
          namespace: default
        spec:
          type: ExternalName
          externalName: external.example.com
        ``` 
        + 외부 DNS 주소에 대해 클러스터 내부에서 사용할 새로운 별칭을 만듦
        + 위 예시의 경우, external.example.com 에 default 라는 별칭으로 접근 가능
        + 쿠버네티스 클러스터에 편입되지 않는 외부 서비스에 쿠버네티스 네트워킹 기능을 연결하고 싶을 때 사용
- 쿠버네티스 네트워크 모델의 특징
    * 모든 리소스가 다른 모든 리소스를 (ex. 노드-노드, 파드-파드, 노드-파드) NAT 없이 고유 IP로 통신이 가능하다
        + cf. 쿠버네티스 전신인 Borg 프로젝트는 NAT를 사용했는데 포트 충돌이 빈번해서 번거로웠기 때문에 독립적 네트워크 환경을 구성함
    * 각 파드는 고유 IP를 할당받는다
    * 파드IP에 클러스터 내부 어디서든 접근 가능하다 (에이전트도 마찬가지) 호스트 서버와의 종속성이 없다

## ReplicaSet
```yaml
apiVersion: apps/v1
kind: ReplicaSet
metadata:
  name: nginx-replicaset
  labels:
    app: nginx
spec:
  replicas: 3                   # 복제해서 유지할 파드의 개수
  selector:
    matchLabels:                # 복제 개수를 유지할 파드의 라벨
      app: nginx
  template:                     # 복제할 파드 정의 (라벨 붙은 파드의 metadata, spec과 동일)
    metadata:
      labels:
        app: nginx
    spec:
      containers:
      - name: nginx
        image: nginx:latest
        ports:
        - containerPort: 80
```
- 파드의 고가용성 보장을 위해 복제를 수행하는 컨트롤러
- `kubectl get replicaset` 으로 상태 조회 가능 (DESIRED, CURRENT, READY, AGE)
- 레플리카셋이 복제를 담당하기는 하지만 실제 프로세스 실행은 역할/책임 범위를 벗어나기 때문에 파드 리소스를 활용함
- **라벨은 여러개 지정 가능하지만, 대상 파드 정의는 1개여야 함**

## Deployment
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: nginx-deployment
  labels:
    app: nginx
spec:
  replicas: 3                   # 유지할 파드 개수
  selector:
    matchLabels:                # 배포할 파드의 라벨
      app: nginx
  strategy:
    type: RollingUpdate         # 배포 전략
    rollingUpdate:
      maxUnavailable: 25%       # 최대 중단 파드 허용 개수/비율
      maxSurge: 25%             # 최대 초과 파드 허용 개수/비율
  template:                     # 복제할 파드 정의
    metadata:
      labels:
        app: nginx
    spec:
      containers:
      - name: nginx
        image: nginx:latest
        ports:
        - containerPort: 80
```
- 리소스 생성 `kubectl apply --record -f ex-dp.yaml`
    * `--record` 롤백시 롤아웃 히스토리에서 사용한 커맨드를 볼 수 있게 하는 옵션
- 애플리케이션 업데이트 및 배포 관련 기능 제공
    * 롤링 업데이트 `설정에 따라 자동 적용`
    * 롤백
        + `kubectl rollout undo deployment {디플로이먼트_이름}` 바로 이전 버전으로 롤백
        + `--to-revision` 옵션으로 특정 배포 버전으로도 롤백 가능
    * 스케일 아웃 지원 `설정에 따라 자동 적용`
    * 배포 상태 확인
- 배포 전략
    * 롤링 업데이트: 점진적 업데이트 (대부분의 경우)
    * 리크리에이트: 일시적으로 전체 파드 삭제 후 새 파드 전체 생성
- 디플로이먼트는 복제를 위해 레플리카셋을 만들고, 그 레플리카셋이 파드 복제본을 생성한다
    * 디플로이먼트: 배포
    * 레플리카셋: 복제
    * 파드: 컨테이너 실행

## Job
```yaml
apiVersion: batch/v1
kind: Job
metadata:
  name: example-job
spec:
  template:
    metadata:
      name: example-job
    spec:
      containers:
      - name: example
        image: busybox
        command: ["sh", "-c", "echo Hello, Kubernetes! && sleep 30"]
      restartPolicy: OnFailure
    backoffLimit: 2
```
- 한번 실행하고 완료되는 일괄처리 프로세스 전용
- 잡도 결국 내부적으로는 파드를 통해 실행하므로, template은 파드 spec과 동일
- `backoffLimit` 재시도 횟수
    * 2로 설정된다면 총 3회 실행됨 (1트 + 설정된 만큼 트)
    * 재시도 횟수가 설정되어 있는 경우, 잡이 멱등성을 보장하는지 확인 필요
- 일반적인 파드는 Running 상태로 지속되나, 잡 리소스의 파드는 Completed 상태로 종료됨

## CronJob
```yaml
apiVersion: batch/v1
kind: CronJob
metadata:
  name: example-cronjob
spec:
  schedule: "*/1 * * * *"               # 잡을 실행할 주기 설정
  jobTemplate:                          # 잡의 스펙을 동일하게 사용
    spec:
      template:
        spec:
          containers:
          - name: example
            image: busybox
            command: ["sh", "-c", "echo Hello, Kubernetes! && sleep 30"]
          restartPolicy: OnFailure
```
- 잡과 유사하지만 주기적으로 실행되어야 하는 경우
- cron 형식으로 스케줄링 가능

## DaemonSet
```yaml
apiVersion: apps/v1
kind: DaemonSet
metadata:
  name: nginx-daemonset
  labels:
    app: nginx
spec:
  selector:
    matchLabels:
      name: nginx
  template:
    metadata:
      labels:
        name: nginx
    spec:
      containers:
      - name: nginx
        image: nginx:latest
        ports:
        - containerPort: 80

```
- 모든 노드에 동일한 파드를 실행시키려는 경우 사용함
- 사용예: 리소스 모니터링, 로그 수집기 등 모든 노드에 대한 정보를 추출할 때
- 클러스터에 신규 노드가 추가될 때 따로 작업하지 않아도 필요한 파드가 생성됨

## StatefulSet
```yaml
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: nginx-statefulset
spec:
  serviceName: "nginx"                          # 스테이트풀셋과 연결할 서비스 이름
  replicas: 3
  selector:
    matchLabels:                                # 스테이트풀하게 관리할 파드의 라벨
      app: nginx
  template:                                     # 복제할 파드의 정의
    metadata:
      labels:
        app: nginx
    spec:
      containers:
      - name: nginx
        image: nginx:latest
        ports:
        - containerPort: 80
        volumeMounts:
        - name: nginx-persistent-storage
          mountPath: /usr/share/nginx/html
  volumeClaimTemplates:                         # 동적으로 볼륨을 생성하는 프로퍼티
  - metadata:
      name: nginx-persistent-storage
    spec:
      accessModes: ["ReadWriteOnce"]
      resources:
        requests:
          storage: 1Gi
```
- Stateful한 파드를 생성해야 하는 경우 사용
    * 디플로이먼트/레플리카셋은 복제된 파드가 완벽히 동일함
    * 스테이트풀셋은 동일한 이미지로 파드를 생성하지만, 각 파드가 **실행 시 순서에 따라 각자 다른 역할을 가지며 그 역할을 교체하지 못함**
- 사용예
    * 실행 순서에 따라 마스터/워커가 결정되는 클러스터
    * 리더 선출이 필요한 클러스터
    * 프라이머리-레플리카 구조 클러스터
    * 파드마다 저장소가 다르게 지정되어야 하는 경우
    * 순서대로 업데이트가 필요한 애플리케이션
- 파드마다 고유한 식별자가 존재하며, 고유한 데이터를 보관함
- 파드가 삭제된 경우 다른 파드로 쉽게 교체하지 못함
- 생성된 이름에 무작위 문자열이 붙지 않고 순서가 붙고, 호스트명과 볼륨도 마찬가지
- 레플리카 개수를 줄일 경우, 식별자 역순으로 파드가 삭제됨