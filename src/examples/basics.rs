pub fn run() {
    // 변수는 기본적으로 불변(immutable)
    let name = "Rust";
    let version = 1.93;
    println!("  {name} v{version}");

    // mut 키워드로 가변 변수
    let mut count = 0;
    count += 1;
    println!("  count = {count}");

    // 타입 추론 + 명시적 타입
    let x: i32 = 42;
    let y = 3.14_f64; // 타입 접미사
    let is_cool = true;
    println!("  x={x}, y={y}, is_cool={is_cool}");

    // 튜플과 배열
    let point = (10, 20);
    let colors = ["red", "green", "blue"];
    println!("  point = ({}, {})", point.0, point.1);
    println!("  첫번째 색 = {}", colors[0]);

    // if 표현식 (값을 반환함!)
    let temp = 35;
    let feeling = if temp > 30 { "덥다" } else { "괜찮다" };
    println!("  {temp}도 -> {feeling}");

    // for 루프
    let sum: i32 = (1..=10).sum();
    println!("  1~10 합계 = {sum}");
}
