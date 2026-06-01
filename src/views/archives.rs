use crate::backend::{get_archived_tasks, save_task};
use crate::models::Priority;
use chrono::NaiveDate;
use dioxus::prelude::*;

#[component]
pub fn Archives() -> Element {
    let mut refresh = use_signal(|| 0);
    let tasks = use_resource(move || {
        let _ = refresh();
        async move { get_archived_tasks().await }
    });

    let mut message = use_signal(|| None::<String>);
    let mut acting_id = use_signal(|| None::<u32>);

    // Filter state
    let mut priority_filter = use_signal(|| None::<Priority>);
    let mut completed_by_filter = use_signal(String::new);
    let mut date_from_filter = use_signal(String::new);
    let mut date_to_filter = use_signal(String::new);
    let mut search_keyword = use_signal(String::new);

    rsx! {
        div { class: "max-w-4xl mx-auto px-6 py-12",
            h1 { class: "text-2xl font-bold text-gray-800 dark:text-gray-100 mb-6",
                "Archived Tasks"
            }

            // Status message (success or error)
            if let Some(ref msg) = message() {
                div { class: "mb-4 p-3 bg-blue-50 dark:bg-blue-900/30 border border-blue-100 dark:border-blue-800 text-blue-600 dark:text-blue-400 text-sm rounded-lg",
                    "{msg}"
                }
            }

            match tasks() {
                // Loading
                None => rsx! {
                    div { class: "flex items-center justify-center py-16",
                        div { class: "animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500" }
                    }
                },
                Some(Err(ref e)) => rsx! {
                    div { class: "p-4 bg-red-50 dark:bg-red-900/30 border border-red-100 dark:border-red-800 text-red-600 dark:text-red-400 text-sm rounded-lg",
                        "Failed to load archived tasks: {e}"
                    }
                },
                Some(Ok(list)) => {
                    if list.is_empty() {
                        rsx! {
                            div { class: "text-center py-16",
                                div { class: "text-4xl mb-4", "\u{1F4E6}" }
                                p { class: "text-gray-500 dark:text-gray-400",
                                    "No archived tasks."
                                }
                                p { class: "text-sm text-gray-400 dark:text-gray-500 mt-2",
                                    "Complete and archive tasks from the kanban board to see them here."
                                }
                            }
                        }
                    } else {
                        // ── Apply all filters ──
                        let priority = priority_filter();
                        let completed_by = completed_by_filter();
                        let date_from = date_from_filter();
                        let date_to = date_to_filter();
                        let keyword = search_keyword();

                        let any_filter = priority.is_some()
                            || !completed_by.is_empty()
                            || !date_from.is_empty()
                            || !date_to.is_empty()
                            || !keyword.is_empty();

                        let filtered: Vec<_> = list.iter().filter(|task| {
                            if let Some(ref p) = priority {
                                if task.priority != *p { return false; }
                            }
                            if !completed_by.is_empty() {
                                match &task.completed_by {
                                    Some(val) => {
                                        if !val.to_lowercase().contains(&completed_by.to_lowercase()) {
                                            return false;
                                        }
                                    }
                                    None => return false,
                                }
                            }
                            if !date_from.is_empty() || !date_to.is_empty() {
                                match task.due_date {
                                    Some(due) => {
                                        if !date_from.is_empty() {
                                            if let Ok(from_date) = NaiveDate::parse_from_str(&date_from, "%Y-%m-%d") {
                                                if due < from_date { return false; }
                                            }
                                        }
                                        if !date_to.is_empty() {
                                            if let Ok(to_date) = NaiveDate::parse_from_str(&date_to, "%Y-%m-%d") {
                                                if due > to_date { return false; }
                                            }
                                        }
                                    }
                                    None => return false,
                                }
                            }
                            if !keyword.is_empty() {
                                let kw = keyword.to_lowercase();
                                if !task.title.to_lowercase().contains(&kw)
                                    && !task.description.to_lowercase().contains(&kw)
                                {
                                    return false;
                                }
                            }
                            true
                        }).cloned().collect();

                        let count = filtered.len();
                        let suffix = if count != 1 { "s" } else { "" };

                        rsx! {
                            p { class: "text-sm text-gray-500 dark:text-gray-400 mb-4",
                                "{count} archived task{suffix}"
                            }

                            // ── Filter bar ──
                            div { class: "mb-6 p-4 bg-gray-50 dark:bg-gray-800/50 rounded-xl border border-gray-100 dark:border-gray-700",
                                div { class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3",
                                    // Priority filter
                                    div {
                                        label { class: "block text-xs font-medium text-gray-500 dark:text-gray-400 mb-1",
                                            "Priority"
                                        }
                                        select {
                                            class: "w-full border border-gray-200 dark:border-gray-600 rounded-lg px-3 py-2 text-sm dark:bg-gray-800 dark:text-gray-100 focus:outline-none focus:ring-2 focus:ring-blue-200 focus:border-blue-300",
                                            value: "{priority_filter().map(|p| p.label().to_string()).unwrap_or_default()}",
                                            oninput: move |e| {
                                                let val = e.value();
                                                priority_filter.set(match val.as_str() {
                                                    "Low" => Some(Priority::Low),
                                                    "Medium" => Some(Priority::Medium),
                                                    "High" => Some(Priority::High),
                                                    "Urgent" => Some(Priority::Urgent),
                                                    _ => None,
                                                });
                                            },
                                            option { value: "", "All" }
                                            option { value: "Low", "Low" }
                                            option { value: "Medium", "Medium" }
                                            option { value: "High", "High" }
                                            option { value: "Urgent", "Urgent" }
                                        }
                                    }

                                    // Completed by filter
                                    div {
                                        label { class: "block text-xs font-medium text-gray-500 dark:text-gray-400 mb-1",
                                            "Completed By"
                                        }
                                        input {
                                            r#type: "text",
                                            placeholder: "Filter by name...",
                                            class: "w-full border border-gray-200 dark:border-gray-600 rounded-lg px-3 py-2 text-sm dark:bg-gray-800 dark:text-gray-100 focus:outline-none focus:ring-2 focus:ring-blue-200 focus:border-blue-300 placeholder:text-gray-300 dark:placeholder:text-gray-500",
                                            value: "{completed_by_filter}",
                                            oninput: move |e| completed_by_filter.set(e.value()),
                                        }
                                    }

                                    // Date From
                                    div {
                                        label { class: "block text-xs font-medium text-gray-500 dark:text-gray-400 mb-1",
                                            "Due Date From"
                                        }
                                        input {
                                            r#type: "date",
                                            class: "w-full border border-gray-200 dark:border-gray-600 rounded-lg px-3 py-2 text-sm dark:bg-gray-800 dark:text-gray-100 focus:outline-none focus:ring-2 focus:ring-blue-200 focus:border-blue-300",
                                            value: "{date_from_filter}",
                                            oninput: move |e| date_from_filter.set(e.value()),
                                        }
                                    }

                                    // Date To
                                    div {
                                        label { class: "block text-xs font-medium text-gray-500 dark:text-gray-400 mb-1",
                                            "Due Date To"
                                        }
                                        input {
                                            r#type: "date",
                                            class: "w-full border border-gray-200 dark:border-gray-600 rounded-lg px-3 py-2 text-sm dark:bg-gray-800 dark:text-gray-100 focus:outline-none focus:ring-2 focus:ring-blue-200 focus:border-blue-300",
                                            value: "{date_to_filter}",
                                            oninput: move |e| date_to_filter.set(e.value()),
                                        }
                                    }

                                    // Search keyword filter
                                    div {
                                        label { class: "block text-xs font-medium text-gray-500 dark:text-gray-400 mb-1",
                                            "Search"
                                        }
                                        input {
                                            r#type: "text",
                                            placeholder: "Title or description...",
                                            class: "w-full border border-gray-200 dark:border-gray-600 rounded-lg px-3 py-2 text-sm dark:bg-gray-800 dark:text-gray-100 focus:outline-none focus:ring-2 focus:ring-blue-200 focus:border-blue-300 placeholder:text-gray-300 dark:placeholder:text-gray-500",
                                            value: "{search_keyword}",
                                            oninput: move |e| search_keyword.set(e.value()),
                                        }
                                    }

                                    // Clear filters button
                                    if any_filter {
                                        div { class: "flex items-end",
                                            button {
                                                class: "w-full px-3 py-2 text-xs font-medium text-gray-500 dark:text-gray-400 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-600 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors whitespace-nowrap",
                                                onclick: move |_| {
                                                    priority_filter.set(None);
                                                    completed_by_filter.set(String::new());
                                                    date_from_filter.set(String::new());
                                                    date_to_filter.set(String::new());
                                                    search_keyword.set(String::new());
                                                },
                                                "Clear filters"
                                            }
                                        }
                                    }
                                }
                            }

                            // ── Filtered results ──
                            if filtered.is_empty() {
                                div { class: "text-center py-16",
                                    div { class: "text-4xl mb-4", "\u{1F50D}" }
                                    p { class: "text-gray-500 dark:text-gray-400",
                                        "No tasks match your filters."
                                    }
                                    p { class: "mt-3",
                                        button {
                                            class: "text-sm text-blue-500 hover:text-blue-600 dark:text-blue-400 dark:hover:text-blue-300 underline transition-colors",
                                            onclick: move |_| {
                                                priority_filter.set(None);
                                                completed_by_filter.set(String::new());
                                                date_from_filter.set(String::new());
                                                date_to_filter.set(String::new());
                                                search_keyword.set(String::new());
                                            },
                                            "Clear filters"
                                        }
                                    }
                                }
                            } else {
                                div { class: "space-y-3",
                                    for task in filtered {{
                                        // Compute date string outside RSX interpolations to avoid parse issues
                                        let date_display = if task.start_date.is_some() || task.due_date.is_some() {
                                            format!(
                                                "{} -> {}",
                                                task.start_date.map(|d| d.to_string()).unwrap_or_else(|| String::from("?")),
                                                task.due_date.map(|d| d.to_string()).unwrap_or_else(|| String::from("?"))
                                            )
                                        } else {
                                            String::new()
                                        };

                                        let task_id = task.id;
                                        let unarchive_task = task.clone();
                                        let delete_task = task.clone();

                                        rsx! {
                                            div { class: "bg-white dark:bg-gray-900 rounded-xl border border-gray-100 dark:border-gray-700 shadow-sm p-5",
                                                // Top row: Title + badges
                                                div { class: "flex items-start justify-between gap-3 mb-3",
                                                    div { class: "flex-1 min-w-0",
                                                        h3 { class: "font-semibold text-gray-800 dark:text-gray-100 text-lg",
                                                            "{task.title}"
                                                        }
                                                    }
                                                    div { class: "flex items-center gap-2 shrink-0",
                                                        span { class: "px-2 py-0.5 text-xs font-medium rounded {task.priority.badge_classes()}",
                                                            "{task.priority.label()}"
                                                        }
                                                        span { class: "px-2 py-0.5 text-xs font-medium rounded-full bg-gray-100 text-gray-600 dark:bg-gray-700 dark:text-gray-400",
                                                            "{task.status.label()}"
                                                        }
                                                    }
                                                }

                                                // Description
                                                if !task.description.is_empty() {
                                                    p { class: "text-sm text-gray-500 dark:text-gray-400 mb-3 line-clamp-2",
                                                        "{task.description}"
                                                    }
                                                }

                                                // Metadata row
                                                div { class: "flex flex-wrap items-center gap-x-6 gap-y-1 text-xs text-gray-500 dark:text-gray-400 mb-4",
                                                    div { class: "flex items-center gap-1",
                                                        span { class: "font-medium text-gray-600 dark:text-gray-300", "Assignee:" }
                                                        if let Some(ref assignee) = task.assignee {
                                                            span { class: "text-gray-500 dark:text-gray-400", "{assignee}" }
                                                        } else {
                                                            span { class: "italic text-gray-400 dark:text-gray-500", "Unassigned" }
                                                        }
                                                    }
                                                    if let Some(ref completed_by) = task.completed_by {
                                                        div { class: "flex items-center gap-1",
                                                            span { class: "font-medium text-gray-600 dark:text-gray-300", "Completed by:" }
                                                            span { class: "text-gray-500 dark:text-gray-400", "{completed_by}" }
                                                        }
                                                    }
                                                    if !date_display.is_empty() {
                                                        div { class: "flex items-center gap-1",
                                                            span { class: "font-medium text-gray-600 dark:text-gray-300", "Dates:" }
                                                            span { class: "text-gray-500 dark:text-gray-400", "{date_display}" }
                                                        }
                                                    }
                                                }

                                                // Action buttons
                                                div { class: "flex items-center gap-2",
                                                    button {
                                                        class: "px-3 py-1.5 text-xs font-medium text-white bg-teal-500 hover:bg-teal-600 dark:bg-teal-600 dark:hover:bg-teal-500 rounded-lg disabled:opacity-50 disabled:cursor-not-allowed transition-colors",
                                                        disabled: acting_id() == Some(task_id),
                                                        onclick: move |_| {
                                                            let mut updated = unarchive_task.clone();
                                                            updated.archived = false;
                                                            acting_id.set(Some(task_id));
                                                            message.set(None);
                                                            spawn(async move {
                                                                let title = updated.title.clone();
                                                                match save_task(updated).await {
                                                                    Ok(()) => {
                                                                        refresh.set(refresh() + 1);
                                                                        acting_id.set(None);
                                                                        message.set(Some(format!("\"{title}\" unarchived and moved back to the board.")));
                                                                    }
                                                                    Err(e) => {
                                                                        acting_id.set(None);
                                                                        message.set(Some(format!("Failed to unarchive: {e}")));
                                                                    }
                                                                }
                                                            });
                                                        },
                                                        if acting_id() == Some(task_id) { "Working..." } else { "Unarchive" }
                                                    }
                                                    button {
                                                        class: "px-3 py-1.5 text-xs font-medium text-red-600 dark:text-red-400 border border-red-200 dark:border-red-800 rounded-lg hover:bg-red-50 dark:hover:bg-red-900/30 disabled:opacity-50 disabled:cursor-not-allowed transition-colors",
                                                        disabled: acting_id() == Some(task_id),
                                                        onclick: move |_| {
                                                            let mut updated = delete_task.clone();
                                                            updated.deleted = true;
                                                            acting_id.set(Some(task_id));
                                                            message.set(None);
                                                            spawn(async move {
                                                                let title = updated.title.clone();
                                                                match save_task(updated).await {
                                                                    Ok(()) => {
                                                                        refresh.set(refresh() + 1);
                                                                        acting_id.set(None);
                                                                        message.set(Some(format!("\"{title}\" permanently deleted.")));
                                                                    }
                                                                    Err(e) => {
                                                                        acting_id.set(None);
                                                                        message.set(Some(format!("Failed to delete: {e}")));
                                                                    }
                                                                }
                                                            });
                                                        },
                                                        if acting_id() == Some(task_id) { "Working..." } else { "Delete" }
                                                    }
                                                }
                                            }
                                        }
                                    }}
                                }
                            }
                        }
                    }
                },
            }
        }
    }
}
