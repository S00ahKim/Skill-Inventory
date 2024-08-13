# GraphQL (GQL)
- 쿼리 언어 like SQL
```
query {
  teams {
    manager
    office
  }
}
```
- 목적: 서버로부터 효율적으로 데이터 가져오기
- 특징
    * 단 하나의 Endpoint (<-> REST API)
    * 불러오는 데이터를 쿼리 조합으로 결정
    * POST 만 사용한다 (통신 프로토콜은 구현체에 따라 다름, 웹소켓 등 사용)
- 장점
    * HTTP 요청 횟수 줄어듦
      * Underfetcing 문제 해결
      * 요청 횟수 감소
    * HTTP 응답 사이즈 줄어듦
      * Overfetching 문제 해결
      * 데이터 전송량 감소
- 단점
    * 텍스트만으로 처리할 수 없는 요청을 다루기 어려움
      * ex. 파일 전송
        + 외부 서비스 사용 필요 
        + ex. base64 인코딩
        + ex. apollo-upload-server (GraphQL multipart 요청 명세를 구현하는 라이브러리)
    * 고정된 요청과 응답만 필요하다면 비효율적
- Apollo GraphQL
    * `Server` -> `Apollo Server Client` -> `Apollo Client`
    * 백엔드/프론트엔드 동시 사용
    * 간편하고 쉬운 설정
    * 많은 기능 제공
- 주요 구성 요소
    1. `Type`: 리턴될 데이터 형태
    2. `Resolver`: 데이터를 리턴하는 함수 (구체적인 과정을 구현해야 함 - 백엔드 단)
    3. `Query`: Read
    4. `Mutation`: Create, Update, Delete


### Reference
- [GraphQL and Apollo](https://www.yalco.kr/lectures/graphql-apollo/)
- [GraphQL과 RESTful API](https://www.holaxprogramming.com/2018/01/20/graphql-vs-restful-api/)
- [gRPC 서비스와 HTTP API 비교](https://docs.microsoft.com/ko-kr/aspnet/core/grpc/comparison?view=aspnetcore-5.0)
- [GraphQL과 REST의 차이점](https://hwasurr.io/api/rest-graphql-differences/)