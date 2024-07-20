# Ingress

```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: example-ingress
  namespace: default
  annotations:  # 메타정보 저장을 위한 프로퍼티 (label과 유사하지만, 리소스 필터링은 불가, 인그레스 컨트롤러에게 메타정보를 전달할 목적)
    kubernetes.io/ingress.class: nginx # 이 인그레스가 엔진엑스 인그레스 컨트롤러에 의해 처리됨을 명시
    nginx.ingress.kubernetes.io/rewrite-target: / # path 기반 라우팅시 추가 필요
spec:
  rules: # 외부 트래픽을 처리하는 방법 정의
  - host: example.com # 특정 도메인으로 들어오는 트래픽에 대한 라우팅을 정의 (생략시 모든 호스트 트래픽)
    http:
      paths:
      - path: / # 인그레스 path
        pathType: Prefix
        backend: # 인그레스의 트래픽을 받을 서비스 포트 정의
          service:
            name: example-service
            port:
              number: 80
      - path: /api
        pathType: Prefix
        backend:
          service:
            name: api-service
            port:
              number: 80
  tls:
  - hosts:
    - example.com
    secretName: example-tls
---
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: example-ingress-sub
  namespace: default
  annotations: 
    kubernetes.io/ingress.class: nginx
spec:
  rules: 
  - host: example-sub.com # 서브 도메인을 두고 라우팅
    http:
      paths:
      - path: / 
        pathType: Prefix
        backend: 
          service:
            name: example-service-sub
            port:
              number: 80
```
- 애플리케이션 계층(layer7)에서 외부 트래픽을 처리
    * 도메인 주소 필요
    * 매번 로드밸런서를 생성하지 않고도 애플리케이션 레이어에서 라우팅 가능
    * 브라우저 -> 인그레스 컨트롤러 (Nginx Ingress Controller) -> Nginx -> Service -> 파드(들)
    * cf. 서비스의 LoadBalancer는 주로 L4에서 동작하며, 기본적으로 클라우드 제공자의 로드 밸런서를 사용하는 것으로 설계되어 있음
- 인그레스
    * 트래픽 처리에 대한 정보를 담은 일종의 정의(규칙)
- 인그레스 컨트롤러
    * 인그레스에 정의된 트래픽 라우팅 규칙을 보고 그에 따라 외부 트래픽을 서비스로 라우팅
    * 쿠버네티스 내장 컨트롤러와 달리 명시적으로 컨트롤러 설치 필요
    * 다양한 구현체 있음
        + ex. `NGINX Ingress Controller`, `HAProxy`, `AWS ALB`, `Ambassador`, `Kong`, `traefik` 등
        + helm을 사용해 설치하면, 인그레스 컨트롤러 관련한 파드와 서비스(로드밸런서 타입)가 생김
- 로드밸런싱
    * 인그레스 컨트롤러가 생성한 엔진엑스 설정 파일에는 인그레스에 정의된 서비스가 선택할 파드의 주소가 적용됨
    * 엔진엑스가 제공하는 다양한 알고리즘으로 서비스가 파드에게 부하를 분산시키는 방법을 지정 가능
    * 엔진엑스 인그레스 컨트롤러의 컨피그맵을 수정하여 **글로벌** 설정을 적용
        ```yaml
        apiVersion: v1
        kind: ConfigMap
        metadata:
          name: nginx-configuration
          namespace: ingress-nginx
        data:
          load-balance: "least_conn"
        ```
    * **특정** 인그레스 리소스에 주석 사용
        ```yaml
        apiVersion: networking.k8s.io/v1
        kind: Ingress
        metadata:
          name: example-ingress
          annotations:
            nginx.ingress.kubernetes.io/load-balance: "least_conn"
        spec:
          rules:
          - host: example.com
            http:
              paths:
              - path: /
                pathType: Prefix
                backend:
                  service:
                    name: example-service
                    port:
                      number: 80
        ```
- path 기반 라우팅
    * 라우팅될 각각의 서비스를 준비함
    * 인그레스를 url의 path에 대해서 정의함
    * 인그레스 설정에 의해 example.com/xxx... 는 example-service 에, example.com/api/xxx...는 api-service 로 라우팅됨
    * `nginx.ingress.kubernetes.io/rewrite-target`는 인그레스로 들어오는 url 주소가 그대로 서비스에 전달되어 비정상 동작하지 않도록 nginx의 rewrite 지시자와 동일하게 프록시되는 서버에 path를 /로 재정의함
