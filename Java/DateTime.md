# 날짜와 시간

## 자바 패키지 발전사
- Date
- Calendar
- java.time (jdk1.8~)

## java.time
- 날짜와 시간을 다루는 핵심 클래스들
    * LocalDateTime = LocalDate + LocalTime
    * Period = 날짜 - 날짜
    * Duration = 시간 - 시간
- `.chrono` 표준ISO이 아닌 달력 시스템을 위한 클래스들
- `.format` 날짜, 시간을 파싱/형식화하는 클래스들
    * 자주 쓰이는 형식들은 DateTimeFormatter에 상수로 정의되어 있음
- `.temporal` 날짜, 시간의 필드와 단위를 위한 클래스들
    * 정의 TemporalUnit
    * 구현 ChronoUnit 
- `.zone` 시간대와 관련된 클래스들
    * ZonedDateTime = LocalDateTimne + 시간대 (by ZoneId)
    * OffsetDateTime (by ZoneOffset; 시간대를 시간의 차이로만 구분, 서로 다른 시간대에 존재하는 컴퓨터 간 통신)
