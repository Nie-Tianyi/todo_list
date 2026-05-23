use crate::models::{Priority, Task, TaskStatus};
use dioxus::prelude::*;

#[component]
pub fn TaskForm(tasks: Signal<Vec<Task>>, is_open: Signal<bool>) -> Element {
    let mut title = use_signal(String::new);
    let mut description = use_signal(String::new);
    let mut priority = use_signal(|| Priority::Medium);
    let mut error = use_signal(|| None::<String>);

    let mut reset_form = move || {
        title.set(String::new());
        description.set(String::new());
        priority.set(Priority::Medium);
        error.set(None);
    };

    if !is_open() {
        return rsx! { div {} };
    }

    rsx! {
        div { class: "bg-white rounded-xl border border-gray-100 shadow-sm p-5 mb-6",
            h3 { class: "text-sm font-semibold text-gray-700 mb-4", "New Task" }

            // Title
            div { class: "mb-3",
                input {
                    class: "w-full border border-gray-200 rounded-lg px-3 py-2 text-sm
                            focus:outline-none focus:ring-2 focus:ring-blue-200 focus:border-blue-300
                            placeholder:text-gray-300",
                    placeholder: "Task title",
                    value: "{title}",
                    oninput: move |e| {
                        title.set(e.value());
                        error.set(None);
                    },
                }
            }

            // Description
            div { class: "mb-3",
                textarea {
                    class: "w-full border border-gray-200 rounded-lg px-3 py-2 text-sm resize-none h-20
                            focus:outline-none focus:ring-2 focus:ring-blue-200 focus:border-blue-300
                            placeholder:text-gray-300",
                    placeholder: "Description (optional)",
                    value: "{description}",
                    oninput: move |e| description.set(e.value()),
                }
            }

            // Priority selector + submit
            div { class: "flex items-center justify-between",
                div { class: "flex items-center gap-1.5",
                    span { class: "text-xs text-gray-500", "Priority:" }
                    for p in [Priority::Low, Priority::Medium, Priority::High, Priority::Urgent] {
                        button {
                            class: if priority() == p {
                                "text-[11px] px-2.5 py-1 rounded-full font-medium transition-colors {p.badge_classes()} ring-2 ring-offset-1 ring-blue-300"
                            } else {
                                "text-[11px] px-2.5 py-1 rounded-full font-medium transition-colors bg-gray-100 text-gray-500 hover:bg-gray-200"
                            },
                            onclick: move |_| priority.set(p.clone()),
                            "{p.label()}"
                        }
                    }
                }

                div { class: "flex items-center gap-2",
                    button {
                        class: "text-xs px-3 py-1.5 rounded-lg font-medium text-gray-500 hover:bg-gray-100 transition-colors",
                        onclick: move |_| {
                            reset_form();
                            is_open.set(false);
                        },
                        "Cancel"
                    }
                    button {
                        class: "text-xs px-4 py-1.5 rounded-lg font-medium bg-blue-500 text-white
                                hover:bg-blue-600 transition-colors shadow-sm",
                        onclick: move |_| {
                            let t = title().trim().to_string();
                            if t.is_empty() {
                                error.set(Some("Title is required".into()));
                                return;
                            }
                            let next_id = tasks().iter().map(|t| t.id).max().unwrap_or(0) + 1;
                            tasks.write().push(Task {
                                id: next_id,
                                title: t,
                                description: description().trim().to_string(),
                                priority: priority(),
                                status: TaskStatus::Todo,
                                assignee: None,
                                completed_by: None,
                            });
                            reset_form();
                            is_open.set(false);
                        },
                        "Add Task"
                    }
                }
            }

            if let Some(ref msg) = error() {
                p { class: "text-xs text-red-500 mt-2", "{msg}" }
            }
        }
    }
}
