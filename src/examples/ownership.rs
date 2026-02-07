pub fn run() {
    // Rust의 가장 독특한 특징: 소유권 시스템
    // 메모리를 안전하게 관리하면서도 가비지 컬렉터가 없음!

    // 1. 각 값은 하나의 소유자만 가진다
    let s1 = String::from("hello");
    let s2 = s1; // s1의 소유권이 s2로 이동(move)
    // println!("{s1}"); // 컴파일 에러! s1은 더 이상 유효하지 않음
    println!("  s2 = {s2}");

    // 2. 복사하고 싶으면 clone
    let s3 = s2.clone();
    println!("  s2 = {s2}, s3 = {s3}");

    // 3. 참조(borrowing) - 소유권 없이 빌려쓰기
    let s4 = String::from("world");
    let len = calculate_length(&s4); // &로 빌려줌
    println!("  '{s4}'의 길이 = {len}"); // s4 여전히 사용 가능!

    // 4. 가변 참조
    let mut s5 = String::from("hello");
    add_world(&mut s5);
    println!("  변경 후: {s5}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
    // s는 빌려온 것이므로 여기서 해제되지 않음
}

fn add_world(s: &mut String) {
    s.push_str(", world!");
}
