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
            TaskStatus::Todo => "border-t-blue-400 bg-blue-50/50",
            TaskStatus::InProgress => "border-t-amber-400 bg-amber-50/50",
            TaskStatus::InReview => "border-t-purple-400 bg-purple-50/50",
            TaskStatus::Done => "border-t-green-400 bg-green-50/50",
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
            Priority::Low => "bg-gray-100 text-gray-600",
            Priority::Medium => "bg-blue-100 text-blue-600",
            Priority::High => "bg-orange-100 text-orange-600",
            Priority::Urgent => "bg-red-100 text-red-600",
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
}

#[cfg(feature = "server")]
pub fn default_tasks() -> Vec<Task> {
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
        },
    ]
}
