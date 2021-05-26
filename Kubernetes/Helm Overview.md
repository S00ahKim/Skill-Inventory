# Helm Overview
- Kubernetes 상의 배포를 위한 Package Manager (패키지 = 차트)
- Kubernetes 클러스터에서 어플리케이션의 구성, 설정을 관리하기 위해 개발
- 헬름은 쿠버네티스 내부에 `charts`를 설치하고, 각 설치에 대해 새로운 `release`를 생성한다. 새로운 차트를 찾기 위해 헬름 차트 `repositories` 를 검색할 수 있다.


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
1. `helm search` 차트 찾기
    ```bash
    helm search hub wordpress                                  # 헬름 허브에서 사용 가능한 모든 wordpress 차트를 찾음
    helm repo add brigade https://brigadecore.github.io/charts # 로컬 헬름 클라이언트에 추가된 저장소들을 검색

    # 검색할 때 단어 또는 문구의 일부분만 입력해도 된다.
    ```
2. `helm install` 패키지 설치
    ```bash
    helm install happy-panda stable/mariadb                    # 파라미터로 {사용자 지정 릴리스 이름}, {설치하려는 차트 이름}을 받음

    # 헬름이 생성해주는 이름을 그대로 사용하려면 릴리스 이름을 넣지 말고 --generate-name을 사용
    # 설치하면서 어떤 리소스가 생성되는지, 릴리스의 상태는 어떤지, 추가 설정단계가 있는지에 관한 정보 출력
    # 차트의 옵션 구성을 확인하려면 helm show values를 사용
    ```
3. `helm upgrade` & `helm rollback` 릴리스 업그레이드 및 실패 복구
    ```bash
    helm upgrade -f panda.yaml happy-panda stable/mariadb     # -f = --values / happy-panda 릴리스의 차트가 업그레이드
    helm get values happy-panda                               # 새로운 설정이 적용되었는지 확인
    helm rollback happy-panda 1                               # helm rollback [RELEASE] [REVISION]를 사용하여 이전 릴리스로 간단히 롤백
    ``` 
4. `helm uninstall` 릴리스 언인스톨하기
   ```bash
   helm uninstall happy-panda                                 # 클러스터에서 릴리스를 제거 (롤백 불가)
   helm list                                                  # 현재 배포된 모든 릴리스들을 확인 (위 명령어 실행 시, 사라져서 나옴)


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