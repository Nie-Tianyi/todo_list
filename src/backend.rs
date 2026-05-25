use crate::models::{LoginResponse, Priority, Task, User};
use chrono::NaiveDate;
use dioxus::prelude::*;
#[cfg(feature = "server")]
use crate::models::TaskStatus;
#[cfg(feature = "server")]
use tracing::info;

// ── Server functions ──────────────────────────────

#[get("/api/tasks", auth: crate::auth::AuthSession)]
pub async fn get_tasks() -> Result<Vec<Task>, ServerFnError> {
    info!("GET /api/tasks user={}", auth.user.username);
    let pool = crate::server::get_pool().await?;
    let rows = sqlx::query(
        "SELECT id, title, description, priority, status, assignee, completed_by, start_date, due_date FROM tasks ORDER BY id",
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| crate::server::map_err(e))?;

    Ok(rows.iter().map(crate::server::row_to_task).collect())
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
    let pool = crate::server::get_pool().await?;
    let result = sqlx::query(
        "INSERT INTO tasks (title, description, priority, status, start_date, due_date) VALUES (?, ?, ?, 'Todo', ?, ?)",
    )
    .bind(&title)
    .bind(&description)
    .bind(crate::server::priority_to_str(&priority))
    .bind(start_date.map(|d| d.format("%Y-%m-%d").to_string()))
    .bind(due_date.map(|d| d.format("%Y-%m-%d").to_string()))
    .execute(&pool)
    .await
    .map_err(|e| crate::server::map_err(e))?;

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
    let pool = crate::server::get_pool().await?;
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
    .bind(crate::server::priority_to_str(&task.priority))
    .bind(crate::server::status_to_str(&task.status))
    .bind(&task.assignee)
    .bind(&task.completed_by)
    .bind(task.start_date.map(|d| d.format("%Y-%m-%d").to_string()))
    .bind(task.due_date.map(|d| d.format("%Y-%m-%d").to_string()))
    .execute(&pool)
    .await
    .map_err(|e| crate::server::map_err(e))?;

    Ok(())
}

#[get("/api/team/members", auth: crate::auth::AuthSession)]
pub async fn get_team_members() -> Result<Vec<User>, ServerFnError> {
    info!("GET /api/team/members user={}", auth.user.username);
    let pool = crate::server::get_pool().await?;
    let rows = sqlx::query_as("SELECT id, username, gender, age, job_title, email FROM users ORDER BY id")
        .fetch_all(&pool)
        .await
        .map_err(|e| crate::server::map_err(e))?;

    Ok(rows
        .into_iter()
        .map(|(id, username, gender, age, job_title, email)| User {
            id,
            username,
            gender,
            age,
            job_title,
            email,
        })
        .collect())
}

#[post("/api/login")]
pub async fn login(username: String, password: String) -> Result<LoginResponse, ServerFnError> {
    info!("POST /api/login username={username:?}");
    let pool = crate::server::get_pool().await?;

    let row: Option<(i32, String, String, Option<String>, Option<i32>, Option<String>, Option<String>)> = sqlx::query_as(
        "SELECT id, username, password_hash, gender, age, job_title, email FROM users WHERE username = ?",
    )
    .bind(&username)
    .fetch_optional(&pool)
    .await
    .map_err(|e| crate::server::map_err(e))?;

    let (id, name, hash, gender, age, job_title, email) = row.ok_or_else(|| {
        ServerFnError::ServerError {
            message: "Invalid username or password".into(),
            code: 401,
            details: None,
        }
    })?;

    if !crate::server::verify_password(&password, &hash) {
        return Err(ServerFnError::ServerError {
            message: "Invalid username or password".into(),
            code: 401,
            details: None,
        });
    }

    let user = User { id, username: name, gender, age, job_title, email };
    let token = crate::server::create_token(&user)
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
    email: Option<String>,
) -> Result<User, ServerFnError> {
    info!("POST /api/profile/update user={}", auth.user.username);
    let pool = crate::server::get_pool().await?;

    let gender = gender.filter(|s| !s.trim().is_empty());
    let job_title = job_title.filter(|s| !s.trim().is_empty());
    let email = email.filter(|s| !s.trim().is_empty());

    sqlx::query(
        "UPDATE users SET gender = ?, age = ?, job_title = ?, email = ? WHERE id = ?",
    )
    .bind(&gender)
    .bind(age)
    .bind(&job_title)
    .bind(&email)
    .bind(auth.user.id)
    .execute(&pool)
    .await
    .map_err(|e| crate::server::map_err(e))?;

    Ok(User {
        id: auth.user.id,
        username: auth.user.username,
        gender,
        age,
        job_title,
        email,
    })
}

#[post("/api/register")]
pub async fn register(
    username: String,
    password: String,
    gender: Option<String>,
    age: Option<i32>,
    job_title: Option<String>,
    email: Option<String>,
) -> Result<LoginResponse, ServerFnError> {
    info!("POST /api/register username={username:?}");
    let pool = crate::server::get_pool().await?;

    let existing: Option<(i64,)> = sqlx::query_as(
        "SELECT COUNT(*) FROM users WHERE username = ?",
    )
    .bind(&username)
    .fetch_optional(&pool)
    .await
    .map_err(|e| crate::server::map_err(e))?;

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

    let hash = crate::server::hash_password(&password);

    let result = sqlx::query(
        "INSERT INTO users (username, password_hash, gender, age, job_title, email) VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(&username)
    .bind(&hash)
    .bind(&gender)
    .bind(age)
    .bind(&job_title)
    .bind(&email)
    .execute(&pool)
    .await
    .map_err(|e| crate::server::map_err(e))?;

    let user = User {
        id: result.last_insert_rowid() as i32,
        username,
        gender,
        age,
        job_title,
        email,
    };

    let token = crate::server::create_token(&user)
        .map_err(|e| ServerFnError::ServerError {
            message: format!("Failed to create token: {e}"),
            code: 500,
            details: None,
        })?;

    Ok(LoginResponse { user, token })
}
