use crate::backend::save_task;
use crate::models::{Task, TaskStatus};
use dioxus::prelude::*;

#[component]
pub fn TaskCard(
    task: Task,
    tasks: Signal<Vec<Task>>,
    current_user: Signal<String>,
) -> Element {
    let task_id = task.id;
    let has_previous = task.status.previous().is_some();
    let has_next = task.status.next().is_some();
    let task_claim = task.clone();
    let task_prev = task.clone();
    let task_next = task.clone();

    rsx! {
        div { class: "bg-white rounded-lg border border-gray-100 shadow-sm p-3 hover:shadow-md transition-shadow duration-200",
            // Title + priority badge
            div { class: "flex items-start justify-between gap-2 mb-1.5",
                h4 { class: "text-sm font-medium text-gray-800 leading-snug", "{task.title}" }
                span { class: "text-[10px] font-medium px-1.5 py-0.5 rounded-full {task.priority.badge_classes()} flex-shrink-0",
                    "{task.priority.label()}"
                }
            }

            // Description
            p { class: "text-xs text-gray-500 line-clamp-2 mb-3 leading-relaxed",
                "{task.description}"
            }

            // Assignee / completed-by info
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

            // Action buttons
            div { class: "flex items-center gap-1.5 border-t border-gray-50 pt-2",
                // Claim button (only when unassigned and not in Done)
                if task.assignee.is_none() {
                    button {
                        class: "text-[11px] px-2 py-1 rounded-md font-medium bg-blue-50 text-blue-600
                                hover:bg-blue-100 transition-colors",
                        onclick: move |_| {
                            let user = current_user();
                            let mut updated = task_claim.clone();
                            updated.assignee = Some(user);
                            tasks
                                .write()
                                .iter_mut()
                                .find(|t| t.id == task_id)
                                .map(|t| {
                                    t.assignee = updated.assignee.clone();
                                });
                            spawn(async move {
                                let _ = save_task(updated).await;
                            });
                        },
                        "Claim"
                    }
                }

                // Move buttons on the right
                div { class: "flex gap-1 ml-auto",
                    if has_previous {
                        button {
                            class: "text-[11px] px-2 py-1 rounded-md font-medium bg-gray-100 text-gray-500
                                    hover:bg-gray-200 transition-colors",
                            onclick: move |_| {
                                let mut updated = task_prev.clone();
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
                                spawn(async move {
                                    let _ = save_task(updated).await;
                                });
                            },
                            "\u{25C0}"
                        }
                    }

                    if has_next {
                        button {
                            class: "text-[11px] px-2 py-1 rounded-md font-medium bg-gray-100 text-gray-500
                                    hover:bg-gray-200 transition-colors",
                            onclick: move |_| {
                                let user = current_user();
                                let mut updated = task_next.clone();
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
                                                t.completed_by = Some(current_user());
                                            }
                                            t.status = nxt;
                                        }
                                    });
                                spawn(async move {
                                    let _ = save_task(updated).await;
                                });
                            },
                            "\u{25B6}"
                        }
                    }
                }
            }
        }
    }
}
