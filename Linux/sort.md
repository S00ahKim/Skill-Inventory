# sort
- 일반
  ```bash
  sort example_sort.txt

  # apple,101
  # banana,10
  # cat,3
  ```

- 역순 (`-r`,`--reverse`)
  ```bash
  sort -r example_sort.txt

  # cat,3
  # banana,10
  # apple,101
  ```

- 필드 구분자 지정 (`-t`, `--field-separator=SEP`)
  ```bash
  # 구분자가 쉼표(,) 인 경우
  sort -t, example_sort.txt
  ```

- 특정 필드를 기준으로 (`-k`, `--key=POS1`)
  ```bash
  sort -t, -k2 example_sort.txt

  # banana,10
  # apple,101
  # cat,3

  # 위와 같은 결과가 나온 이유는 10을 숫자 10이 아니라 문자 '10'으로 여겼기 때문이다.
  ```

- 숫자 데이터로 지정
  ```bash
  sort -t, -k2 -n example_sort.txt

  # cat,3
  # banana,10
  # apple,101
  ```

- 정렬하고 중복 제거 (`-u`, `--unique`)
  ```bash
  sort -u example_sort.txt
  ```

- 대소문자 구분하지 않고 정렬 (`-f`, `--ignore-case`)
  ```bash
  sort -f many_alphabets.txt

  # A
  # a
  # B
  # b
  # C
  # c
  ```