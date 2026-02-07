pub fn run() {
    // match는 switch문의 강화 버전
    let number = 7;
    let description = match number {
        1 => "하나",
        2..=5 => "2에서 5 사이",
        6 | 7 => "6 또는 7",
        _ => "그 외", // _ 는 나머지 모든 경우
    };
    println!("  {number}은 {description}");

    // enum + match = 강력한 조합
    let coin = Coin::Quarter;
    println!("  동전 가치: {}원", coin_value(&coin));

    // Option - null 대신 사용 (null이 없음!)
    let items = vec![10, 20, 30];
    match items.get(1) {
        Some(value) => println!("  인덱스 1의 값: {value}"),
        None => println!("  값이 없음"),
    }

    // if let - 하나의 패턴만 확인할 때
    if let Some(first) = items.first() {
        println!("  첫번째 항목: {first}");
    }
}

#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn coin_value(coin: &Coin) -> u32 {
    match coin {
        Coin::Penny => 10,
        Coin::Nickel => 50,
        Coin::Dime => 100,
        Coin::Quarter => 250,
    }
}
