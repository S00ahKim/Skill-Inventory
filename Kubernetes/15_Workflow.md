# 워크플로우 관리
> 워크플로우를 사용하면 컨테이너간 종속성을 부여하여 순차적으로 작업을 수행할 수 있다


## Argo Workflow
- 고급 스케줄링을 위한 프로젝트
    * 조건부/병렬 실행
    * 작업간 종속성
    * 에러 핸들링 등
- 웹 UI 제공
- 오퍼레이터 패턴
    * 사용자 정의 리소스 Workflow
    * 컨트롤러 argo-controller
- 장점
    * 높은 고립성: 실행의 단위가 컨테이너 레벨
    * 높은 재사용성: 하나의 역할만 담당하는 Job을 단일 개발 가능
- 단점
    * 작은 일을 처리하는 많은 Job을 생성할 경우 파드 생성/삭제 비용 때문에 성능 저하
    * 스텝마다 개별 컨테이너 실행되어 Job 간 데이터 공유 쉽지 않음 (use 볼륨!)
- 참고
    * 워크플로우 생성 시 매번 새로운 워크플로우를 만들어야 해서 create 을 사용 (not apply)
    * 컨테이너 실행 시 파라미터를 전달할 수 있음 (arguments)
    * 순차 실행 및 병렬 실행 가능 (steps)
    * 복잡한 DAG 실행 가능 (dag, dependencies)
    * 종료 핸들링으로 에러 발생시 노티 가능
- 사용예
    * 데이터 파이프라인
    * CI 파이프라인