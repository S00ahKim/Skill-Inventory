# Helm


## 개념
- 왜 사용하나?
    * 다양한 리소스의 조합으로 구성된 애플리케이션을 묶어서 배포하기 위해
        + 리소스를 개별적으로 챙기는 번거로움을 덜고, 여러 리소스를 한번에 추가 및 업데이트 가능
    * 다른 사람이 만든 애플리케이션을 쉽게 나의 클러스터에 가져올 수 있음
        + 도커 = 프로세스 레벨에서 외부 정의서 활용 가능케 함
        + 헬름 = 쿠버네티스 리소스(프로세스pod, 네트워크service, 저장소persistentVolume 등) 레벨에서 외부 정의서 활용 가능케 함
- 헬름이란 뭔가?
    * 쿠버네티스 클러스터에 원하는 패키지를 설치 및 관리하도록 지원하는 **패키지 매니저**
    * 헬름은 쿠버네티스에 차트를 설치하고, 각 차트의 릴리즈를 생성한다. 차트 리포지토리에 저장/검색이 가능하다.
- 구성?
    * 헬름 패키지 = YAML 형식으로 구성된 `chart`
    * chart = `values.yaml` `template/` 디렉터리
        + values.yaml: 사용자가 원하는 값을 설정하는 파일
        + template/: 설치할 리소스 파일들이 존재하는 디렉터리 (쿠버네티스 리소스가 YAML 파일 형태로 들어있음, 설정값은 placeholder로 비워져 있음)


## 주요 개념 3가지
1. 차트 (Chart)
    * 쿠버네티스 애플리케이션의 인스턴스를 생성하는 데에 필요한 정보의 꾸러미
    * 쿠버네티스 클러스터 내에서 애플리케이션, 도구, 서비스를 구동하는데 필요한 모든 리소스 정의가 포함된 헬름 패키지
    * 쿠버네티스에서의 Homebrew 포뮬러, Apt dpkg, YUM RPM 파일과 같음
2. 저장소 (Repository)
    * 차트를 모아두고 공유하는 장소
    * Perl의 CPAN 아카이브나 페도라 패키지 데이터베이스와 같음
3. 릴리스 (Release)
    * 쿠버네티스 클러스터에서 구동되는 차트의 인스턴스
    * 일반적으로 하나의 차트는 동일한 클러스터내에 여러 번 설치될 수 있음. 설치될 때마다 새로운 release가 만들어짐
    * 릴리스 가능한 객체를 생성하기 위해 패키징된 차트와 특정 config를 결합


## 기본 커맨드
1. `helm create {릴리스_이름}`
    * `Chart.yaml`, `charts`, `template`, `values.yaml` 이 생성됨
    * Chart.yaml: 차트 이름, 버전 등 기본 정보를 담음
    * charts: 차트 속에 다른 차트를 넣으려고 할 때 사용 가능. 기본적으로 빈 디렉토리
    * template: 차트의 뼈대가 되는 쿠버네티스 리소스를 담은 디렉토리 (내부 파일에 `{{ .Values.service.type }}` 과 같이 플레이스홀더가 있음)
    * values.yaml: 사용자가 정의하는 설정값을 담음
2. `helm install {릴리스_이름} {차트_경로}`
    * 릴리스_이름 으로 경로에 해당하는 패키지를 설치함
    * 라이브러리 종속성은 컨테이너 안에서 해결되므로, 실제 사용할 프로세스만 생성함
    * 이 커맨드로 만들어진 리소스를 조회해보면, values.yaml에서 정의한대로 생성된 것을 확인할 수 있음
3. `helm list`
    * 설치된 헬름 차트 조회
    * `-n` 옵션: 네임스페이스별로 조회 가능 
4. `helm template {릴리스_이름} {차트_경로} > {렌더링된_파일이름}.yaml`
    * values.yaml 의 값이 매핑된 템플릿 파일 정의서를 조회 (=렌더링, kubectl --dry-run 과 유사함)
    * `helm install foo bar` = `helm template foo bar > bar-mapping.yaml && kubectl apply -f bar-mapping.yaml`
    * 설치하기 전에 어떻게 만들어질지 디버깅하는 용도로 사용 가능
5. `helm upgrade {릴리스_이름} {차트_경로}`
    * 이미 설치한 차트에 대해 values.yaml 값을 수정 및 업데이트
    * REVISION 값이 증가함
