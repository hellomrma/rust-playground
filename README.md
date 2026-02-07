# Rust Playground

Rust 언어 학습을 위한 예제 모음 프로젝트입니다.

## 개요

기본 문법부터 비동기·웹 서버까지 단계별 예제를 실행하며 Rust를 익힐 수 있습니다.

## 요구 사항

- [Rust](https://www.rust-lang.org/) (rustup 권장)
- `cargo` (Rust 설치 시 포함)

## 실행 방법

```bash
# 프로젝트 루트에서
cargo run
```

개별 예제만 실행하려면 해당 모듈을 `main.rs`에서 주석 처리하거나, 예제 바이너리를 추가해 사용할 수 있습니다.

## 프로젝트 구조

```
src/
├── main.rs              # 진입점, 모든 예제 순차 실행
└── examples/
    ├── mod.rs
    ├── basics.rs        # 1. 기본 문법
    ├── ownership.rs     # 2. 소유권(Ownership)
    ├── pattern_matching.rs  # 3. 패턴 매칭
    ├── error_handling.rs   # 4. 에러 처리
    ├── structs_traits.rs   # 5. 구조체와 트레이트
    ├── collections.rs     # 6. 컬렉션과 이터레이터
    ├── concurrency.rs     # 7. 동시성
    ├── async_await.rs     # 8. Async/Await
    └── web_server.rs     # 9. 웹 서버 (Axum)
```

## 예제 목록

| 순서 | 모듈 | 설명 |
|------|------|------|
| 1 | basics | 변수, 함수, 제어문 등 기본 문법 |
| 2 | ownership | 소유권, 참조, 라이프타임 개념 |
| 3 | pattern_matching | match, if let, 패턴 |
| 4 | error_handling | Result, Option, ? 연산자 |
| 5 | structs_traits | 구조체, impl, 트레이트 |
| 6 | collections | Vec, HashMap, 이터레이터 |
| 7 | concurrency | 스레드, Mutex, 채널 |
| 8 | async_await | Future, tokio 기초 |
| 9 | web_server | Axum으로 간단한 HTTP 서버 |

## 의존성

- **tokio** – 비동기 런타임
- **axum** – 웹 프레임워크
- **serde** / **serde_json** – 직렬화

## 라이선스

MIT (또는 프로젝트에 맞게 수정하세요.)
