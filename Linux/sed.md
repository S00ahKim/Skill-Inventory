# sed (streamlined editor)
- 텍스트 파일을 편집하는 일종의 에디터
- vi가 실행 결과를 저장하는 데 비해, sed는 원본 변경을 기본적으로는 하지 않음 (하려면 -i 옵션 필요)
- 내부에 패턴 버퍼, 홀드 버퍼를 둠
    1. 인풋 스트림에서 패턴 버퍼에 데이터를 담음 (원본을 조작하지 않기 위함)
    2. 데이터의 변형/추가를 위해 홀드 버퍼를 사용
    3. 작업이 완료되면 다시 패턴 버퍼에 데이터가 담김
    4. 아웃풋 스트림에 전달 (일반적으로 stdout)


### commands
- 단어 치환하기: s (substitute)
  ```bash
  # 처음 등장하는 공백 제거
  sed 's/^ //' example_text.txt
  ``` 