6. `helm rollback {릴리스_이름} {돌아가려는_리비전_번호}`
    * 이전 릴리스로 간단히 롤백
7. `helm status {릴리스_이름}`
    * 배포된 차트의 상태 조회
8. `helm delete {릴리스_이름}` 또는 `helm uninstall {릴리스_이름}`
    * 사용하지 않는 차트를 삭제 (릴리스는 삭제되나, 히스토리는 남아있음)
    * Helm2에서는 delete를 사용함
    * Helm3에서는 uninstall을 사용함
    * `--purge` 옵션: 히스토리까지 삭제해서 롤백이 불가능하게 만듦


## 원격 리포지토리
- repository = 차트 원격 저장소
1. `helm repo add {저장소_이름} {저장소_주소}`
    * 저장소_이름 을 가진 리포지토리 추가
2. `helm repo update`
    * 추가한 리포지토리의 인덱스 정보를 최신으로 업데이트
3. `helm repo list`
    * 현재 등록된 리포지토리 리스트 조회
4. `helm search repo {저장소_이름}`
    * 특정 저장소에 저장된 차트 리스트 조회
    * 검색할 때 단어 또는 문구의 일부분만 입력해도 된다.
    * [helm 허브](https://hub.helm.sh/charts)에서 다양한 리포지토리 조회 가능
5. `helm install {릴리스_이름} {저장소_이름}/{저장소에_있는_차트_이름}`
    * 특정 원격 리포지토리에 있는 차트를 설치
    * 설치하면서 어떤 리소스가 생성되는지, 릴리스의 상태는 어떤지, 추가 설정단계가 있는지에 관한 정보 출력함
    * `helm show values` 커맨드: 차트의 옵션 구성 조회
    * `--version` 옵션: 차트 버전 지정
    * `--set` 옵션: values.yaml 값을 동적으로 설정
    * `--namespace` 옵션: 차트가 설치될 네임스페이스 지정
    * `--generate-name` 옵션: 헬름이 생성해주는 이름을 그대로 사용할 경우
6. `helm fetch --untar {저장소_이름}/{차트_이름}`
    * 특정 원격 리포지토리에 있는 차트를 로컬에 설치 (세부 설정 수정 등을 위함)
    * 차트가 tar 로 묶인 상태로 저장되어 `--untar` 옵션 붙인 것
    * `--version` 옵션: 특정 버전의 차트를 다운로드


## 디렉터리 구조
```
wordpress/
  Chart.yaml          # 차트에 대한 정보를 가진 YAML 파일
  LICENSE             # 옵션: 차트의 라이센스 정보를 가진 텍스트 파일
  README.md           # 옵션: README 파일
  values.yaml         # 차트에 대한 기본 환경설정 값들
  values.schema.json  # 옵션: values.yaml 파일의 구조를 제약하는 JSON 파일
  charts/             # 이 차트에 종속된 차트들을 포함하는 디렉터리
  crds/               # 커스텀 자원에 대한 정의 (helm3~)
  templates/          # values와 결합될 때, 유효한 쿠버네티스 manifest 파일들이 생성될 템플릿들의 디렉터리
  templates/NOTES.txt # 옵션: 간단한 사용법을 포함하는 텍스트 파일
```

## Chart.yaml 파일
```yaml
apiVersion: 차트 API 버전 (필수)
name: 차트명 (필수)
version: SemVer 2 버전 (필수)
kubeVersion: 호환되는 쿠버네티스 버전의 SemVer 범위 (선택)
description: 이 프로젝트에 대한 간략한 설명 (선택)
type: 차트 타입 (선택)
keywords:
  - 이 프로젝트에 대한 키워드 리스트 (선택)
home: 프로젝트 홈페이지의 URL (선택)
sources:
  - 이 프로젝트의 소스코드 URL 리스트 (선택)
dependencies: # 차트 필요조건들의 리스트 (optional)
  - name: 차트명 (nginx)
    version: 차트의 버전 ("1.2.3")
    repository: 저장소 URL ("https://example.com/charts") 또는 ("@repo-name")
    condition: (선택) 차트들의 활성/비활성을 결정하는 boolean 값을 만드는 yaml 경로 (예시: subchart1.enabled)
    tags: # (선택)
      - 활성화 / 비활성을 함께하기 위해 차트들을 그룹화 할 수 있는 태그들
    enabled: (선택) 차트가 로드될수 있는지 결정하는 boolean
    import-values: # (선택)
      - ImportValues 는 가져올 상위 키에 대한 소스 값의 맵핑을 보유한다. 각 항목은 문자열이거나 하위 / 상위 하위 목록 항목 쌍일 수 있다.
    alias: (선택) 차트에 대한 별명으로 사용된다. 같은 차트를 여러번 추가해야할때 유용하다.
maintainers: # (선택)
  - name: maintainer들의 이름 (각 maintainer마다 필수)
    email: maintainer들의 email (각 maintainer마다 선택)
    url: maintainer에 대한 URL (각 maintainer마다 선택)
icon: 아이콘으로 사용될 SVG나 PNG 이미지 URL (선택)
appVersion: 이 앱의 버전 (선택). SemVer인 필요는 없다.
deprecated: 차트의 deprecated 여부 (선택, boolean)
annotations:
  example: 키로 매핑된 주석들의 리스트 (선택).
```
- 모든 차트는 버전 번호를 가져야 함
- 헬름 v2 이상은 버전 번호를 release 마커로 사용 & 저장소에 있는 패키지들은 이름과 버전으로 구분
- 패키지를 생성할 때, helm package 명령은 Chart.yaml 에서 찾은 버전을 패키지 이름에 있는 토큰으로 사용하며, 시스템은 차트 패키지명 안의 버전 번호가 Chart.yaml 안의 버전 번호와 일치한다고 가정하고, 아닐 경우 에러 발생함


## template
```yaml
apiVersion: v1
kind: ReplicationController
metadata:
  name: deis-database
  namespace: deis
  labels:
    app.kubernetes.io/managed-by: deis
spec:
  replicas: 1
  selector:
    app.kubernetes.io/name: deis-database
  template:
    metadata:
      labels:
        app.kubernetes.io/name: deis-database
    spec:
      serviceAccount: deis-database
      containers:
        - name: deis-database
          image: {{ .Values.imageRegistry }}/postgres:{{ .Values.dockerTag }}
          imagePullPolicy: {{ .Values.pullPolicy }}
          ports:
            - containerPort: 5432
          env:
            - name: DATABASE_STORAGE
              value: {{ default "minio" .Values.storage }}
```
- [Go 템플릿 작성에 대한 표준 규약](https://golang.org/pkg/text/template/) 준수
- values.yaml 에 구체적인 템플릿 값이 정의됨
    * values.yaml 파일 (혹은 --set 플래그)로 제공되는 값은 템플릿 안에서 .Values 객체로 접근 가능
    * 모든 템플릿에서 접근 가능하며, 덮어쓸수 없는 미리 정의된 값 (대소문자 구별)
        1. Release.Name: 릴리즈의 이름(차트의 이름 아님)
        2. Release.Namespace: 차트가 릴리즈된 네임스페이스
        3. Release.Service: 릴리즈를 수행하는 서비스
        4. Release.IsUpgrade: 현재 업그레이드나 롤백중이면 true로 설정된다.
        5. Release.IsInstall: 현재 설치중이면 true로 설정된다.
        6. Chart: Chart.yaml의 내용. 차트의 버전은 Chart.Version으로, 메인테이너는 Chart.Maintainers로 얻을 수 있다.
        7. Files: 차트에 모든 특별하지 않은 파일을 포함하는 맵과 같은 오브젝트이다. 템플릿에 접근을 제공하지는 않지만, (.helmignore로 배제되지 않았다면) 존재하는 추가 파일에 대한 엑세스를 제공한다. 파일은 {{ index .Files "file.name" }}나 {{.Files.Get name }}함수를 사용해서 접근할 수 있다.
        8. Capabilities: 쿠버네티스의 버전({{ .Capabilities.KubeVersion }})과 지원되는 쿠버네티스 API 버전 ({{ .Capabilities.APIVersions.Has "batch/v1" }})에 대한 정보를 포함하는 맵과 같은 오브젝트  
    * 예시
        ```yaml
        imageRegistry: "quay.io/deis"
        dockerTag: "latest"
        pullPolicy: "Always"
        storage: "s3"
        ``` 
    * 차트는 기본 values.yaml 파일을 포함하고, 헬름 설치 명령은 추가 YAML 값을 제공함으로써 사용자가 값을 덮어쓸 수 있게한다.
        ```bash
        helm install --generate-name --values=myvals.yaml wordpress
        ``` 
- 헬름은 파라미터를 요구하거나 지시하지 않음


### References
- [Helm Docs](https://helm.sh/ko/docs/)