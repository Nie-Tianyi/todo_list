use crate::backend::create_task;
use crate::models::{Priority, Task};
use chrono::NaiveDate;
use dioxus::prelude::*;

#[component]
pub fn TaskForm(tasks: Signal<Vec<Task>>, is_open: Signal<bool>) -> Element {
    let mut title = use_signal(String::new);
    let mut description = use_signal(String::new);
    let mut priority = use_signal(|| Priority::Medium);
    let mut start_date = use_signal(String::new);
    let mut due_date = use_signal(String::new);
    let mut error = use_signal(|| None::<String>);

    let mut reset_form = move || {
        title.set(String::new());
        description.set(String::new());
        priority.set(Priority::Medium);
        start_date.set(String::new());
        due_date.set(String::new());
        error.set(None);
    };

    if !is_open() {
        return rsx! {
            div {}
        };
    }

    rsx! {
        div { class: "bg-white dark:bg-gray-900 rounded-xl border border-gray-100 dark:border-gray-700 shadow-sm p-5 mb-6",
            h3 { class: "text-sm font-semibold text-gray-700 dark:text-gray-200 mb-4", "New Task" }

            // Title
            div { class: "mb-3",
                input {
                    class: "w-full border border-gray-200 dark:border-gray-600 rounded-lg px-3 py-2 text-sm dark:bg-gray-800 dark:text-gray-100
                            focus:outline-none focus:ring-2 focus:ring-blue-200 focus:border-blue-300
                            placeholder:text-gray-300 dark:placeholder:text-gray-500",
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
                    class: "w-full border border-gray-200 dark:border-gray-600 rounded-lg px-3 py-2 text-sm resize-none h-20 dark:bg-gray-800 dark:text-gray-100
                            focus:outline-none focus:ring-2 focus:ring-blue-200 focus:border-blue-300
                            placeholder:text-gray-300 dark:placeholder:text-gray-500",
                    placeholder: "Description (optional)",
                    value: "{description}",
                    oninput: move |e| description.set(e.value()),
                }
            }

            // Date fields
            div { class: "mb-3 flex gap-3",
                div { class: "flex-1",
                    label { class: "block text-xs text-gray-500 dark:text-gray-400 mb-1", r#for: "start_date", "Start Date" }
                    input {
                        id: "start_date",
                        r#type: "date",
                        class: "w-full border border-gray-200 dark:border-gray-600 rounded-lg px-3 py-2 text-sm dark:bg-gray-800 dark:text-gray-100
                                focus:outline-none focus:ring-2 focus:ring-blue-200 focus:border-blue-300",
                        value: "{start_date}",
                        oninput: move |e| { start_date.set(e.value()); error.set(None); },
                    }
                }
                div { class: "flex-1",
                    label { class: "block text-xs text-gray-500 dark:text-gray-400 mb-1", r#for: "due_date", "Due Date" }
                    input {
                        id: "due_date",
                        r#type: "date",
                        class: "w-full border border-gray-200 dark:border-gray-600 rounded-lg px-3 py-2 text-sm dark:bg-gray-800 dark:text-gray-100
                                focus:outline-none focus:ring-2 focus:ring-blue-200 focus:border-blue-300",
                        value: "{due_date}",
                        oninput: move |e| { due_date.set(e.value()); error.set(None); },
                    }
                }
            }

            // Priority selector + submit
            div { class: "flex items-center justify-between",
                div { class: "flex items-center gap-1.5",
                    span { class: "text-xs text-gray-500 dark:text-gray-400", "Priority:" }
                    for p in [Priority::Low, Priority::Medium, Priority::High, Priority::Urgent] {
                        button {
                            class: if priority() == p { "text-[11px] px-2.5 py-1 rounded-full font-medium transition-colors {p.badge_classes()} ring-2 ring-offset-1 ring-blue-300" } else { "text-[11px] px-2.5 py-1 rounded-full font-medium transition-colors bg-gray-100 dark:bg-gray-700 text-gray-500 dark:text-gray-400 hover:bg-gray-200 dark:hover:bg-gray-600" },
                            onclick: move |_| priority.set(p.clone()),
                            "{p.label()}"
                        }
                    }
                }

                div { class: "flex items-center gap-2",
                    button {
                        class: "text-xs px-3 py-1.5 rounded-lg font-medium text-gray-500 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors",
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
                            let desc = description().trim().to_string();
                            let pri = priority();
                            let sd = start_date().trim().to_string();
                            let dd = due_date().trim().to_string();
                            let parsed_start = if sd.is_empty() {
                                None
                            } else {
                                NaiveDate::parse_from_str(&sd, "%Y-%m-%d").ok()
                            };
                            let parsed_due = if dd.is_empty() {
                                None
                            } else {
                                NaiveDate::parse_from_str(&dd, "%Y-%m-%d").ok()
                            };

                            spawn(async move {
                                if let Ok(new_task) = create_task(t, desc, pri, parsed_start, parsed_due).await {
                                    tasks.write().push(new_task);
                                }
                            });

                            reset_form();
                            is_open.set(false);
                        },
                        "Add Task"
                    }
                }
            }

            if let Some(ref msg) = error() {
                p { class: "text-xs text-red-500 dark:text-red-400 mt-2", "{msg}" }
            }
        }
    }
}
