# 프로그램의 실행
![ProgramExecution1](images/ProgramExecution1.jpeg)

## 1. 프로그램 작성
1. 언어의 문법에 맞게 프로그램을 작성
2. 실행 가능한 파일로 변환 (빌드)
    - `전처리기`: 치환 작업 (ex. C의 경우 #으로 시작하는 지시자 ex.`#include`)
    - `컴파일러`: 네이티브 코드(사용하는 환경의 칩에 맞는 기계어)로 소스코드를 번역함
        * CPU를 디자인할 때 명령어도 디자인하기 때문에, 칩에 따라 명령어가 다르다.
        * 네이티브 코드는 CPU가 이해할 수 있는 명령문이다.
        * 컴파일러가 하는 일?
            + 변수 -> 메모리의 변수 할당 주소
            + 함수 -> 명령문load, store, add, sub, mul, div, gt, eq, lt, jump, goto 등) 주소
            + 각종 타입 -> 기본 타입 (int, float, double, byte 등)
            + 제어 로직 -> 명령문 주소, jump, goto
    - `어셈블러`: 바이너리 코드로 변환
    - `링커`: 프로그램에 필요한 요소들(바이너리, 라이브러리 등)을 하나로 합침 (ex. `.exe`)


## cf. Compiler vs Interpreter
### 컴파일러
- ex. `C`
- 한 언어에서 다른 언어로 번역하는 프로그램
- 장점: 바이너리를 실행시키는 것이기 때문에 실행 속도가 인터프리터에 비해 빠르다
- 단점: 컴퓨터의 칩마다 명령어 셋이 다르기 때문에 환경별로 재컴파일이 필요하다

### 인터프리터
- ex. `Javascript`
- 번역해야 할 파일을 받아 한 줄씩 실행시키는 프로그램
- 장점: 인터프리터만 설치되어 있으면 같은 프로그램(스크립트)을 다른 컴퓨터에서도 실행 시킬 수 있다
- 단점: 실행 전 번역 과정을 거치기 때문에 실행 속도가 컴파일러에 비해 느리다

### 하이브리드 언어
- ex. `Java`
- java -(컴파일러 `javac`)-> class -(인터프리터 `JVM`)-> 실행
    * 자바 프로그램은 기계어 코드로 바뀌는 게 아니라 바이트 코드(`.class`)라는 특수한 명령문으로 컴파일됨
    * 실행이 VM에 의해 소프트웨어적으로 실행되므로, C와 다르게 링크 과정 불필요, JVM이 필요할 때마다 참조 (링커+OS의 역할까지도 일부 수행)
- JIT(Just-in-Time) 컴파일러: 자주 사용되는 바이트코드를 기계어로 컴파일해 사용


## 2. OS에 프로그램 실행 요청
- OS(운영체제)? 
    * 하는 일: 입출력 관리, 메모리할당, 스케줄링, 장치 드라이버 관리 등 `자원 관리`
    * 위치: 하드웨어 - OS - bash 등 셸, 응용 프로그램 - 사용자


## 3. OS가 하드디스크에서 데이터를 읽어 메인 메모리에 load
- 저장 공간의 계층
    * CPU와 가까운 순으로 저장 공간을 나열하면, `레지스터`, `CPU 캐시 메모리`, `메인 메모리`, `보조기억장치`, `외부기억장치`가 있다.
    * CPU로부터 멀어질수록 데이터를 저장하는 용량이 커지고 접근 속도는 느려진다.

### 메모리 공간
- 숫자가 작을수록 low address memory
1. Code 영역
    - 실행할 프로그램의 코드(명령어)
    - ROM
2. Data 영역
    - 전역 변수, 스태틱 변수
    - 프로그램이 시작되면서 할당되어 종료될 때 해제됨
    - 영역 구분
        1. ROM - Data
        2. RAM
           * BSS(Block Stated Symbol): 초기화되지 않은 변수
           * Data: 초기화된 변수. 프로그램 실행 중 자유롭게 접근해서 수정, 변경이 가능. 런타임 시 변경되는 값은 RAM에 복사된 값을 사용.
3. Heap 영역
    - `Dynamic Memory Allocaiton`
        * 힙 영역 이외의 영역들은 모두 컴파일러가 행동(할당, 해제 등)을 제어함
        * 힙 영역의 경우, 개발자가 행동 제어 가능 + 런타임에 데이터 적재
        * C: malloc(), free(), realloc() 등
        * 자바: Garbage Collector가 제어
