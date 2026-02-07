# Claude 프로젝트 컨텍스트 — Rust Playground

이 문서는 이 저장소에서 작업할 때 Claude가 참고할 프로젝트 규칙과 구조입니다.

## 프로젝트 개요

- **이름**: rust-playground
- **목적**: Rust 학습용 예제 모음 (기본 문법 → 소유권 → 비동기 → Axum 웹 서버)
- **언어**: Rust (Edition 2021), 주석·출력은 한국어

## 디렉터리 구조

```
rust-playground/
├── Cargo.toml          # 패키지 및 의존성 (tokio, axum, serde, serde_json), edition = "2021"
├── README.md           # 프로젝트 소개·실행·구조·웹 API
├── CLAUDE.md            # 본 문서 (AI 컨텍스트)
├── .cursorrules        # Cursor 규칙 요약
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
│       ├── async_await.rs
│       └── web_server.rs   # Axum, 포트 3001, GET / → HTML (소개·예제 목록·Rust·용도)
├── docs/               # PDCA/문서 등 (선택)
└── .claude/            # Claude 관련 로컬 설정
```

## 웹 서버 (9번, web_server.rs)

- **포트**: 3001 (`http://localhost:3001`)
- **GET /**  
  HTML 페이지: 프로젝트 용도, 1~9 예제 목록, Rust란?, Rust로 할 수 있는 것(용도), `/todos` 링크
- **GET /todos** — 할일 목록 JSON  
- **POST /todos** — 할일 추가 (JSON body)  
- **GET /todos/:id/done** — 완료 처리

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
- 웹 페이지(HTML) 내용 변경 시 프로젝트 소개·예제 목록·Rust 용도가 README/CLAUDE와 어긋나지 않게 유지.

## 참고

- 실행: `cargo run`
- 포맷: `cargo fmt`
- 검사: `cargo check` / `cargo clippy`
