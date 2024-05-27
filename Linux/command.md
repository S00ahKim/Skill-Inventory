# 명령어

## 기초
- `man`
    * manual
    * 명령어에 대해 알고 싶을 때 (사용 가능한 옵션 확인 등)
    * man mode 에서 나가고 싶다면 `q`
- `clear`
    * 창에 나온 내용을 모두 지움

## 파일 탐색
- `pwd`
    * print working directory
    * 현재 나의 경로
- `ls`
    * list
    * 특정 디렉토리 안의 폴더와 파일 조회
    * args로 경로를 주면 해당 경로의 폴더와 파일 조회
    * `-l` 폴더/파일의 성격(크기 등)까지 조회
    * `-a` 숨겨진 파일 함께 표시
    * `-la` 위 두 기능을 함께 사용
- `cd`
    * change directory
    * 특정 디렉토리로 이동
    * `.` 현재의 경로 (hidden path)
    * `..` 현재 경로의 상위 경로 (hidden path)
    * `~` 최상위 경로
    * `-` 바로 이전 경로
- `find`
    * `-type` 검색 대상. file 또는 directory
    * `-name` 검색하려는 이름
    * ex. 현재 경로와 하위 경로에 대해 텍스트 파일 검색 `find . -type file -name "*.txt"`
- `which`
    * 내가 실행하려는 프로그램이 설치된 경로 확인
    * ex. 노드가 깔린 경로를 확인 `which node`

## 파일 생성 및 관리
- `touch`
    * args로 파일 이름을 넘겨서...
        + 없는 파일이었다면 해당 이름의 파일을 만들 수 있음
        + 있는 파일이었다면 수정 시각이 현재 시각이 됨
- `cat`
    * 파일 내용을 빠르게 확인해볼 수 있음
    * args로 여러 파일 이름을 넘겨서 많은 파일의 내용을 확인 가능
- `echo`
    * args로 넘긴 문자열을 화면에 출력
    * `> {file_name}` 을 붙이면, args를 파일에 덮어씌울 수 있음
    * `>> {file_name}` 을 붙이면, args를 파일에 추가할 수 있음
- `mkdir`
    * make directory
    * 현재 경로에서 새로운 경로를 만들어줌
    * `-p` 하위 경로를 한번에 만듦 ex. `mkdir -p dir1/subdir/subsubdir`
- `cp`
    * copy
    * 파일 복사
    * `cp {file_name} {dir_name}`
- `mv`
    * move
    * 파일 이동 `mv {file_name} {dir_name}`
    * 파일 내용을 복사해서 새로운 파일 이름으로 생성 `mv {file_name} {new_file_name}`
- `rm`
    * remove
    * 폴더 삭제 `rm -r {dir_name}` (recursive)
    * 강제 삭제 `rm -f {file_name}`
- `grep`
    * global regular expression print
    * 모든 텍스트 파일 안에서 특정 문자열 찾기 ex. `grep "문자열" *.txt`
    * 일치하는 라인이 몇 번째인지 확인 ex. `grep -n "문자열" *.txt`
    * 대소문자 구분 없이 확인하기 ex. `grep -ni "world" *.txt` (insensitive)
    * 현재 프로젝트 안의 모든 파일에 대해 찾기 ex. `grep -nir "world" .` (recursive)

## 환경변수
- `export`
    * 환경변수는 대문자로 작성 & 구분자는 = 으로 작성 ex. `export MY_DIR="dir1"`
    * 사용할 때에는 $ 를 붙여줌 ex. `cd $MY_DIR`
- `env`
    * 설정된 모든 환경변수 조회
- `unset {env_name}`
    * 환경변수 삭제

## vi, vim editor
> 터미널에서 자주 사용되는 텍스트 에디터
- `vi {file_name}`
    * 파일 생성 / 생성된 파일 편집
- `i`
    * 입력 신호
- `esc`
    * 종료 신호
- `:w`
    * 저장
- `:q`
    * 종료
- `:wq`
    * 저장하고 종료
- `:q!`
    * (수정한 내용을 저장하지 않고) 강제 종료