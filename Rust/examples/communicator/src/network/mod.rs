// 똑같은 파일 이름 쓰기 패턴을 계속해서 따르는 대신 network 디렉토리로 분리됨.

pub fn connect() { // network 모듈 바깥의 스크립트에서 호출하고자 한다면 network::connect()과 같이 호출
}

pub mod server; // 이 친구는 network의 서브모듈인데, 만약 파일로만 분리한다면 러스트는 그런 계층 구조를 인식하지 못한다.
