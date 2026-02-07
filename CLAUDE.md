# Claude 프로젝트 컨텍스트 — Rust Playground

이 문서는 이 저장소에서 작업할 때 Claude가 참고할 프로젝트 규칙과 구조입니다.

## 프로젝트 개요

- **이름**: rust-playground
- **목적**: Rust 학습용 예제 모음 (기본 문법 → 소유권 → 비동기 → Axum 웹 서버)
- **언어**: Rust (Edition 2024), 주석·출력은 한국어

## 디렉터리 구조

```
rust-playground/
├── Cargo.toml          # 패키지 및 의존성 (tokio, axum, serde, serde_json)
├── src/
│   ├── main.rs         # 진입점, examples::* 모듈을 순서대로 실행
│   └── examples/       # 학습용 예제 모듈
│       ├── mod.rs      # 하위 모듈 pub 선언
│       ├── basics.rs
│       ├── ownership.rs
│       ├── pattern_matching.rs
│       ├── error_handling.rs
│       ├── structs_traits.rs
│       ├── collections.rs
│       ├── concurrency.rs
│       ├── async_await.rs
│       └── web_server.rs
├── docs/               # PDCA/문서 등 (선택)
└── .claude/             # Claude 관련 설정
```

## 코딩 규칙

- **스타일**: `cargo fmt` 기준 (표준 Rust 포매팅).
- **예제 모듈**: 각 파일은 `pub fn run()` 또는 `pub async fn run()` 하나를 노출하고, `main.rs`에서만 호출.
- **주석/출력**: 사용자 대상 설명과 `println!` 등은 한국어 사용.
- **에러 처리**: 예제 내에서는 `unwrap()`/`expect()` 허용. 라이브러리/재사용 코드는 `Result`/`Option`과 `?` 사용 권장.
- **비동기**: `#[tokio::main]` 사용, 필요 시 `tokio::spawn` 등 tokio API 활용.

## 작업 시 유의사항

- 새 예제 추가 시 `src/examples/mod.rs`에 `pub mod 이름;` 추가하고, `main.rs`에서 호출 순서에 맞게 `examples::이름::run()` 호출.
- 의존성 추가는 `Cargo.toml`의 `[dependencies]`에만 추가. 버전은 기존 스타일 유지.
- 문서/주석 수정 시 README.md, CLAUDE.md, .cursorrules와 일치하도록 유지.

## 참고

- 실행: `cargo run`
- 포맷: `cargo fmt`
- 검사: `cargo check` / `cargo clippy`
