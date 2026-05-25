use crate::models::{LoginResponse, Priority, Task, User};
#[cfg(feature = "server")]
use crate::models::TaskStatus;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions, SqliteRow};
#[cfg(feature = "server")]
use sqlx::Row;
#[cfg(feature = "server")]
use tracing::{error, info};
#[cfg(feature = "server")]
use std::sync::Mutex;

#[cfg(feature = "server")]
static POOL: Mutex<Option<SqlitePool>> = Mutex::new(None);

#[cfg(feature = "server")]
fn map_err(e: impl ToString) -> ServerFnError {
    let msg = e.to_string();
    error!("{msg}");
    ServerFnError::ServerError {
        message: msg,
        code: 500,
        details: None,
    }
}

/// Lazy init: creates the pool, runs migrations, and seeds data — only on first call.
#[cfg(feature = "server")]
async fn get_pool() -> Result<SqlitePool, ServerFnError> {
    {
        let guard = POOL.lock().unwrap();
        if let Some(ref pool) = *guard {
            return Ok(pool.clone());
        }
    }

    let db_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("todo_list.db");
    let db_path_str = db_path.display().to_string();
    info!("connecting to database at: {db_path_str}");

    let options = SqliteConnectOptions::new()
        .filename(&db_path)
        .create_if_missing(true);
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
        .map_err(|e| map_err(e))?;

    info!("database connection established, running migrations");
    run_migrations(&pool).await?;

    let mut guard = POOL.lock().unwrap();
    if guard.is_none() {
        *guard = Some(pool.clone());
    }
    // If another request already set it, use theirs
    Ok(guard.as_ref().unwrap().clone())
}

#[cfg(feature = "server")]
async fn run_migrations(pool: &SqlitePool) -> Result<(), ServerFnError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            description TEXT NOT NULL DEFAULT '',
            priority TEXT NOT NULL DEFAULT 'Medium',
            status TEXT NOT NULL DEFAULT 'Todo',
            assignee TEXT,
            completed_by TEXT
        )",
    )
    .execute(pool)
    .await
    .map_err(|e| map_err(e))?;

    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM tasks")
        .fetch_one(pool)
        .await
        .map_err(|e| map_err(e))?;

    if count.0 == 0 {
        info!("seeding {} default tasks", crate::models::default_tasks().len());
        for task in crate::models::default_tasks() {
            sqlx::query(
                "INSERT INTO tasks (id, title, description, priority, status, assignee, completed_by)
                 VALUES (?, ?, ?, ?, ?, ?, ?)",
            )
            .bind(task.id as i64)
            .bind(&task.title)
            .bind(&task.description)
            .bind(priority_to_str(&task.priority))
            .bind(status_to_str(&task.status))
            .bind(&task.assignee)
            .bind(&task.completed_by)
            .execute(pool)
            .await
            .map_err(|e| map_err(e))?;
        }
    }

    // ── Users table ──────────────────────────────

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL
        )",
    )
    .execute(pool)
    .await
    .map_err(|e| map_err(e))?;

    let user_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(pool)
        .await
        .map_err(|e| map_err(e))?;

    if user_count.0 == 0 {
        info!("seeding default users");
        let default_password = "password123";
        let usernames = ["Alice", "Bob", "Charlie"];

        for username in usernames {
            let hash = hash_password(default_password);
            sqlx::query("INSERT INTO users (username, password_hash) VALUES (?, ?)")
                .bind(username)
                .bind(&hash)
                .execute(pool)
                .await
                .map_err(|e| map_err(e))?;
        }
    }

    Ok(())
}

#[cfg(feature = "server")]
fn hash_password(password: &str) -> String {
    use argon2::{
        password_hash::{PasswordHasher, SaltString},
        Argon2,
    };
    let salt = SaltString::generate(&mut rand::rngs::OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .expect("failed to hash password")
        .to_string()
}

#[cfg(feature = "server")]
fn verify_password(password: &str, hash: &str) -> bool {
    use argon2::{
        password_hash::{PasswordHash, PasswordVerifier},
        Argon2,
    };
    let Ok(parsed) = PasswordHash::new(hash) else {
        return false;
    };
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok()
}

#[cfg(feature = "server")]
fn priority_to_str(p: &Priority) -> &'static str {
    match p {
        Priority::Low => "Low",
        Priority::Medium => "Medium",
        Priority::High => "High",
        Priority::Urgent => "Urgent",
    }
}

#[cfg(feature = "server")]
fn str_to_priority(s: &str) -> Priority {
    match s {
        "Low" => Priority::Low,
        "Medium" => Priority::Medium,
        "High" => Priority::High,
        "Urgent" => Priority::Urgent,
        _ => Priority::Medium,
    }
}

#[cfg(feature = "server")]
fn status_to_str(s: &TaskStatus) -> &'static str {
    match s {
        TaskStatus::Todo => "Todo",
        TaskStatus::InProgress => "InProgress",
        TaskStatus::InReview => "InReview",
        TaskStatus::Done => "Done",
    }
}

#[cfg(feature = "server")]
fn str_to_status(s: &str) -> TaskStatus {
    match s {
        "Todo" => TaskStatus::Todo,
        "InProgress" => TaskStatus::InProgress,
        "InReview" => TaskStatus::InReview,
        "Done" => TaskStatus::Done,
        _ => TaskStatus::Todo,
    }
}

