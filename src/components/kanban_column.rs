use crate::models::TaskStatus;
use dioxus::prelude::*;

#[component]
pub fn KanbanColumn(
    status: TaskStatus,
    tasks: Signal<Vec<crate::models::Task>>,
    current_user: Signal<String>,
) -> Element {
    let filtered: Vec<crate::models::Task> = tasks()
        .into_iter()
        .filter(|t| t.status == status)
        .collect();
    let count = filtered.len();
    let color = status.column_color();

    rsx! {
        div { class: "flex flex-col bg-gray-50 rounded-xl border border-gray-100 shadow-sm min-h-[400px]",
            // Colored top border + header
            div { class: "border-t-4 rounded-t-xl {color} px-4 py-3",
                div { class: "flex items-center justify-between",
                    h3 { class: "text-sm font-semibold text-gray-700",
                        "{status.label()}"
                    }
                    span { class: "text-xs font-medium text-gray-400 bg-white rounded-full px-2 py-0.5",
                        "{count}"
                    }
                }
            }

            // Task list
            div { class: "flex flex-col gap-2 p-2 flex-1",
                for task in filtered {
                    super::task_card::TaskCard {
                        key: "{task.id}",
                        task,
                        tasks,
                        current_user,
                    }
                }

                // Empty placeholder
                if count == 0 {
                    div { class: "flex-1 flex items-center justify-center border-2 border-dashed border-gray-200 rounded-lg m-1",
                        p { class: "text-sm text-gray-400", "No tasks" }
                    }
                }
            }
        }
    }
}
