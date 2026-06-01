use crate::models::{Document, Priority, Task, TaskStatus};
use chrono::NaiveDate;
use dioxus::prelude::*;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions, SqliteRow};
use sqlx::Row;
use std::sync::Mutex;
use tracing::{error, info};

use super::auth::hash_password;

// ── Database pool ───────────────────────────────────────────────────

static POOL: Mutex<Option<SqlitePool>> = Mutex::new(None);

pub fn map_err(e: impl ToString) -> ServerFnError {
    let msg = e.to_string();
    error!("{msg}");
    ServerFnError::ServerError {
        message: msg,
        code: 500,
        details: None,
    }
}

/// Lazy init: creates the pool, runs migrations, and seeds data — only on first call.
pub async fn get_pool() -> Result<SqlitePool, ServerFnError> {
    {
        let guard = POOL.lock().unwrap();
        if let Some(ref pool) = *guard {
            return Ok(pool.clone());
        }
    }

    let db_path_str = std::env::var("DB_PATH").unwrap_or_else(|_| {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("todo_list.db")
            .display()
            .to_string()
    });
    let db_path = std::path::PathBuf::from(&db_path_str);
    // ensure parent directory exists (necessary when DB_PATH is a custom path)
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent).ok();
    }
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
    Ok(guard.as_ref().unwrap().clone())
}

// ── Migrations & seeding ────────────────────────────────────────────

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

    let _ = sqlx::query("ALTER TABLE tasks ADD COLUMN start_date TEXT")
        .execute(pool)
        .await;
    let _ = sqlx::query("ALTER TABLE tasks ADD COLUMN due_date TEXT")
        .execute(pool)
        .await;
    let _ = sqlx::query("ALTER TABLE tasks ADD COLUMN deleted INTEGER NOT NULL DEFAULT 0")
        .execute(pool)
        .await;
    let _ = sqlx::query("ALTER TABLE tasks ADD COLUMN archived INTEGER NOT NULL DEFAULT 0")
        .execute(pool)
        .await;

    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM tasks")
        .fetch_one(pool)
        .await
        .map_err(|e| map_err(e))?;

    if count.0 == 0 {
        info!("seeding {} default tasks", default_tasks().len());
        for task in default_tasks() {
            sqlx::query(
                "INSERT INTO tasks (id, title, description, priority, status, assignee, completed_by, start_date, due_date, deleted, archived)
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
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
            .bind(task.deleted as i64)
            .bind(task.archived as i64)
            .execute(pool)
            .await
            .map_err(|e| map_err(e))?;
        }
    }

    let null_dates: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM tasks WHERE start_date IS NULL OR due_date IS NULL",
    )
    .fetch_one(pool)
    .await
    .map_err(|e| map_err(e))?;

    if null_dates.0 > 0 {
        info!("backfilling dates on {n} existing tasks", n = null_dates.0);
        for task in default_tasks() {
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

    let _ = sqlx::query("ALTER TABLE users ADD COLUMN gender TEXT")
        .execute(pool)
        .await;
    let _ = sqlx::query("ALTER TABLE users ADD COLUMN age INTEGER")
        .execute(pool)
        .await;
    let _ = sqlx::query("ALTER TABLE users ADD COLUMN job_title TEXT")
        .execute(pool)
        .await;
    let _ = sqlx::query("ALTER TABLE users ADD COLUMN email TEXT")
        .execute(pool)
        .await;
    let _ = sqlx::query("ALTER TABLE users ADD COLUMN role TEXT NOT NULL DEFAULT 'user'")
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

        // Seed Administrator account
        let admin_hash = hash_password("password123");
        let _ = sqlx::query(
            "INSERT OR IGNORE INTO users (username, password_hash, role) VALUES ('Administrator', ?, 'admin')",
        )
        .bind(&admin_hash)
        .execute(pool)
        .await;
    }

    // ── Documents table ─────────────────────────

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS documents (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            username TEXT NOT NULL,
            title TEXT NOT NULL DEFAULT 'Untitled',
            content TEXT NOT NULL DEFAULT '',
            updated_at TEXT NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users(id)
        )",
    )
    .execute(pool)
    .await
    .map_err(|e| map_err(e))?;

    // ── Indexes ───────────────────────────────

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_tasks_deleted_archived ON tasks(deleted, archived)")
        .execute(pool)
        .await
        .map_err(|e| map_err(e))?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_tasks_status ON tasks(status)")
        .execute(pool)
        .await
        .map_err(|e| map_err(e))?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_tasks_assignee ON tasks(assignee)")
        .execute(pool)
        .await
        .map_err(|e| map_err(e))?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_documents_user_id_updated ON documents(user_id, updated_at)")
        .execute(pool)
        .await
        .map_err(|e| map_err(e))?;

    Ok(())
}

