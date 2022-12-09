pub mod client; //client 모듈의 스코프 내에 정의된 코드를 다른 위치에서 찾으라고 말하는 것
pub mod network; //함수와 모듈 모두 공개 상태여야 에러가 없다.

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::client;

    #[test]
    fn it_works() {
        client::connect();
    }
}