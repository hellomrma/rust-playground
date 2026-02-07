use std::num::ParseIntError;

pub fn run() {
    // Rust는 예외(exception)가 없다!
    // 대신 Result<T, E> 타입을 사용

    // 1. Result로 에러 처리
    match "42".parse::<i32>() {
        Ok(n) => println!("  파싱 성공: {n}"),
        Err(e) => println!("  파싱 실패: {e}"),
    }

    match "abc".parse::<i32>() {
        Ok(n) => println!("  파싱 성공: {n}"),
        Err(e) => println!("  파싱 실패: {e}"),
    }

    // 2. ? 연산자로 간결하게
    match add_strings("10", "20") {
        Ok(result) => println!("  10 + 20 = {result}"),
        Err(e) => println!("  에러: {e}"),
    }

    match add_strings("10", "oops") {
        Ok(result) => println!("  10 + oops = {result}"),
        Err(e) => println!("  에러: {e}"),
    }

    // 3. unwrap_or 같은 편의 메서드
    let value: i32 = "not_a_number".parse().unwrap_or(0);
    println!("  기본값 사용: {value}");
}

fn add_strings(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let x = a.parse::<i32>()?; // 실패하면 에러를 바로 반환
    let y = b.parse::<i32>()?;
    Ok(x + y)
}
