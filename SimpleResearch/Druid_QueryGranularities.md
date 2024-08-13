# Druid - Query granularities (세분성)
- [docs](https://druid.apache.org/docs/latest/querying/granularities.html)
- 드루이드는 Druid SQL과 native query를 제공
- 세분성은 시간 차원에서 데이터를 버킷화하는 방법 또는 시간, 일, 분 등으로 데이터를 집계하는 방법을 결정
- Simple Granularities
    * group by 한 결과값. 이하 단위의 데이터는 삭제됨.
    * 다음 중 하나의 단위를 선택: all, none, second, minute, fifteen_minute, thirty_minute, hour, day, week, month, quarter, year
- Duration Granularities
    * 밀리초(ex. 360000)로 단위의 크기를 지정
- Period Granularities 
    * ISO8601 형식(ex. P2W, PT1M)으로 단위의 크기를 지정
    * timeZone, origin (opt.)