// proj_03_web_api_axum — REST API with Axum + Tokio + sqlx (in-memory SQLite).
//
// Run the server:
//   cargo run --bin proj_03_web_api_axum
//   # then:
//   curl localhost:3000/health
//   curl localhost:3000/tasks
//   curl -X POST localhost:3000/tasks -H "content-type: application/json" -d "{\"text\":\"buy milk\"}"
//
// Run a self-contained check (no HTTP client needed):
//   cargo run --bin proj_03_web_api_axum -- --selftest
//
// Uses in-memory SQLite so it runs with NO database server or DATABASE_URL.
use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::get,
};
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};
use std::env;

#[derive(Serialize)]
struct Task {
    id: i64,
    text: String,
    done: bool,
}

#[derive(Deserialize)]
struct NewTask {
    text: String,
}

// App state shared across handlers: the DB connection pool (cheaply cloneable).
#[derive(Clone)]
struct AppState {
    pool: SqlitePool,
}

// Create the pool and the schema.
async fn init_db() -> anyhow::Result<SqlitePool> {
    let pool = SqlitePool::connect("sqlite::memory:").await?;
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS tasks (
            id   INTEGER PRIMARY KEY AUTOINCREMENT,
            text TEXT NOT NULL,
            done INTEGER NOT NULL DEFAULT 0
        )",
    )
    .execute(&pool)
    .await?;
    Ok(pool)
}

async fn health() -> &'static str {
    "ok"
}

// GET /tasks -> list all tasks.
async fn list_tasks(State(state): State<AppState>) -> Result<Json<Vec<Task>>, StatusCode> {
    let rows = sqlx::query("SELECT id, text, done FROM tasks ORDER BY id")
        .fetch_all(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let tasks = rows
        .into_iter()
        .map(|r| Task {
            id: r.get::<i64, _>("id"),
            text: r.get::<String, _>("text"),
            done: r.get::<i64, _>("done") != 0,
        })
        .collect();
    Ok(Json(tasks))
}

// POST /tasks -> create a task, return it.
async fn create_task(
    State(state): State<AppState>,
    Json(input): Json<NewTask>,
) -> Result<(StatusCode, Json<Task>), StatusCode> {
    let id = sqlx::query("INSERT INTO tasks (text) VALUES (?)")
        .bind(&input.text)
        .execute(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .last_insert_rowid();

    let task = Task { id, text: input.text, done: false };
    Ok((StatusCode::CREATED, Json(task)))
}

fn app(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/tasks", get(list_tasks).post(create_task))
        .with_state(state)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = init_db().await?;
    let state = AppState { pool };

    // Self-test path: exercise the DB layer directly and exit (for CI/verification).
    if env::args().any(|a| a == "--selftest") {
        selftest(&state).await?;
        return Ok(());
    }

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    println!("listening on http://127.0.0.1:3000");
    axum::serve(listener, app(state)).await?;
    Ok(())
}

// Insert two rows and read them back, printing results.
async fn selftest(state: &AppState) -> anyhow::Result<()> {
    for t in ["buy milk", "write rust"] {
        sqlx::query("INSERT INTO tasks (text) VALUES (?)")
            .bind(t)
            .execute(&state.pool)
            .await?;
    }
    let rows = sqlx::query("SELECT id, text, done FROM tasks ORDER BY id")
        .fetch_all(&state.pool)
        .await?;
    println!("selftest: {} tasks", rows.len());
    for r in rows {
        println!(
            "  #{} {} (done={})",
            r.get::<i64, _>("id"),
            r.get::<String, _>("text"),
            r.get::<i64, _>("done")
        );
    }
    Ok(())
}
