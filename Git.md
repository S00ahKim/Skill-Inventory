# Git

## 개념
- 버전 관리 시스템
- 저장소(repository)에서 버전 관리가 이루어진다
    * 로컬 저장소: 내 PC에 파일이 저장됨. 작업자만 볼 수 있음. (`git init`)
    * 원격 저장소: 원격 저장소 서버에 파일이 저장됨. 여러 사람이 함께 공유할 수 있음. (`git clone`)
- 구조
    * working directory: 작업 공간
    * (--`git add`-->) staging area: 일종의 대기실
    * (--`git commit`-->) local repository
    * (--`git push`-->) remote repository
- [커밋 메시지 잘 남기는 법](https://meetup.toast.com/posts/106)
    * 제목은 적당한 길이로, 명령조로 작성
    * 커밋이 제목과 본문으로 구분될 경우, 한 줄 띄워 구분하기
    * 본문은 무엇을, 왜 했는가를 기술하기


## 툴
- GitHub: 깃(Git)을 사용하는 프로젝트를 지원하는 웹호스팅 서비스. Git 저장소 웹사이트. (유사: `Bitbucket`, `Gitlab`)
- GitHub Desktop
- SourceTree
- GitKraken
- Tortoise Git