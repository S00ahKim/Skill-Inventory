# Linux


## general
1. pipe
```bash
ls | grep hello
```
- pipe를 사용해 명령어를 이어지게 할 수 있음
- 앞|뒤 라고 하면, 앞의 결과를 뒤에 전달할 수 있음


## grep: 문자열 검색
```bash
grep -B10                  //현재 줄 전으로 10줄까지 Back
grep -A10                  //현재 줄 뒤로 10줄까지 After
grep -C10                  //현재 줄 앞뒤로 10줄까지 Context
grep -E 정규표현식            // 정규표현식 패턴 검색
grep -v 제외하고싶은문자열      // 해당 문자열을 제외한 검색 결과 표시
```