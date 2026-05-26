use crate::backend::get_tasks;
use crate::models::{Priority, Task, TaskStatus};
use chrono::{Datelike, Duration, NaiveDate};
use dioxus::prelude::*;

const LEFT_PANEL_WIDTH: f64 = 220.0;
const PIXELS_PER_DAY: f64 = 36.0;
const ROW_HEIGHT: f64 = 48.0;
const BAR_HEIGHT: f64 = 28.0;

fn bar_color(priority: &Priority) -> &'static str {
    match priority {
        Priority::Low => "bg-gray-400 dark:bg-gray-500",
        Priority::Medium => "bg-blue-400 dark:bg-blue-500",
        Priority::High => "bg-amber-400 dark:bg-amber-500",
        Priority::Urgent => "bg-red-400 dark:bg-red-500",
    }
}

fn status_accent(status: &TaskStatus) -> &'static str {
    match status {
        TaskStatus::Todo => "border-l-blue-500",
        TaskStatus::InProgress => "border-l-amber-500",
        TaskStatus::InReview => "border-l-purple-500",
        TaskStatus::Done => "border-l-green-500",
    }
}

fn is_weekend(date: &NaiveDate) -> bool {
    matches!(date.weekday(), chrono::Weekday::Sat | chrono::Weekday::Sun)
}

#[component]
pub fn Gantt() -> Element {
    let tasks_data = use_resource(move || async move { get_tasks().await.unwrap_or_default() });

    let all_tasks = tasks_data().unwrap_or_default();

    // Only tasks with both dates
    let gantt_tasks: Vec<Task> = all_tasks
        .iter()
        .filter(|t| t.start_date.is_some() && t.due_date.is_some())
        .cloned()
        .collect();

    if gantt_tasks.is_empty() {
        return rsx! {
            div { class: "max-w-6xl mx-auto px-6 py-12",
                h1 { class: "text-2xl font-bold text-gray-800 dark:text-gray-100 mb-6", "Gantt Chart" }
                div { class: "bg-white dark:bg-gray-900 rounded-xl border border-gray-100 dark:border-gray-700 shadow-sm p-12 text-center",
                    p { class: "text-gray-500 dark:text-gray-400 text-sm",
                        "No tasks with start and due dates yet."
                    }
                    p { class: "text-gray-400 dark:text-gray-500 text-xs mt-2",
                        "Add start and due dates to your tasks to see them on the Gantt chart."
                    }
                }
            }
        };
    }

    let min_date = gantt_tasks.iter().filter_map(|t| t.start_date).min().unwrap();
    let max_date = gantt_tasks.iter().filter_map(|t| t.due_date).max().unwrap();
    let total_days = (max_date - min_date).num_days().max(0) + 1;
    let timeline_width = total_days as f64 * PIXELS_PER_DAY;

    let days: Vec<NaiveDate> = (0..total_days)
        .map(|i| min_date + Duration::days(i))
        .collect();

    // Precompute bar positions (let-bindings not allowed inside rsx! for loops)
    struct RowData {
        task: Task,
        bar_left: f64,
        bar_w: f64,
        bar_days: i64,
        bar_top: f64,
        row_bg: &'static str,
    }
    let rows: Vec<RowData> = gantt_tasks
        .iter()
        .enumerate()
        .map(|(idx, task)| {
            let sd = task.start_date.unwrap();
            let dd = task.due_date.unwrap();
            let bar_left = (sd - min_date).num_days() as f64 * PIXELS_PER_DAY;
            let bar_days = (dd - sd).num_days().max(0) + 1;
            let bar_w = bar_days as f64 * PIXELS_PER_DAY;
            let bar_top = (ROW_HEIGHT - BAR_HEIGHT) / 2.0;
            let row_bg = if idx % 2 == 0 { "bg-white dark:bg-gray-900" } else { "bg-gray-50/30 dark:bg-gray-900/50" };
            RowData { task: task.clone(), bar_left, bar_w, bar_days, bar_top, row_bg }
        })
        .collect();

    rsx! {
        div { class: "max-w-6xl mx-auto px-6 py-12",
            h1 { class: "text-2xl font-bold text-gray-800 dark:text-gray-100 mb-6", "Gantt Chart" }

            div { class: "bg-white dark:bg-gray-900 rounded-xl border border-gray-100 dark:border-gray-700 shadow-sm overflow-hidden",
                // Scrollable timeline area
                div {
                    class: "overflow-x-auto",
                    div {
                        style: "min-width: {LEFT_PANEL_WIDTH + timeline_width}px",

                        // ── Month header row ──────────────────
                        div {
                            class: "flex sticky top-0 z-10 bg-gray-50 dark:bg-gray-800 border-b border-gray-200 dark:border-gray-600",
                            div {
                                class: "flex-shrink-0 px-3 py-2 text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wider border-r border-gray-200 dark:border-gray-600",
                                style: "width: {LEFT_PANEL_WIDTH}px",
                                "Task"
                            }
                            div { class: "flex",
                                for day in &days {
                                    div {
                                        class: "flex-shrink-0 text-center py-2 border-r border-gray-100 dark:border-gray-700",
                                        class: if is_weekend(day) { "bg-gray-100/50 dark:bg-gray-800/50" },
                                        style: "width: {PIXELS_PER_DAY}px",
                                        span {
                                            class: "text-[10px] font-medium",
                                            class: if is_weekend(day) { "text-gray-400 dark:text-gray-500" } else { "text-gray-500 dark:text-gray-400" },
                                            "{day.format(\"%a\")}"
                                        }
                                        span {
                                            class: "text-[10px] ml-0.5",
                                            class: if is_weekend(day) { "text-gray-400 dark:text-gray-500" } else { "text-gray-500 dark:text-gray-400" },
                                            "{day.format(\"%d\")}"
                                        }
                                    }
                                }
                            }
                        }

                        // ── Task rows ─────────────────────────
                        for row in &rows {
                            div {
                                class: "flex {row.row_bg} border-b border-gray-100 dark:border-gray-700",
                                // Task label
                                div {
                                    class: "flex-shrink-0 px-3 py-2 text-xs text-gray-700 dark:text-gray-200 truncate border-r border-gray-100 dark:border-gray-700 flex items-center",
                                    style: "width: {LEFT_PANEL_WIDTH}px",
                                    title: "{row.task.title}",
                                    span { class: "truncate", "{row.task.title}" }
                                }
                                // Timeline cell
                                div {
                                    class: "relative",
                                    style: "width: {timeline_width}px; height: {ROW_HEIGHT}px",
                                    div {
                                        class: "absolute rounded-r border-l-4 {bar_color(&row.task.priority)} {status_accent(&row.task.status)} cursor-default",
                                        style: "left: {row.bar_left}px; width: {row.bar_w}px; top: {row.bar_top}px; height: {BAR_HEIGHT}px",
                                        title: "{row.task.title}: {row.task.start_date.unwrap().format(\"%b %d\")} \u{2013} {row.task.due_date.unwrap().format(\"%b %d\")} ({row.bar_days}d)",
                                        if row.bar_w > 60.0 {
                                            span {
                                                class: "text-[10px] font-medium text-white truncate block px-2 leading-7",
                                                "{row.task.title}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
