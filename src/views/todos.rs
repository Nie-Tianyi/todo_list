use crate::components::kanban_column::KanbanColumn;
use crate::components::task_form::TaskForm;
use crate::components::user_selector::UserSelector;
use crate::models::{default_tasks, Task, TaskStatus};
use dioxus::prelude::*;

#[cfg(target_arch = "wasm32")]
fn load_tasks() -> Vec<Task> {
    web_sys::window()
        .and_then(|w| w.local_storage().ok())
        .flatten()
        .and_then(|s| s.get_item("kanban_tasks").ok())
        .flatten()
        .and_then(|json| serde_json::from_str::<Vec<Task>>(&json).ok())
        .unwrap_or_else(default_tasks)
}

#[cfg(not(target_arch = "wasm32"))]
fn load_tasks() -> Vec<Task> {
    default_tasks()
}

#[cfg(target_arch = "wasm32")]
fn persist_tasks(tasks: &[Task]) {
    if let Some(storage) = web_sys::window()
        .and_then(|w| w.local_storage().ok())
        .flatten()
    {
        if let Ok(json) = serde_json::to_string(tasks) {
            let _ = storage.set_item("kanban_tasks", &json);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn persist_tasks(_tasks: &[Task]) {}

#[cfg(target_arch = "wasm32")]
fn load_current_user() -> String {
    web_sys::window()
        .and_then(|w| w.local_storage().ok())
        .flatten()
        .and_then(|s| s.get_item("kanban_user").ok())
        .flatten()
        .unwrap_or_default()
}

#[cfg(not(target_arch = "wasm32"))]
fn load_current_user() -> String {
    String::new()
}

#[cfg(target_arch = "wasm32")]
fn persist_current_user(name: &str) {
    if let Some(storage) = web_sys::window()
        .and_then(|w| w.local_storage().ok())
        .flatten()
    {
        let _ = storage.set_item("kanban_user", name);
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn persist_current_user(_name: &str) {}

#[component]
pub fn Todos() -> Element {
    let tasks = use_signal(load_tasks);
    let current_user = use_signal(load_current_user);
    let mut form_open = use_signal(|| false);

    let tasks_effect = tasks;
    use_effect(move || persist_tasks(&tasks_effect()));

    let user_effect = current_user;
    use_effect(move || persist_current_user(&user_effect()));

    rsx! {
        div { class: "max-w-7xl mx-auto px-6 py-6",
            // Header bar
            div { class: "flex items-center justify-between mb-6",
                h1 { class: "text-2xl font-bold text-gray-800",
                    "Kanban Board"
                }
                div { class: "flex items-center gap-4",
                    UserSelector { current_user }
                    button {
                        class: "text-sm px-4 py-2 rounded-lg font-medium bg-blue-500 text-white
                                hover:bg-blue-600 transition-colors shadow-sm",
                        onclick: move |_| form_open.set(!form_open()),
                        if form_open() { "Close" } else { "+ New Task" }
                    }
                }
            }

            // Task creation form
            TaskForm {
                tasks,
                is_open: form_open,
            }

            // 4-column grid
            div { class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4",
                for status in TaskStatus::all() {
                    KanbanColumn {
                        key: "{status.label()}",
                        status,
                        tasks,
                        current_user,
                    }
                }
            }
        }
    }
}
