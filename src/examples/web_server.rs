use axum::{
    Router,
    Json,
    response::Html,
    extract::{Path, State},
    routing::get,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// --- 데이터 모델 ---

#[derive(Serialize, Deserialize, Clone)]
struct Todo {
    id: usize,
    title: String,
    done: bool,
}

#[derive(Deserialize)]
struct CreateTodo {
    title: String,
}

// 앱 상태 (메모리 DB 역할)
type AppState = Arc<Mutex<Vec<Todo>>>;

// --- 핸들러 함수들 ---

// GET / - 프로젝트 소개 + Rust 소개
async fn hello() -> Html<&'static str> {
    Html(r#"
<!DOCTYPE html>
<html lang="ko">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Rust Playground</title>
  <style>
    * { box-sizing: border-box; }
    body { font-family: 'Segoe UI', system-ui, sans-serif; max-width: 720px; margin: 2rem auto; padding: 0 1rem; line-height: 1.6; color: #1a1a1a; }
    h1 { color: #ce422b; font-size: 1.75rem; border-bottom: 2px solid #ce422b; padding-bottom: 0.5rem; }
    h2 { color: #333; font-size: 1.2rem; margin-top: 1.5rem; }
    p { margin: 0.5rem 0; }
    ul { margin: 0.5rem 0; padding-left: 1.5rem; }
    .muted { color: #666; font-size: 0.95rem; }
    a { color: #ce422b; }
    code { background: #f4f4f4; padding: 0.15em 0.4em; border-radius: 4px; font-size: 0.9em; }
    .example-list { margin: 0.5rem 0; padding-left: 1.5rem; }
    .example-list li { margin: 0.4rem 0; }
    .use-list { margin: 0.5rem 0; padding-left: 1.5rem; }
    .use-list li { margin: 0.35rem 0; }
  </style>
</head>
<body>
  <h1>🦀 Rust Playground</h1>

  <h2>이 프로젝트는 무슨 용도인가요?</h2>
  <p><strong>Rust 학습용 예제 모음</strong>입니다. 기본 문법, 소유권, 패턴 매칭, 에러 처리, 구조체·트레이트, 컬렉션, 동시성, async/await부터 이 웹 서버(Axum)까지, 단계별로 실행해 보며 Rust를 익힐 수 있게 만들었습니다.</p>
  <p class="muted">실행: <code>cargo run</code> → 1~9번 예제가 순서대로 실행되고, 마지막에 이 웹서버가 뜹니다.</p>

  <h2>1번부터 9번까지 예제 목록</h2>
  <ol class="example-list">
    <li><strong>기본 문법</strong> — 변수, 함수, 튜플, enum, 제어문(if/for), 기본 타입</li>
    <li><strong>소유권(Ownership)</strong> — Rust 핵심 개념: 소유권, 이동, 참조, 빌림</li>
    <li><strong>패턴 매칭</strong> — match, if let, Option/Result 패턴</li>
    <li><strong>에러 처리</strong> — Result, Option, ? 연산자, unwrap/expect</li>
    <li><strong>구조체와 트레이트</strong> — struct, impl, trait (OOP에 가까운 구조)</li>
    <li><strong>컬렉션과 이터레이터</strong> — Vec, HashMap, iterator, map/filter/collect</li>
    <li><strong>동시성(Concurrency)</strong> — 스레드, Mutex, 채널</li>
    <li><strong>Async/Await</strong> — Future, tokio, 비동기 작업·select</li>
    <li><strong>웹서버 (Axum)</strong> — 이 페이지가 뜨는 HTTP 서버 (라우팅, JSON API)</li>
  </ol>

  <h2>Rust란?</h2>
  <p>Rust는 <strong>안전성·성능·동시성</strong>을 중요하게 설계한 시스템 프로그래밍 언어입니다.</p>
  <ul>
    <li><strong>메모리 안전</strong> — 소유권·빌림·라이프타임으로 버퍼 오버플로우/댕글링 포인터 없이 컴파일 타임에 보장</li>
    <li><strong>성능</strong> — 가비지 컬렉터 없이 C/C++에 버금가는 제로 코스트 추상화</li>
    <li><strong>동시성</strong> — 데이터 레이스 방지를 타입 시스템으로 보장</li>
    <li><strong>도구</strong> — cargo, rustfmt, clippy, 문서화가 잘 갖춰져 있음</li>
  </ul>
  <h2>Rust로 할 수 있는 것 (용도)</h2>
  <p>Rust는 아래와 같은 분야에서 사용됩니다.</p>
  <ul class="use-list">
    <li><strong>웹 백엔드·API</strong> — Axum, Actix, Rocket 등으로 REST/GraphQL 서버, 마이크로서비스</li>
    <li><strong>CLI 도구</strong> — ripgrep, fd, bat, eza, bottom 등 터미널 유틸, 빌드/배포 스크립트</li>
    <li><strong>시스템 프로그래밍·OS</strong> — 커널, 드라이버, 리눅스 커널 모듈, Redox OS</li>
    <li><strong>임베디드·IoT</strong> — no_std, MCU, 센서·액추에이터 제어, 실시간 시스템</li>
    <li><strong>웹 프론트엔드 (WASM)</strong> — WebAssembly로 브라우저에서 실행, 게임·편집기·도구</li>
    <li><strong>게임·엔진</strong> — Bevy, Amethyst 등 게임 엔진, 서버/클라이언트 공통 로직</li>
    <li><strong>DevOps·인프라</strong> — Docker, Kubernetes 관련 도구, CI/CD, 모니터링 에이전트</li>
    <li><strong>데이터베이스·스토리지</strong> — DB 엔진, 인덱서, 캐시, 파일 시스템</li>
    <li><strong>네트워크·프록시</strong> — 로드밸런서, VPN, CDN 엣지, P2P·블록체인 노드</li>
    <li><strong>크로스플랫폼 앱</strong> — Tauri로 데스크톱 앱(웹 UI + Rust 백엔드)</li>
    <li><strong>크립토·블록체인</strong> — Solana, Polkadot, 많은 체인/노드 구현</li>
    <li><strong>ML·데이터 파이프라인</strong> — 수치 연산, 추론 엔진, ETL·스트리밍 처리</li>
  </ul>
  <p class="muted">공식: <a href="https://www.rust-lang.org/" target="_blank" rel="noopener">rust-lang.org</a></p>

  <p style="margin-top: 2rem;"><a href="/todos">할일 API 보기 (JSON)</a></p>
</body>
</html>
"#)
}

// GET /todos - 전체 목록
async fn list_todos(State(state): State<AppState>) -> Json<Vec<Todo>> {
    let todos = state.lock().unwrap();
    Json(todos.clone())
}

// POST /todos - 새 할일 추가
async fn create_todo(
    State(state): State<AppState>,
    Json(input): Json<CreateTodo>,
) -> Json<Todo> {
    let mut todos = state.lock().unwrap();
    let todo = Todo {
        id: todos.len() + 1,
        title: input.title,
        done: false,
    };
    todos.push(todo.clone());
    Json(todo)
}

// GET /todos/:id/done - 완료 처리
async fn complete_todo(
    State(state): State<AppState>,
    Path(id): Path<usize>,
) -> Json<serde_json::Value> {
    let mut todos = state.lock().unwrap();
    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
        todo.done = true;
        Json(serde_json::json!({"message": format!("'{}' 완료!", todo.title)}))
    } else {
        Json(serde_json::json!({"error": "할일을 찾을 수 없습니다"}))
    }
}

pub async fn run() {
    // 초기 데이터
    let state: AppState = Arc::new(Mutex::new(vec![
        Todo { id: 1, title: "Rust 배우기".to_string(), done: true },
        Todo { id: 2, title: "웹서버 만들기".to_string(), done: false },
    ]));

    // 라우터 구성
    let app = Router::new()
        .route("/", get(hello))
        .route("/todos", get(list_todos).post(create_todo))
        .route("/todos/{id}/done", get(complete_todo))
        .with_state(state);

    println!("  웹서버 시작: http://localhost:3001");
    println!("  API 엔드포인트:");
    println!("    GET  /           -> 프로젝트·Rust 소개 (HTML)");
    println!("    GET  /todos      -> 할일 목록");
    println!("    POST /todos      -> 할일 추가 (body: {{\"title\": \"...\"}})");
    println!("    GET  /todos/1/done -> 완료 처리");
    println!();
    println!("  테스트 방법 (다른 터미널에서):");
    println!("    curl http://localhost:3001/");
    println!("    curl http://localhost:3001/todos");
    println!("    curl -X POST -H \"Content-Type: application/json\" -d '{{\"title\":\"Axum 마스터\"}}' http://localhost:3001/todos");
    println!();
    println!("  종료: Ctrl+C");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
