use crate::components::kanban_column::KanbanColumn;
use crate::components::task_form::TaskForm;
use crate::components::user_selector::UserSelector;
use crate::models::TaskStatus;
use dioxus::prelude::*;

use crate::backend::get_tasks;

#[component]
pub fn Todos() -> Element {
    let mut tasks = use_signal(Vec::new);
    let current_user = use_signal(String::new);
    let mut form_open = use_signal(|| false);

    let tasks_data = use_resource(move || async move {
        get_tasks().await.unwrap_or_default()
    });

    use_effect(move || {
        tasks.set(tasks_data().unwrap_or_default());
    });

    rsx! {
        div { class: "max-w-7xl mx-auto px-6 py-6",
            // Header bar
            div { class: "flex items-center justify-between mb-6",
                h1 { class: "text-2xl font-bold text-gray-800", "Kanban Board" }
                div { class: "flex items-center gap-4",
                    UserSelector { current_user }
                    button {
                        class: "text-sm px-4 py-2 rounded-lg font-medium bg-blue-500 text-white
                                hover:bg-blue-600 transition-colors shadow-sm",
                        onclick: move |_| form_open.set(!form_open()),
                        if form_open() {
                            "Close"
                        } else {
                            "+ New Task"
                        }
                    }
                }
            }

            // Task creation form
            TaskForm { tasks, is_open: form_open }

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
