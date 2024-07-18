# 클러스터 관리


## 리소스 관리
- X 쿠버네티스 리소스, O 노드의 CPU, 메모리 리소스
- 가상의 논리 클러스터인 네임스페이스를 이용하여 리소스 관리함
- 클러스터 관리자가 일반 사용자의 리소스 사용량을 제한하는 방법

### LimitRange
```yaml
apiVersion: v1
kind: LimitRange
metadata:
  name: example-limitrange
  namespace: default
spec:
  limits:
  - max:                # 일반 사용자가 요청할 수 있는 최대치
      cpu: "2"
      memory: "4Gi"
    min:                # 일반 사용자가 요청할 수 있는 최소치
      cpu: "100m"
      memory: "256Mi"
    default:            # 생략시 설정되는 기본 리밋 설정값
      cpu: "500m"
      memory: "1Gi"
    defaultRequest:     # 생략시 설정되는 기본 request 설정값
      cpu: "200m"
      memory: "512Mi"
    type: Container
```
- 개별 파드 생성에 관여
    * 일반 사용자가 리소스 사용량 정의를 생략하더라도 자동으로 파드 리소스 사용량을 설정함
        + 생략할 경우 원래 무제한으로 사용 가능
    * 관리자가 설정한 최대 요청량을 일반 사용자가 넘지 못하게 제한함
- 관리자가 만든 LimitRange를 초과하는 파드 생성 요청이 되면 에러가 발생함

### ResourceQuota
```yaml
apiVersion: v1
kind: ResourceQuota
metadata:
  name: example-resourcequota
  namespace: default
spec:
  hard:
    pods: "10"
    requests.cpu: "4"               # 네임스페이스의 CPU request 총합
    requests.memory: "8Gi"          # 네임스페이스의 메모리 request 총합
    limits.cpu: "10"                # 네임스페이스의 CPU limit 총합
    limits.memory: "16Gi"           # 네임스페이스의 메모리 limit 총합
    persistentvolumeclaims: "5"
    requests.storage: "100Gi"
    configmaps: "10"
    services: "10"
    services.nodeports: "5"
    secrets: "10"
    replicationcontrollers: "10"
    resourcequotas: "1"
```
- 전체 네임스페이스에 대한 제약
- LimitRange만 사용하면 파드를 여러개 사용하는 과정에서 클러스터 리소스를 소진시킬 우려가 있기 때문
- 관리자가 만든 ResourceQuota를 초과하면 파드가 생성되지 않고 에러가 발생함


## 노드 관리
- 노드 자체에 대한 관리를 위해 일시적으로 중단해야 하는 경우 있음
    * 온프레미스의 경우: 물리 디스크 손상, 내부 네트워크 장애 등
    * 클라우드의 경우: 서버 타입 변경, 디스크 교체 등
- 쿠버네티스가 지원하는 특정 노드를 유지보수 상태로 전환하여 신규 파드 스케줄링을 막는 방법
    * Cordon
    * Uncordon
    * Drain

### Cordon
- 노드를 유지보수 모드로 전환
- `kubectl cordon {NODE}`
- 해당 노드에 스케줄링하게 강제된 파드의 경우에는 Pending으로 남아있음

### Uncordon
- 유지보수가 완료된 노드를 다시 정상화
- `kubectl uncordon {NODE}`
- 해당 노드에 신규 파드가 스케줄링 가능한 상태가 되어 이 노드에 대해 Pending이던 파드가 있다면 다시 스케줄링됨

### Drain
- 노드를 유지보수 모드로 전환 & 기존 파드를 퇴거evict 시킴
- cordon은 기존에 돌고 있는 파드에는 관여하지 않으나, drain은 기존에 도는 파드도 퇴거시킴
- cf. 디버깅 툴: tmux, screen 등 terminal multiplexer
- drain 된 노드도 uncordon으로 되돌릴 수 있음
- drain 될 때 급 종료를 막는 방법 = PodDisruptionBudget(pdb)
    * 운영 중인 파드 개수를 항상 일정 수준으로 유지할 수 있도록 파드의 퇴거를 막는 역할을 함
    * 장애 상황이 아니라 관리자가 일부러 파드를 내릴 때 일정 개수 이하로 내려가지 않게 해서 응답 불가 상황을 피하여 안전하게 보수할 수 있게 함
    * ex. 10개 중 9개를 유지해야 한다면 1개씩 evict 되면서 다른 노드로 옮겨감