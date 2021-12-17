# CPU 스케줄링
> 어떤 프로세스를 프로세서에 할당할 것인가?


## 기본 개념
- 멀티프로그래밍 환경에서 CPU 이용률을 최대화하기 위해, 항상 실행 중인 프로세스를 갖게 하려면 꼭 필요!
- CPU의 유휴 시간
    * 프로세스의 실행은 CPU burst, I/O burst의 사이클로 구성됨.
    * IO burst bound에서는 CPU가 일하지 않는 시간이 생김.
    * 이 시간에 OS는 **레디 큐**에서 CPU 스케줄러에 의해 선택된 프로세스를 실행함.
        + 레디 큐는 링크드리스트/바이너리트리/FIFO/우선순위 큐 등으로 구현 가능
        + 레디 큐에는 PCB가 들어 있음 
- 선점형`preemptive` vs 비선점형`non-preemptive`
    * 스케줄링은 언제 일어나는가?
        1. 한 프로세스가 실행`running` 상태에서 대기`waiting` 상태로 전환될 때 (I/O 발생)
        2. 프로세스가 실행`running` 상태에서 준비 완료`ready` 상태로 전환될 때 (인터럽트 발생)
        3. 프로세스가 대기`waiting` 상태에서 준비 완료`ready` 상태로 전환될 때 (I/O 종료)
        4. 프로세스가 종료할 때
    * 선점형? 시분할(time-sharing) 시스템에서 타임 슬라이스가 소진되었거나, 인터럽트나 시스템 호출 종료시에 더 높은 우선 순위 프로세스가 발생 되었음을 알았을 때, 현 실행 프로세스로부터 강제로 CPU를 회수하는 것 (2,3; 경우에 따라 비선점형일 수도)
        + 데이터가 다수의 프로세스에 의해 공유될 때 racing condition이 발생 가능
        + mutex lock, monitor 등의 기법을 사용해서 racing condition을 피함 
    * 비선점형? 일단 할당받은 이후에는 프로세스가 스스로 CPU를 릴리즈하기 전까지는 terminate/switch 없음 (1,4)
- 디스패처(Dispatcher)
    * CPU 코어의 제어를 CPU 스케줄러가 선택한 프로세스에 주는 모듈
    * 작업?
        1. 한 프로세스에서 다른 프로세스로 컨텍스트 스위칭
        2. 유저 모드로 전환
        3. 프로그램을 다시 시작하기 위해 사용자 프로그램의 적절한 위치로 이동(jump; 새로운 프로세스의 적당한 위치로 resume) 
    * dispatch latency: 하나의 프로세스를 정지하고 다른 프로세스의 수행을 시작하는데까지 소요되는 시간


## 스케줄링 기준
> trade-off
- cpu utilization (cpu를 놀지 않게 하기. 단위 시간 동안 이용되는 CPU 비율을 최대화하기)
- throughput 높이기 (단위시간 내 작업 완료되는 프로세스 수를 최대화하기)
- turnaround time (어떤 프로세스를 제출하고 종료할 때까지의 시간을 최소화하기)
- waiting time (어떤 프로세스가 레디 큐에서 대기하는 시간을 최소화하기)
- response time (하나의 요청을 보낸 뒤 첫 응답이 돌아오는 시간을 최소화하기)


## 스케줄링 알고리즘
1. First Come First Served Scheduling, FCFS
    - 먼저 요청한 프로세스에게 먼저 할당한다
    - convoy effect: 평균 대기 시간이 아주 길어질 수 있음
    - 비선점형 
2. Shortest Job First Schduling, SJF
    - cpu burst가 짧게 걸리는 프로세스에게 먼저 할당한다
    - 평균 대기 시간은 최소, cpu burst 길이는 정확히 모르고 예측해야 함
    - 선점형 또는 비선점형 (선점형일 경우 앞의 프로세스가 실행되는 동안 새로운 프로세스가 레디 큐에 도착하면 선택이 발생)
3. Round Robin Scheduling, RR
4. Priority Scheduling
5. Multilevel Queue Scheduling
6. Multilevel Feedback Queue Scheduling (현대적 스케줄링)
