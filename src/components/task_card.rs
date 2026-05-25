use crate::auth::AuthContext;
use crate::models::{Task, TaskStatus};
use dioxus::prelude::*;

#[component]
pub fn TaskCard(task: Task, tasks: Signal<Vec<Task>>) -> Element {
    let save_tx = use_coroutine_handle::<Task>();
    let auth = use_context::<AuthContext>();
    let task_id = task.id;
    let has_previous = task.status.previous().is_some();
    let has_next = task.status.next().is_some();

    rsx! {
        div { class: "bg-white rounded-lg border border-gray-100 shadow-sm p-3 hover:shadow-md transition-shadow duration-200",
            div { class: "flex items-start justify-between gap-2 mb-1.5",
                h4 { class: "text-sm font-medium text-gray-800 leading-snug", "{task.title}" }
                span { class: "text-[10px] font-medium px-1.5 py-0.5 rounded-full {task.priority.badge_classes()} flex-shrink-0",
                    "{task.priority.label()}"
                }
            }

            p { class: "text-xs text-gray-500 line-clamp-2 mb-3 leading-relaxed",
                "{task.description}"
            }

            div { class: "flex items-center gap-2 text-[11px] mb-3",
                if let Some(ref name) = task.assignee {
                    span { class: "text-gray-500",
                        "Assigned to "
                        span { class: "font-medium text-gray-700", "{name}" }
                    }
                } else {
                    span { class: "text-gray-400 italic", "Unassigned" }
                }

                if let Some(ref name) = task.completed_by {
                    span { class: "text-green-600 ml-auto",
                        "\u{2713} "
                        span { class: "font-medium", "{name}" }
                    }
                }
            }

            div { class: "flex items-center gap-1.5 border-t border-gray-50 pt-2",
                if task.assignee.is_none() {
                    button {
                        class: "text-[11px] px-2 py-1 rounded-md font-medium bg-blue-50 text-blue-600
                                hover:bg-blue-100 transition-colors",
                        onclick: move |_| {
                            let user = auth.username().unwrap_or_default();
                            let mut updated = tasks
                                .read()
                                .iter()
                                .find(|t| t.id == task_id)
                                .cloned()
                                .unwrap();
                            updated.assignee = Some(user);
                            tasks
                                .write()
                                .iter_mut()
                                .find(|t| t.id == task_id)
                                .map(|t| t.assignee = updated.assignee.clone());
                            save_tx.send(updated);
                        },
                        "Claim"
                    }
                }

                div { class: "flex gap-1 ml-auto",
                    if has_previous {
                        button {
                            class: "text-[11px] px-2 py-1 rounded-md font-medium bg-gray-100 text-gray-500
                                    hover:bg-gray-200 transition-colors",
                            onclick: move |_| {
                                let mut updated = tasks
                                    .read()
                                    .iter()
                                    .find(|t| t.id == task_id)
                                    .cloned()
                                    .unwrap();
                                if let Some(prev) = updated.status.previous() {
                                    updated.status = prev;
                                }
                                tasks
                                    .write()
                                    .iter_mut()
                                    .find(|t| t.id == task_id)
                                    .map(|t| {
                                        if let Some(prev) = t.status.previous() {
                                            t.status = prev;
                                        }
                                    });
                                save_tx.send(updated);
                            },
                            "\u{25C0}"
                        }
                    }

                    if has_next {
                        button {
                            class: "text-[11px] px-2 py-1 rounded-md font-medium bg-gray-100 text-gray-500
                                    hover:bg-gray-200 transition-colors",
                            onclick: move |_| {
                                let user = auth.username().unwrap_or_default();
                                let mut updated = tasks
                                    .read()
                                    .iter()
                                    .find(|t| t.id == task_id)
                                    .cloned()
                                    .unwrap();
                                if let Some(nxt) = updated.status.next() {
                                    if nxt == TaskStatus::Done {
                                        updated.completed_by = Some(user);
                                    }
                                    updated.status = nxt;
                                }
                                tasks
                                    .write()
                                    .iter_mut()
                                    .find(|t| t.id == task_id)
                                    .map(|t| {
                                        if let Some(nxt) = t.status.next() {
                                            if nxt == TaskStatus::Done {
                                                t.completed_by = Some(auth.username().unwrap_or_default());
                                            }
                                            t.status = nxt;
                                        }
                                    });
                                save_tx.send(updated);
                            },
                            "\u{25B6}"
                        }
                    }
                }
            }
        }
    }
}
