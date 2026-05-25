use crate::models::{LoginResponse, Priority, Task, User};
#[cfg(feature = "server")]
use crate::models::TaskStatus;
use chrono::NaiveDate;
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

    // Gantt date columns — add if missing
    let _ = sqlx::query("ALTER TABLE tasks ADD COLUMN start_date TEXT")
        .execute(pool)
        .await;
    let _ = sqlx::query("ALTER TABLE tasks ADD COLUMN due_date TEXT")
        .execute(pool)
        .await;

    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM tasks")
        .fetch_one(pool)
        .await
        .map_err(|e| map_err(e))?;

    if count.0 == 0 {
        info!("seeding {} default tasks", crate::models::default_tasks().len());
        for task in crate::models::default_tasks() {
            sqlx::query(
                "INSERT INTO tasks (id, title, description, priority, status, assignee, completed_by, start_date, due_date)
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            )
            .bind(task.id as i64)
            .bind(&task.title)
            .bind(&task.description)
            .bind(priority_to_str(&task.priority))
            .bind(status_to_str(&task.status))
            .bind(&task.assignee)
            .bind(&task.completed_by)
            .bind(task.start_date.map(|d| d.format("%Y-%m-%d").to_string()))
            .bind(task.due_date.map(|d| d.format("%Y-%m-%d").to_string()))
            .execute(pool)
            .await
            .map_err(|e| map_err(e))?;
        }
    }

    // Backfill dates on existing tasks that lack them (pre-Gantt migration)
    let null_dates: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM tasks WHERE start_date IS NULL OR due_date IS NULL",
    )
    .fetch_one(pool)
    .await
    .map_err(|e| map_err(e))?;

    if null_dates.0 > 0 {
        info!("backfilling dates on {n} existing tasks", n = null_dates.0);
        for task in crate::models::default_tasks() {
            sqlx::query(
                "UPDATE tasks SET start_date = ?, due_date = ? WHERE id = ? AND start_date IS NULL",
            )
            .bind(task.start_date.map(|d| d.format("%Y-%m-%d").to_string()))
            .bind(task.due_date.map(|d| d.format("%Y-%m-%d").to_string()))
            .bind(task.id as i64)
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

    // Profile columns — add if missing (silently ignore "duplicate column" errors)
    let _ = sqlx::query("ALTER TABLE users ADD COLUMN gender TEXT")
        .execute(pool)
        .await;
    let _ = sqlx::query("ALTER TABLE users ADD COLUMN age INTEGER")
        .execute(pool)
        .await;
    let _ = sqlx::query("ALTER TABLE users ADD COLUMN job_title TEXT")
        .execute(pool)
        .await;

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
        start_date: row
            .get::<Option<String>, _>("start_date")
            .and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()),
        due_date: row
            .get::<Option<String>, _>("due_date")
            .and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()),
    }
}

// ── Server functions ──────────────────────────────

#[get("/api/tasks", auth: crate::auth::AuthSession)]
pub async fn get_tasks() -> Result<Vec<Task>, ServerFnError> {
    info!("GET /api/tasks user={}", auth.user.username);
    let pool = get_pool().await?;
    let rows = sqlx::query(
        "SELECT id, title, description, priority, status, assignee, completed_by, start_date, due_date FROM tasks ORDER BY id",
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
    start_date: Option<NaiveDate>,
    due_date: Option<NaiveDate>,
) -> Result<Task, ServerFnError> {
    info!("POST /api/tasks title={title:?} user={}", auth.user.username);
    let pool = get_pool().await?;
    let result = sqlx::query(
        "INSERT INTO tasks (title, description, priority, status, start_date, due_date) VALUES (?, ?, ?, 'Todo', ?, ?)",
    )
    .bind(&title)
    .bind(&description)
    .bind(priority_to_str(&priority))
    .bind(start_date.map(|d| d.format("%Y-%m-%d").to_string()))
    .bind(due_date.map(|d| d.format("%Y-%m-%d").to_string()))
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
        start_date,
        due_date,
    })
}

