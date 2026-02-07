mod examples;

#[tokio::main]
async fn main() {
    println!("=== Rust Playground ===\n");

    println!("--- 1. 기본 문법 ---");
    examples::basics::run();

    println!("\n--- 2. 소유권(Ownership) - Rust의 핵심 ---");
    examples::ownership::run();

    println!("\n--- 3. 패턴 매칭 ---");
    examples::pattern_matching::run();

    println!("\n--- 4. 에러 처리 ---");
    examples::error_handling::run();

    println!("\n--- 5. 구조체와 트레이트 (OOP 비슷한 것) ---");
    examples::structs_traits::run();

    println!("\n--- 6. 컬렉션과 이터레이터 ---");
    examples::collections::run();

    println!("\n--- 7. 동시성(Concurrency) ---");
    examples::concurrency::run();

    println!("\n--- 8. Async/Await ---");
    examples::async_await::run().await;

    println!("\n--- 9. 웹서버 (Axum) ---");
    examples::web_server::run().await;
}