- 도메인 기반 라우팅
    * 서브 도메인과 라우팅될 각각의 서비스를 준비함
    * 인그레스를 서브 도메인에 대해서도 정의함
    * 인그레스 설정에 의해 spec.rules.host => spec.rules.http.paths.backend.serviceName:servicePort 로 라우팅됨
    * L4에서는 동일한 IP를 가져도 L7에서는 다른 도메인을 두어 서로 다른 서비스로 트래픽 라우팅 가능
- Authentication
    * 인그레스 리소스에 auth 기능을 추가 가능하며, 인그레스 컨트롤러 구현체는 기반 서버가 제공하는 기능을 대부분 사용 가능함
    * 인증을 추가할 경우, 헤더에 적절한 값을 세팅해주어야 정상 응답을 받을 수 있게 됨
    * ex. Basic Auth 설정
        + httpasswd 툴을 사용해서 인증 파일 생성
            - `htpasswd -cb auth myuser foo`
            - -c: 새로운 htpasswd 파일을 생성
            - -b: 비밀번호를 명령줄에서 직접 지정
            - auth: 생성할 또는 업데이트할 htpasswd 파일의 이름
            - myuser: 추가할 사용자의 이름
            - foo: myuser의 비밀번호 
        + 인증 파일을 시크릿 리소스로 만듦
            - `kubectl create secret generic basic-auth --from-file=auth`
        + 시크릿 리소스를 인그레스에 설정
            ```yaml
            (...)
            metadata:
              name: example-ingress
              annotations:
                nginx.ingress.kubernetes.io/auth-type: basic
                nginx.ingress.kubernetes.io/auth-secret: basic-auth
                nginx.ingress.kubernetes.io/auth-realm: 'Authentication Required - myapp' 
            (...)
            ```
        + cf. auth-realm은 인증을 요청할 때 보여주는 메시지로, 어느 영역에서 인증이 필요한지 명확히 알려주는 역할을 함
        + cf. 위 설정은 엔진엑스 인그레스 컨트롤러에 의해 엔진엑스 설정 파일에 적용된다
            ```conf
            server {
                listen 80;
                server_name example.com;

                location / {
                    auth_basic "Authentication Required - myapp"; # nginx.ingress.kubernetes.io/auth-realm
                    auth_basic_user_file /etc/nginx/secrets/basic-auth; # nginx.ingress.kubernetes.io/auth-secret

                    proxy_pass http://example-service; # spec.rules[].http.paths[].backend
                    # 기타 설정들...
                }
            }
            ```
- TLS
    * Transport Layer Security, 네트워크 통신의 보안을 제공하기 위해 설계된 암호화 프로토콜 (HTTPS 통신)
    * 인그레스 리소스의 annotations에 인증서 정보를 등록하여 쉽게 제공 가능
    * Self-signed 인증서 설정
        + 정식 기관으로부터 발급받은 인증서가 아니고, 직접 서명한 인증서로, 브라우저상 경고는 뜨지만 테스트해보기 좋음
        + openssl 툴을 이용하여 인증서 생성
        + 생성된 인증서를 시크릿 리소스 형태로 저장  ex.`type: kubernetes.io/tls`
        + tls 설정을 한 인그레스 생성 ex. `tls.secretName: my-tls-certs`
    * 정식 CA 서명 인증서 설정
        + 공인된 인증서 발급을 돕고, 지속적으로 유효하도록 갱신하는 쿠버네티스 인증서 관리 컴포넌트 `cert-manager`를 헬름으로 설치
        + cert-manager가 사용할 Issuer라는 커스텀 리소스를 생성 (특정 네임스페이스 안의 인그레스만 관리하는 발급자일 경우)
            - cf. 만약 네임스페이스와 무관하게 클러스터 레벨에서 동작하게 하려면 ClusterIssuer
            - yaml 파일에는 갱신유형, 관리자 이메일, 인증서 발급 서버, 개인키 시크릿 등을 설정할 수 있음 
        + cert-manager 설정을 적용한 인그레스 생성
        + `kubectl get certificate` 커맨드로 인증서 상태를 확인 가능