#[post("/api/tasks/save", auth: crate::auth::AuthSession)]
pub async fn save_task(task: Task) -> Result<(), ServerFnError> {
    info!("POST /api/tasks/save id={} user={}", task.id, auth.user.username);
    let pool = get_pool().await?;
    sqlx::query(
        "INSERT INTO tasks (id, title, description, priority, status, assignee, completed_by, start_date, due_date)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
         ON CONFLICT(id) DO UPDATE SET
             title = excluded.title,
             description = excluded.description,
             priority = excluded.priority,
             status = excluded.status,
             assignee = excluded.assignee,
             completed_by = excluded.completed_by,
             start_date = excluded.start_date,
             due_date = excluded.due_date",
    )
    .bind(task.id as i64)
    .bind(&task.title)
    .bind(&task.description)
    .bind(priority_to_str(&task.priority))
    .bind(status_to_str(&task.status))
    .bind(&task.assignee)
    .bind(&task.completed_by)
    .bind(task.start_date.map(|d| d.format("%Y-%m-%d").to_string()))
    .bind(task.due_date.map(|d| d.format("%Y-%m-%d").to_string()))
    .execute(&pool)
    .await
    .map_err(|e| map_err(e))?;

    Ok(())
}

#[post("/api/login")]
pub async fn login(username: String, password: String) -> Result<LoginResponse, ServerFnError> {
    info!("POST /api/login username={username:?}");
    let pool = get_pool().await?;

    let row: Option<(i32, String, String, Option<String>, Option<i32>, Option<String>)> = sqlx::query_as(
        "SELECT id, username, password_hash, gender, age, job_title FROM users WHERE username = ?",
    )
    .bind(&username)
    .fetch_optional(&pool)
    .await
    .map_err(|e| map_err(e))?;

    let (id, name, hash, gender, age, job_title) = row.ok_or_else(|| {
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

    let user = User { id, username: name, gender, age, job_title };
    let token = crate::auth::create_token(&user)
        .map_err(|e| ServerFnError::ServerError {
            message: format!("Failed to create token: {e}"),
            code: 500,
            details: None,
        })?;

    Ok(LoginResponse { user, token })
}

#[post("/api/profile/update", auth: crate::auth::AuthSession)]
pub async fn update_profile(
    gender: Option<String>,
    age: Option<i32>,
    job_title: Option<String>,
) -> Result<User, ServerFnError> {
    info!("POST /api/profile/update user={}", auth.user.username);
    let pool = get_pool().await?;

    let gender = gender.filter(|s| !s.trim().is_empty());
    let job_title = job_title.filter(|s| !s.trim().is_empty());

    sqlx::query(
        "UPDATE users SET gender = ?, age = ?, job_title = ? WHERE id = ?",
    )
    .bind(&gender)
    .bind(age)
    .bind(&job_title)
    .bind(auth.user.id)
    .execute(&pool)
    .await
    .map_err(|e| map_err(e))?;

    Ok(User {
        id: auth.user.id,
        username: auth.user.username,
        gender,
        age,
        job_title,
    })
}
#[post("/api/register")]
pub async fn register(
    username: String,
    password: String,
    gender: Option<String>,
    age: Option<i32>,
    job_title: Option<String>,
) -> Result<LoginResponse, ServerFnError> {
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
        "INSERT INTO users (username, password_hash, gender, age, job_title) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(&username)
    .bind(&hash)
    .bind(&gender)
    .bind(age)
    .bind(&job_title)
    .execute(&pool)
    .await
    .map_err(|e| map_err(e))?;

    let user = User {
        id: result.last_insert_rowid() as i32,
        username,
        gender,
        age,
        job_title,
    };

    let token = crate::auth::create_token(&user)
        .map_err(|e| ServerFnError::ServerError {
            message: format!("Failed to create token: {e}"),
            code: 500,
            details: None,
        })?;

    Ok(LoginResponse { user, token })
}
