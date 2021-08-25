# cut
- 텍스트 파일의 각 줄에서 특정 부분 자르기


### commands
- example_text 파일에서 특정 문자(-) 기준으로 잘라서 두번째 열 가져오기
  ```bash
  cut -d "-" -f 2 example_text.txt
  ```
- example_text 파일에서 특정 문자(,) 기준으로 잘라서 첫번째, 세번째, 다섯번째 열 가져오기
  ```bash
  cut -d "," -f 1,3,5 example_text.txt
  ```