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


## git flow
![git flow](../images/git_1.png)
- 브랜치
    * master : 제품으로 출시될 수 있는 브랜치
    * develop : 다음 출시 버전을 개발하는 브랜치
    * feature : 기능을 개발하는 브랜치
    * release : 이번 출시 버전을 준비하는 브랜치
    * hotfix : 출시 버전에서 발생한 버그를 수정 하는 브랜치
- 특징
    * 항시 유지되는 메인 브랜치 master, develop
    * merge 되면 사라지는 보조 브랜치 feature, release, hotfix
- git flow
    1. 일단 master 브랜치에서 시작을 합니다.
    2. 동일한 브랜치를 develop에도 생성을 합니다. 개발자들은 이 develop 브랜치에서 개발을 진행합니다.
    3. 개발을 진행하다가 회원가입, 장바구니 등의 기능 구현이 필요할 경우 A개발자는 develop 브랜치에서 feature 브랜치를 하나 생성해서 회원가입 기능을 구현하고 B개발자도 develop 브랜치에서 feature 브랜치를 하나 생성해서 장바구니 기능을 구현합니다.
    4. 완료된 feature 브랜치는 검토를 거쳐 다시 develop 브랜치에 합칩니다.(Merge)
    5. 이제 모든 기능이 완료되면 develop 브랜치를 release 브랜치로 만듭니다. 그리고 QA(품질검사)를 하면서 보완점을 보완하고 버그를 픽스합니다.
    6. 모든 것이 완료되면 이제 release 브랜치를 master 브랜치와 develop 브랜치로 보냅니다. master 브랜치에서 버전추가를 위해 태그를 하나 생성하고 배포를 합니다.
    7. 배포를 했는데 미처 발견하지 못한 버그가 있을 경우 hotfixes 브랜치를 만들어 긴급 수정 후 태그를 생성하고 바로 수정 배포를 합니다.


## 툴
- GitHub: 깃(Git)을 사용하는 프로젝트를 지원하는 웹호스팅 서비스. Git 저장소 웹사이트. (유사: `Bitbucket`, `Gitlab`)
- GitHub Desktop
- SourceTree
- GitKraken
- Tortoise Git