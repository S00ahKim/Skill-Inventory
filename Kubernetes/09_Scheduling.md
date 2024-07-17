# 스케줄링

## 고가용성 확보: 파드 레벨
- `HorizontalPodAutoScaler`(hpa) 파드 리소스 사용량에 따라 자동으로 **수평 확장**하는 리소스
- metrics-server라는 컴포넌트를 사용하여 파드의 작업량을 모니터링하다가 임계값을 넘으면 replica 수를 동적으로 증/감함
    * metrics-server? 파드의 리소스 사용량을 수집하는 서버 / 설치 후 `kubectl top {node|pod}` 로 리소스 확인 가능
- hpa 생성
    1. 선언형: yaml 정의서 작성 후 apply
          + hpa가 정상적으로 동작하려면 모니터링 대상의 Deployment.yaml에 기준점을 정의한 `requests` 프로퍼티가 정의되어 있어야 함 
    2. 명령형: ex. `kubectl autoscale deployment heavy-cal --cpu-percent=50 --min=1 --max=50`


## 고가용성 확보: 노드 레벨
- 노드에 리소스가 부족해지는 경우, 노드 레벨 수평 확장을 통해 클러스터 레벨의 고가용성을 보장 가능
- 단, 클러스터 오토스케일링은 클라우드 서비스의 지원이 필요함
- ex. AWS EKS Cluster AutoScaler
    * 오토스케일러 차트를 설치
    * 직접 설치하므로 약간 수고스럽지만, 자동 확장 성책을 세밀하게 조정 가능
- ex. GCP GKE Cluster AutoScaler
    * 오토스케일링 옵션을 켜고 클러스터 생성
    * 간편하게 자동 확장 가능하지만, 상세한 정책을 설정하기 어려움
- 파드 오토스케일러는 실제 자원 사용량을 판단 기준으로 사용하지만, 클러스터 오토스케일러는 자원 요청량을 판단 기준으로 사용하므로 유의


## Taint & Toleration
- 특정 노드에 대해 보수 작업을 하거나 네트워크 이상이 생긴다거나 하는 경우 노드에 taint 처리를 하면 그 상태를 감안하는 파드에만 스케줄링함
- 두 설정 모두 1개 이상 적용 가능
- Taint
    * 노드에 적용하는 설정값
    * 이 노드가 오염taint되었으니 파드에게 다가오지 말라not schedule고 알림 (나에게 오점이 있으니 감안하라)
    * effect 종류에 따라 스케줄링 정책이 달라짐
        + PreferNotSchedule 새로운 파드를 스케줄링하는 것을 지양하라
        + NoSchedule 새로운 파드를 스케줄링하는 것을 금지하라
        + NoExecute 실행 중인 파드도 삭제하라
- Toleration
    * 파드에 적용하는 설정값
    * 노드가 오염taint되었어도 파드가 견딜tolerate 수 있음을 표시 (오점을 견딜 수 있으니 나를 실행하라)
    * 만약 어떤 Taint가 설정된 노드여도 해당 Taint의 effect에 대한 Toleration이 적용된 파드는 스케줄링 가능
        ```yaml
        (...)
        spec:
          containers:
          - name: nginx
            image: nginx
          toleration:
          - key: "project"          # key가 project고
            value: "A"              # value가 A로
            operator: "Equal"       # 일치하며
            effect: "NoSchedule"    # effect가 NoSchedule로 taint된 노드에 대해 tolerate하다
        ```
    * taint를 견딜 수 있단 의미지 항상 그 노드에 할당되는 것은 아니다


## Affinity & AntiAffinity
- 특정 노드나 파드의 거리를 조절하는 데 사용
- Affinity
    * NodeAffinity 특정 노드와 가까이 할당되기를 원할 때
        + nodeSelector 보다 더 복잡한 라벨링 선택 매커니즘을 제공
        + ex. 노드에 스케줄링되기를 희망/강제, 값의 equals 매치/존재/비존재/Gt/Lt etc.
    * PodAffinity 특정 파드와 가까이 할당되기를 원할 때
        + 여러 파드에 동일한 PodAffinity를 설정하는 식으로 작동
- AntiAffinity
    * PodAntiAffinity 특정 파드끼리 멀리 할당되기를 원할 때
- 활용법
    * PodAntiAffinity 고가용성을 위해 노드가 죽더라도 다른 노드에서 서비스를 지속할 수 있도록 웹서버를 다른 노드에 스케줄링
    * PodAffinity 네트워크 레이턴시를 최소화하기 위해 캐시 서버와 웹서버를 같은 노드에 스케줄링