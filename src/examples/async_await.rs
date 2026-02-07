use tokio::time::{sleep, Duration};

pub async fn run() {
    // async/await = 비동기 프로그래밍
    // JavaScript의 async/await와 개념이 같지만,
    // Rust는 "런타임"을 직접 선택한다 (여기선 tokio 사용)

    // 1. 기본 async 함수
    let greeting = say_hello("Rust").await;
    println!("  {greeting}");

    // 2. 여러 작업을 동시에 실행 (tokio::join!)
    //    순차 실행이면 3초 걸릴 작업이 1초만에 끝남!
    println!("  3개 작업 동시 시작...");
    let (a, b, c) = tokio::join!(
        do_task("API 호출", 300),
        do_task("DB 조회", 500),
        do_task("파일 읽기", 200),
    );
    println!("  결과: {a}, {b}, {c}");

    // 3. tokio::spawn - 백그라운드 태스크
    let handle = tokio::spawn(async {
        sleep(Duration::from_millis(100)).await;
        42 // 백그라운드에서 계산
    });
    let value = handle.await.unwrap();
    println!("  백그라운드 태스크 결과: {value}");

    // 4. select! - 가장 먼저 끝나는 것만 취함
    tokio::select! {
        result = do_task("빠른 작업", 100) => {
            println!("  select 승자: {result}");
        }
        result = do_task("느린 작업", 500) => {
            println!("  select 승자: {result}");
        }
    }
}

async fn say_hello(name: &str) -> String {
    // await 없이도 async 함수를 만들 수 있음
    format!("{name}에서 async/await!")
}

async fn do_task(name: &str, ms: u64) -> String {
    sleep(Duration::from_millis(ms)).await; // 비동기 대기
    format!("{name} 완료({ms}ms)")
}
