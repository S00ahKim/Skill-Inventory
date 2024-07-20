# 컨트롤러
> 쿠버네티스의 특정 리소스를 지속적으로 바라보며 리소스 생명주기에 따라 미리 정해진 작업을 수행하는 주체
> - contol-loop를 돌면서 관찰하고, desired state가 업데이트되면 current state를 그와 일치시킨다.

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
    * ex. 카프카, 레디스 클러스터, mySQL 클러스터 등
- 파드마다 고유한 식별자가 존재하며, 고유한 데이터를 보관함
- 파드가 삭제된 경우 다른 파드로 쉽게 교체하지 못함
- 생성된 이름에 무작위 문자열이 붙지 않고 순서가 붙고, 호스트명과 볼륨도 마찬가지
- 레플리카 개수를 줄일 경우, 식별자 역순으로 파드가 삭제됨