4. Stack 영역
    - 지역 변수, 매개 변수
    - 함수가 호출될 때 할당되어 종료될 때 해제됨
    - 힙과 스택은 같은 공간을 공유할 수 있음
        * 힙은 낮은 주소부터, 스택은 높은 주소부터
        * 서로의 공간을 침범할 경우 각각 `Heap Overflow`, `Stack Overflow`
        * 스택은 컴파일 시 이미 할당된 공간 안에서 포인터의 위치만 변경하지만, 힙은 필요한 크기와 메모리 상황을 연산해야 하기 때문에 스택 할당 속도가 더 빠르다.

### 폰 노이만 아키텍처
- 현대 컴퓨터 구조의 기초. a.k.a. `Stored Program Concept`
- 명령어는 메모리에 저장되고, CPU에 의해 fetch, decode, execute된다.
    * `fetch`: 메인메모리에 저장되어있던 명령어를 CPU내부로 가져온다. (by 버스인터페이스)
    * `decode`: 명령어를 해석한다. (by 컨트롤유닛)
    * `execute`: 연산을 실행한다. (by ALU. 크게보면 CPU 전체)


## 4. 연산을 위해 데이터가 메인 메모리에서 CPU로 fetch
- CPU?
    * 중앙처리장치(Central Processing Unit)
    * 컴퓨터의 연산 담당, 데이터를 처리
    * 구성: ALU, Control Unit, Register set, Bus Interface etc.
- fetch?
    * 메모리의 명령어를 CPU의 IR(Instruction Register)에 저장
    * 이때, 하드웨어끼리의 데이터 전송을 위해 버스 사용


### System Bus
- CPU와 메인 메모리 사이에 있는 연결고리를 버스라고 한다.
- 종류
    1. `data bus`: 데이터 이동
    2. `address bus`: 메모리 주소 이동
    3. `control bus`: 위 두 버스를 제어하기 위한 컨트롤 신호 이동 (양방향으로 데이터를 주고받기 때문)
- CPU의 Bus Interface란?
    * a.k.a. Controller, Adapter
    * 버스가 어떻게 데이터를 전송하는지, 그에 대한 프로토콜 또는 통신에 대한 인터페이스
    * CPU는 이를 통해 Register에 저장된 Data를 I/O버스에 보내거나 받는다.
- cf. 32비트 컴퓨터와 64비트 컴퓨터의 구분
    * 한번에 송.수신 할수 있는 데이터의 크기 & 한번에 처리 할수 있는 데이터의 크기 기준으로 구분
    * 한번에 버스로 이동하는 데이터의 크기가 32, 64인지
    * CPU 내부에서 한번에 처리할 수 있는 데이터의 크기가 32, 64인지


## 5. CPU의 Control Unit이 decode
- Control Unit
    * 프로그램의 명령어를 적절한 순서로 꺼내고,
    * 각 명령을 분석, 해석하여
    * 그에 적절한 신호를 CPU의 연산 장치 등에 보낸다.


## 6. CPU의 ALU(Arithmetic and Logic Unit)에서 execute
- ALU (Arithmetic Logic Unit)
    * 산술연산 + -, 논리연산 AND, OR 과 같은 `연산` 담당
    * 독립적으로 데이터 처리를 하지 못하여 레지스터(피연산자)와 조합하여 쓰인다.
    * 연산 결과를 레지스터 중 하나에 저장한다.
- 컨트롤 로직(control logic): 명령을 해석한 후에 실행을 위해 ALU와 레지스터 파일을 제어하는 회로


## 7. 연산 결과를 레지스터에 store
- Register Set
    * CPU 내부의 이진 데이터 저장을 위한 임시 저장소(메모리)
    * 메모리의 데이터로 연산이나 비교를 하고자 하는 경우, CPU Register로 가져와야 함
    * Control Unit이 필요로 하는 명령어들(->IR), ALU가 필요로 하는 피연산자들을 저장
- 시스템 레지스터: CPU가 명령을 수행하는 데 필요
- 범용 레지스터: 프로그래머에 의해서 사용 가능 (범용 레지스터의 묶음 = 레지스터 파일)


## Reference
- Operating System Concepts 9th Edition
- 뇌를 자극하는 윈도우 시스템 프로그래밍
- [메모리의 구조](http://www.tcpschool.com/c/c_memory_structure)
- [(정적할당/동적할당) 스택(stack)과 힙(heap)영역](https://blog.naver.com/star7sss/220851224606)
- [JIT - Just in Time Compiler](https://plas.tistory.com/44?category=768972)
- [1장. 컴퓨터구조에 대한 첫번째 이야기](https://awesome-dev.tistory.com/35?category=742265)