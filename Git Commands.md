# Git 명령어

- `git init` : git 생성하기
- `git clone {git_path}` : 코드가져오기
- `git checkout {branch_name}` : 브랜치 선택하기
- `git checkout -t {remote_path/branch_name}` : 원격 브랜치 선택하기
- `git branch {branch_name}` : 브랜치 생성하기
- `git branch -r` : 원격 브랜치 목록보기
- `git branch -a` : 로컬 브랜치 목록보기
- `git branch -m {branch_name} {new_branch_name}` : 브랜치 이름 바꾸기
- `git branch -d {branch_name}` : 브랜치 삭제하기
- `git push {remote_name} — delete {branch_name}` : 원격 브랜치 삭제하기 ( git push origin — delete gh-pages )
- `git add {file_path}` : 수정한 코드 선택하기 ( git add * )
- `git commit -m “commit_description”` : 선택한 코드 설명 적기 ( git commit -m “내용”)
- `git push {romote_name} {branch_name}` : add하고 commit한 코드 git server에 보내기 (git push origin master)
- `git pull` : git서버에서 최신 코드 받아와 merge 하기
- `git fetch` : git서버에서 최신 코드 받아오기
- `git reset -- hard HEAD^` : commit한 이전 코드 취소하기
- `git reset -- soft HEAD^` : 코드는 살리고 commit만 취소하기
- `git reset -- merge` : merge 취소하기
- `git reset -- hard HEAD && git pull` : git 코드 강제로 모두 받아오기
- `git config — global user.name “user_name”` : git 계정Name 변경하기
- `git config — global user.email “user_email”` : git 계정Mail변경하기
- `git stash / git stash save “description”` : 작업코드 임시저장하고 브랜치 바꾸기
- `git stash pop` : 마지막으로 임시저장한 작업코드 가져오기
- `git branch — set-upstream-to={remote_path/branch_name}` : git pull no tracking info 에러해결
- `git cherry-pick A B` : 다른 브랜치의 A, B 커밋을 브랜치로 가져오기
- `git rebase NEW_BASE_BRANCH` : 커밋을 병합하되, 여러 갈래로 나뉘지 않고 하나의 히스토리로 정리하기
- `git rebase -i HEAD~{합치려는 커밋 수} -> pick, squash 명시` : 여러 커밋 병합하기


### Reference
- [자주 사용하는 기초 Git 명령어 정리하기](https://pks2974.medium.com/%EC%9E%90%EC%A3%BC-%EC%82%AC%EC%9A%A9%ED%95%98%EB%8A%94-%EA%B8%B0%EC%B4%88-git-%EB%AA%85%EB%A0%B9%EC%96%B4-%EC%A0%95%EB%A6%AC%ED%95%98%EA%B8%B0-533b3689db81)