use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TaskStatus {
    Todo,
    InProgress,
    InReview,
    Done,
}

impl TaskStatus {
    pub fn label(&self) -> &'static str {
        match self {
            TaskStatus::Todo => "Todo",
            TaskStatus::InProgress => "In Progress",
            TaskStatus::InReview => "In Review",
            TaskStatus::Done => "Done",
        }
    }

    pub fn all() -> [TaskStatus; 4] {
        [
            TaskStatus::Todo,
            TaskStatus::InProgress,
            TaskStatus::InReview,
            TaskStatus::Done,
        ]
    }

    pub fn previous(&self) -> Option<TaskStatus> {
        match self {
            TaskStatus::Todo => None,
            TaskStatus::InProgress => Some(TaskStatus::Todo),
            TaskStatus::InReview => Some(TaskStatus::InProgress),
            TaskStatus::Done => Some(TaskStatus::InReview),
        }
    }

    pub fn next(&self) -> Option<TaskStatus> {
        match self {
            TaskStatus::Todo => Some(TaskStatus::InProgress),
            TaskStatus::InProgress => Some(TaskStatus::InReview),
            TaskStatus::InReview => Some(TaskStatus::Done),
            TaskStatus::Done => None,
        }
    }

    pub fn column_color(&self) -> &'static str {
        match self {
            TaskStatus::Todo => "border-t-blue-400 bg-blue-50/50 dark:border-t-blue-500 dark:bg-blue-950/30",
            TaskStatus::InProgress => "border-t-amber-400 bg-amber-50/50 dark:border-t-amber-500 dark:bg-amber-950/30",
            TaskStatus::InReview => "border-t-purple-400 bg-purple-50/50 dark:border-t-purple-500 dark:bg-purple-950/30",
            TaskStatus::Done => "border-t-green-400 bg-green-50/50 dark:border-t-green-500 dark:bg-green-950/30",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
    Urgent,
}

impl Priority {
    pub fn label(&self) -> &'static str {
        match self {
            Priority::Low => "Low",
            Priority::Medium => "Medium",
            Priority::High => "High",
            Priority::Urgent => "Urgent",
        }
    }

    pub fn badge_classes(&self) -> &'static str {
        match self {
            Priority::Low => "bg-gray-100 text-gray-600 dark:bg-gray-700 dark:text-gray-300",
            Priority::Medium => "bg-blue-100 text-blue-600 dark:bg-blue-900/30 dark:text-blue-400",
            Priority::High => "bg-orange-100 text-orange-600 dark:bg-orange-900/30 dark:text-orange-400",
            Priority::Urgent => "bg-red-100 text-red-600 dark:bg-red-900/30 dark:text-red-400",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub gender: Option<String>,
    pub age: Option<i32>,
    pub job_title: Option<String>,
    pub email: Option<String>,
    pub role: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoginResponse {
    pub user: User,
    pub token: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub priority: Priority,
    pub status: TaskStatus,
    pub assignee: Option<String>,
    pub completed_by: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub due_date: Option<NaiveDate>,
    pub deleted: bool,
    pub archived: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Document {
    pub id: i32,
    pub user_id: i32,
    pub username: String,
    pub title: String,
    pub content: String,
    pub updated_at: String,
}

// `default_tasks()` moved to `server.rs`
