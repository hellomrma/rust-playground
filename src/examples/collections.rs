use std::collections::HashMap;

pub fn run() {
    // Vec - 동적 배열
    let mut scores = vec![85, 92, 78, 95, 88];
    scores.push(100);
    println!("  점수들: {scores:?}");

    // 이터레이터 체인 - 함수형 프로그래밍 스타일
    let avg: f64 = scores.iter().sum::<i32>() as f64 / scores.len() as f64;
    println!("  평균: {avg:.1}");

    let high_scores: Vec<&i32> = scores.iter().filter(|&&s| s >= 90).collect();
    println!("  90점 이상: {high_scores:?}");

    // map으로 변환
    let doubled: Vec<i32> = scores.iter().map(|s| s * 2).collect();
    println!("  2배: {doubled:?}");

    // HashMap - 딕셔너리/맵
    let mut fruits: HashMap<&str, i32> = HashMap::new();
    fruits.insert("사과", 3);
    fruits.insert("바나나", 5);
    fruits.insert("포도", 2);

    for (name, count) in &fruits {
        println!("  {name}: {count}개");
    }

    // entry API - 없으면 추가
    fruits.entry("사과").or_insert(0);
    fruits.entry("망고").or_insert(1);
    println!("  망고: {}개", fruits["망고"]);

    // 단어 빈도 세기
    let text = "the cat sat on the mat the cat";
    let mut word_count: HashMap<&str, i32> = HashMap::new();
    for word in text.split_whitespace() {
        *word_count.entry(word).or_insert(0) += 1;
    }
    println!("  단어 빈도: {word_count:?}");
}
