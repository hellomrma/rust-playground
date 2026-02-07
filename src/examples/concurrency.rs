use std::sync::{Arc, Mutex};
use std::thread;

pub fn run() {
    // 1. 스레드 생성
    let handle = thread::spawn(|| {
        "스레드에서 인사!"
    });
    let result = handle.join().unwrap();
    println!("  {result}");

    // 2. 여러 스레드로 병렬 계산
    let mut handles = vec![];
    for i in 0..4 {
        let handle = thread::spawn(move || {
            let sum: i64 = (i * 250_000 + 1..=(i + 1) * 250_000).sum();
            sum
        });
        handles.push(handle);
    }

    let total: i64 = handles.into_iter().map(|h| h.join().unwrap()).sum();
    println!("  4개 스레드로 1~1,000,000 합계 = {total}");

    // 3. 공유 상태 - Arc + Mutex
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("  5개 스레드 카운터 결과: {}", *counter.lock().unwrap());
}
