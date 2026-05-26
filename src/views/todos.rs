use crate::backend::get_tasks;
use crate::components::kanban_column::KanbanColumn;
use crate::components::task_form::TaskForm;
use crate::models::{Task, TaskStatus};
use dioxus::prelude::*;
use futures_util::StreamExt;

#[component]
pub fn Todos() -> Element {
    let mut tasks = use_signal(Vec::new);
    let mut form_open = use_signal(|| false);

    use_future(move || async move {
        tasks.set(get_tasks().await.unwrap_or_default());
    });

    // Persistent coroutine for backend saves — tied to Todos' scope, never unmounted.
    let _save_tx = use_coroutine(|mut rx: UnboundedReceiver<Task>| async move {
        while let Some(task) = rx.next().await {
            let _ = crate::backend::save_task(task).await;
        }
    });

    rsx! {
        div { class: "max-w-7xl mx-auto px-6 py-6",
            div { class: "flex items-center justify-between mb-6",
                h1 { class: "text-2xl font-bold text-gray-800 dark:text-gray-100", "Kanban Board" }
                div { class: "flex items-center gap-4",
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

            TaskForm { tasks, is_open: form_open }

            div { class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4",
                for status in TaskStatus::all() {
                    KanbanColumn {
                        key: "{status.label()}",
                        status,
                        tasks,
                    }
                }
            }
        }
    }
}
