# 사용자 정의 리소스


## Custom Resource
> 사용자가 직접 생성한 쿠버네티스의 새로운 리소스
```yaml
apiVersion: apiextensions.k8s.io/v1
kind: CustomResourceDefinition
metadata:
  name: crontabs.stable.example.com     # <NAMES>.<GROUP>
spec:
  group: stable.example.com             # apiVersion의 그룹 이름 (<GROUP>)
  version: v1                           # 사용자 리소스 버전
  scope: Namespaced                     # 클러스터 레벨 OR 네임스페이스 레벨
  names:                                # CRD 리소스의 이름
    plural: crontabs
    singular: crontab
    kind: CronTab
    shortNames:
    - ct

```
- 쿠버네티스 API를 사용자가 원하는대로 확장한 리소스
- 쿠버네티스는 사용자가 쉽게 API를 확장할 수 있도록 CustomResourceDefinition 이라는 리소스를 제공함
- 이 리소스를 정의하는 것만으로는 실제로 동작하지 않음. 어떻게 동작해야 하는지에 대한 정보가 없기 때문임.


## Custom Controller
> 커스텀 리소스를 컨트롤하는 컨트롤러
```python
"""
간단한 CronTab CRD를 감시하고, 
새로운 CronTab 리소스가 생성되면 
해당 정보를 기반으로 Kubernetes Job을 생성하며, 
CronTab 리소스가 삭제되면 관련 Job도 삭제
"""
from kubernetes import client, config, watch

# 설정 로드 및 API 클라이언트 초기화
config.load_kube_config()
crd_api = client.CustomObjectsApi()
core_api = client.CoreV1Api()

NAMESPACE = 'default'  

# Job 생성 함수
# CronTab 리소스의 정보를 사용하여 Kubernetes Job을 생성
def create_job(crontab):
    name = crontab['metadata']['name']
    image = crontab['spec']['image']
    replicas = crontab['spec']['replicas']

    job = client.V1Job(
        api_version="batch/v1",
        kind="Job",
        metadata=client.V1ObjectMeta(name=name),
        spec=client.V1JobSpec(
            template=client.V1PodTemplateSpec(
                spec=client.V1PodSpec(
                    containers=[
                        client.V1Container(
                            name=name,
                            image=image
                        )
                    ],
                    restart_policy='Never'
                )
            )
        )
    )

    core_api.create_namespaced_job(namespace=NAMESPACE, body=job)
    print(f"Job {name} created")

# Job 삭제 함수
# CronTab 리소스의 정보를 사용하여 관련 Job을 삭제
def delete_job(crontab):
    name = crontab['metadata']['name']
    core_api.delete_namespaced_job(name=name, namespace=NAMESPACE, body=client.V1DeleteOptions())
    print(f"Job {name} deleted")

# 주요 로직
# Kubernetes 이벤트 스트림을 감시하며, ADDED 및 DELETED 이벤트에 따라 Job을 생성하거나 삭제
def main():
    w = watch.Watch()
    for event in w.stream(crd_api.list_namespaced_custom_object, group="stable.example.com", version="v1", namespace=NAMESPACE, plural="crontabs"):
        crontab = event['object']
        event_type = event['type']

        if event_type == 'ADDED':
            print(f"New CronTab created: {crontab['metadata']['name']}")
            create_job(crontab)
        elif event_type == 'DELETED':
            print(f"CronTab deleted: {crontab['metadata']['name']}")
            delete_job(crontab)

if __name__ == '__main__':
    main()
```
- 컨트롤러를 만드는 것은 언어/동작 방식에 따라 다양함
- 예제와 같은 기본 로직 외에 에러 처리, 재시도 로직, 리소스 업데이트 처리 등 다양한 추가 기능이 필요할 수 있음


## Operator 패턴
> 커스텀 리소스와 그에 대응하는 커스텀 컨트롤러 조합으로 특정 애플리케이션/서비스의 생성과 삭제를 관리하는 패턴
- 쿠버네티스 코어 API에 포함되지 않은 애플리케이션을 마치 쿠버네티스 네이티브 리소스처럼 동작하게끔 만들 수 있음
- 동일한 애플리케이션을 여러 개 설치해야 하는 경우, 리소스 생성만으로 바로 애플리케이션을 사용할 수 있음
- 오퍼레이터 패턴 사용예
    * 상황: 신규 플젝을 시작할 때 CI, CD를 위한 젠킨스 애플리케이션을 매번 새로 생성하고, 완료 시점엔 삭제함
    * 사용예: 오퍼레이터 패턴으로 젠킨스라는 사용자 리소스를 생성하고 새로운 애플리케이션이 필요할 때마다 생성시킬 수 있음
- 오퍼레이터 툴
    * 쿠버네티스 API로 처음부터 개발할 수도 있지만, 오퍼레이터를 편리하게 만들 수 있게 제공하는 툴들이 존재함
    * ex. `kubebuilder`, `Operator Framework`, `Metacontroller`, `KUDO`
- 유용한 오퍼레이터들
    * 직접 개발할 수도 있지만, 대부분의 경우 helm chart 처럼 이미 잘 만들어진 오퍼레이터를 활용할 수 있음
    * ex. `MinIO Operator`: 오브젝트 스토리지 솔루션인 MinIO를 오퍼레이터로 생성할 수 있음. 여러 환경에 오브젝트 스토리지 서버를 제공하는 경우 유용함.
    * ex. `Prometheus Operator`: 프로메테우스와 그라파나를 설치할 수 있음.
    * ex. `Helm Operator`: 헬름 차트를 쿠버네티스 리소스처럼 관리할 수 있게 해주는 오퍼레이터. 리소스 생성시 차트 설치, 리소스 삭제시 차트 삭제.
        + 헬름 오퍼레이터에서 사용하는 사용자 정의 리소스를 HelmRelease 라고 부름
        + 헬름 차트를 선언형 YAML 정의서로 관리 가능함
    * 그 외 다양한 오퍼레이터를 https://operatorhub.io 에서 다운로드 가능