#[cfg(feature = "server")]
fn row_to_task(row: &SqliteRow) -> Task {
    Task {
        id: row.get::<i64, _>("id") as u32,
        title: row.get("title"),
        description: row.get("description"),
        priority: str_to_priority(&row.get::<String, _>("priority")),
        status: str_to_status(&row.get::<String, _>("status")),
        assignee: row.get("assignee"),
        completed_by: row.get("completed_by"),
    }
}

// ── Server functions ──────────────────────────────

#[get("/api/tasks", auth: crate::auth::AuthSession)]
pub async fn get_tasks() -> Result<Vec<Task>, ServerFnError> {
    info!("GET /api/tasks user={}", auth.user.username);
    let pool = get_pool().await?;
    let rows = sqlx::query(
        "SELECT id, title, description, priority, status, assignee, completed_by FROM tasks ORDER BY id",
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| map_err(e))?;

    Ok(rows.iter().map(row_to_task).collect())
}

#[post("/api/tasks", auth: crate::auth::AuthSession)]
pub async fn create_task(
    title: String,
    description: String,
    priority: Priority,
) -> Result<Task, ServerFnError> {
    info!("POST /api/tasks title={title:?} user={}", auth.user.username);
    let pool = get_pool().await?;
    let result = sqlx::query(
        "INSERT INTO tasks (title, description, priority, status) VALUES (?, ?, ?, 'Todo')",
    )
    .bind(&title)
    .bind(&description)
    .bind(priority_to_str(&priority))
    .execute(&pool)
    .await
    .map_err(|e| map_err(e))?;

    Ok(Task {
        id: result.last_insert_rowid() as u32,
        title,
        description,
        priority,
        status: TaskStatus::Todo,
        assignee: None,
        completed_by: None,
    })
}

#[post("/api/tasks/save", auth: crate::auth::AuthSession)]
pub async fn save_task(task: Task) -> Result<(), ServerFnError> {
    info!("POST /api/tasks/save id={} user={}", task.id, auth.user.username);
    let pool = get_pool().await?;
    sqlx::query(
        "INSERT INTO tasks (id, title, description, priority, status, assignee, completed_by)
         VALUES (?, ?, ?, ?, ?, ?, ?)
         ON CONFLICT(id) DO UPDATE SET
             title = excluded.title,
             description = excluded.description,
             priority = excluded.priority,
             status = excluded.status,
             assignee = excluded.assignee,
             completed_by = excluded.completed_by",
    )
    .bind(task.id as i64)
    .bind(&task.title)
    .bind(&task.description)
    .bind(priority_to_str(&task.priority))
    .bind(status_to_str(&task.status))
    .bind(&task.assignee)
    .bind(&task.completed_by)
    .execute(&pool)
    .await
    .map_err(|e| map_err(e))?;

    Ok(())
}

#[post("/api/login")]
pub async fn login(username: String, password: String) -> Result<LoginResponse, ServerFnError> {
    info!("POST /api/login username={username:?}");
    let pool = get_pool().await?;

    let row: Option<(i32, String, String)> = sqlx::query_as(
        "SELECT id, username, password_hash FROM users WHERE username = ?",
    )
    .bind(&username)
    .fetch_optional(&pool)
    .await
    .map_err(|e| map_err(e))?;

    let (id, name, hash) = row.ok_or_else(|| {
        ServerFnError::ServerError {
            message: "Invalid username or password".into(),
            code: 401,
            details: None,
        }
    })?;

    if !verify_password(&password, &hash) {
        return Err(ServerFnError::ServerError {
            message: "Invalid username or password".into(),
            code: 401,
            details: None,
        });
    }

    let user = User { id, username: name };
    let token = crate::auth::create_token(&user)
        .map_err(|e| ServerFnError::ServerError {
            message: format!("Failed to create token: {e}"),
            code: 500,
            details: None,
        })?;

    Ok(LoginResponse { user, token })
}

#[post("/api/register")]
pub async fn register(username: String, password: String) -> Result<LoginResponse, ServerFnError> {
    info!("POST /api/register username={username:?}");
    let pool = get_pool().await?;

    let existing: Option<(i64,)> = sqlx::query_as(
        "SELECT COUNT(*) FROM users WHERE username = ?",
    )
    .bind(&username)
    .fetch_optional(&pool)
    .await
    .map_err(|e| map_err(e))?;

    if let Some((count,)) = existing {
        if count > 0 {
            return Err(ServerFnError::ServerError {
                message: "Username already taken".into(),
                code: 409,
                details: None,
            });
        }
    }

    if username.trim().is_empty() {
        return Err(ServerFnError::ServerError {
            message: "Username cannot be empty".into(),
            code: 400,
            details: None,
        });
    }

    if password.len() < 6 {
        return Err(ServerFnError::ServerError {
            message: "Password must be at least 6 characters".into(),
            code: 400,
            details: None,
        });
    }

    let hash = hash_password(&password);

    let result = sqlx::query(
        "INSERT INTO users (username, password_hash) VALUES (?, ?)",
    )
    .bind(&username)
    .bind(&hash)
    .execute(&pool)
    .await
    .map_err(|e| map_err(e))?;

    let user = User {
        id: result.last_insert_rowid() as i32,
        username,
    };

    let token = crate::auth::create_token(&user)
        .map_err(|e| ServerFnError::ServerError {
            message: format!("Failed to create token: {e}"),
            code: 500,
            details: None,
        })?;

    Ok(LoginResponse { user, token })
}