// ── Helpers ──────────────────────────────────────────────────────────

pub fn now_iso() -> String {
    chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string()
}

// ── Type conversions ────────────────────────────────────────────────

pub fn priority_to_str(p: &Priority) -> &'static str {
    match p {
        Priority::Low => "Low",
        Priority::Medium => "Medium",
        Priority::High => "High",
        Priority::Urgent => "Urgent",
    }
}

fn str_to_priority(s: &str) -> Priority {
    match s {
        "Low" => Priority::Low,
        "Medium" => Priority::Medium,
        "High" => Priority::High,
        "Urgent" => Priority::Urgent,
        _ => Priority::Medium,
    }
}

pub fn status_to_str(s: &TaskStatus) -> &'static str {
    match s {
        TaskStatus::Todo => "Todo",
        TaskStatus::InProgress => "InProgress",
        TaskStatus::InReview => "InReview",
        TaskStatus::Done => "Done",
    }
}

fn str_to_status(s: &str) -> TaskStatus {
    match s {
        "Todo" => TaskStatus::Todo,
        "InProgress" => TaskStatus::InProgress,
        "InReview" => TaskStatus::InReview,
        "Done" => TaskStatus::Done,
        _ => TaskStatus::Todo,
    }
}

// ── Row mapping ─────────────────────────────────────────────────────

pub fn row_to_task(row: &SqliteRow) -> Task {
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
        deleted: row.get::<i64, _>("deleted") != 0,
        archived: row.get::<i64, _>("archived") != 0,
    }
}

pub fn row_to_document(row: &SqliteRow) -> Document {
    Document {
        id: row.get("id"),
        user_id: row.get("user_id"),
        username: row.get("username"),
        title: row.get("title"),
        content: row.get("content"),
        updated_at: row.get("updated_at"),
    }
}

// ── Seed data ───────────────────────────────────────────────────────

fn default_tasks() -> Vec<Task> {
    let base = NaiveDate::from_ymd_opt(2026, 6, 1).unwrap();
    let d = |offset: i64| base.checked_add_signed(chrono::Duration::days(offset));
    vec![
        Task {
            id: 1,
            title: "Design project architecture".into(),
            description: "Define module boundaries, data flow, and component tree for the application.".into(),
            priority: Priority::High,
            status: TaskStatus::Done,
            assignee: Some("Alice".into()),
            completed_by: Some("Alice".into()),
            start_date: d(0),
            due_date: d(4),
            deleted: false,
            archived: false,
        },
        Task {
            id: 2,
            title: "Set up CI/CD pipeline".into(),
            description: "Configure GitHub Actions for automated builds, tests, and deployments.".into(),
            priority: Priority::High,
            status: TaskStatus::InProgress,
            assignee: Some("Bob".into()),
            completed_by: None,
            start_date: d(2),
            due_date: d(9),
            deleted: false,
            archived: false,
        },
        Task {
            id: 3,
            title: "Implement user authentication".into(),
            description: "Add login, registration, and session management with JWT tokens.".into(),
            priority: Priority::Urgent,
            status: TaskStatus::Todo,
            assignee: None,
            completed_by: None,
            start_date: d(7),
            due_date: d(15),
            deleted: false,
            archived: false,
        },
        Task {
            id: 4,
            title: "Write API documentation".into(),
            description: "Document all REST endpoints with request/response examples using OpenAPI spec.".into(),
            priority: Priority::Medium,
            status: TaskStatus::InReview,
            assignee: Some("Charlie".into()),
            completed_by: None,
            start_date: d(11),
            due_date: d(18),
            deleted: false,
            archived: false,
        },
        Task {
            id: 5,
            title: "Optimize database queries".into(),
            description: "Profile slow queries, add indexes, and implement query caching layer.".into(),
            priority: Priority::Low,
            status: TaskStatus::Todo,
            assignee: None,
            completed_by: None,
            start_date: d(14),
            due_date: d(22),
            deleted: false,
            archived: false,
        },
        Task {
            id: 6,
            title: "Add dark mode support".into(),
            description: "Implement theme switching with CSS variables and system preference detection.".into(),
            priority: Priority::Low,
            status: TaskStatus::Todo,
            assignee: Some("Alice".into()),
            completed_by: None,
            start_date: d(17),
            due_date: d(19),
            deleted: false,
            archived: false,
        },
    ]
}
