use crate::auth::AuthContext;
use crate::backend::get_team_members;
use dioxus::prelude::*;

#[component]
pub fn Team() -> Element {
    let auth = use_context::<AuthContext>();
    let current_user = auth.user();

    let members = use_resource(move || async move { get_team_members().await });

    if current_user.is_none() {
        return rsx! {
            div { class: "max-w-4xl mx-auto px-6 py-12",
                p { class: "text-gray-500 dark:text-gray-400", "Not logged in." }
            }
        };
    }

    rsx! {
        div { class: "max-w-4xl mx-auto px-6 py-12",
            h1 { class: "text-2xl font-bold text-gray-800 dark:text-gray-100 mb-6", "Team Members" }

            match members() {
                None => rsx! {
                    div { class: "flex items-center justify-center py-16",
                        div { class: "animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500" }
                    }
                },
                Some(Ok(list)) => {
                    if list.is_empty() {
                        rsx! {
                            p { class: "text-gray-500 dark:text-gray-400 text-center py-16", "No team members found." }
                        }
                    } else {
                        rsx! {
                            div { class: "grid gap-4 sm:grid-cols-2 lg:grid-cols-3",
                                for member in list {
                                    div { class: "bg-white dark:bg-gray-900 rounded-xl border border-gray-100 dark:border-gray-700 shadow-sm p-5 hover:shadow-md transition-shadow",
                                        div { class: "flex items-center gap-3 mb-3",
                                            div { class: "w-10 h-10 rounded-full bg-blue-100 dark:bg-blue-900/30 flex items-center justify-center text-blue-600 dark:text-blue-400 font-semibold text-sm",
                                                "{member.username.chars().next().unwrap_or('?').to_uppercase().to_string()}"
                                            }
                                            div {
                                                h3 { class: "font-semibold text-gray-800 dark:text-gray-100", "{member.username}" }
                                                if let Some(ref title) = member.job_title {
                                                    p { class: "text-xs text-gray-500 dark:text-gray-400", "{title}" }
                                                }
                                            }
                                        }

                                        div { class: "space-y-1.5 text-sm",
                                            div { class: "flex justify-between",
                                                span { class: "text-gray-500 dark:text-gray-400", "Gender" }
                                                span { class: "text-gray-700 dark:text-gray-200",
                                                    "{member.gender.as_deref().unwrap_or(\"-\")}"
                                                }
                                            }
                                            div { class: "flex justify-between",
                                                span { class: "text-gray-500 dark:text-gray-400", "Age" }
                                                span { class: "text-gray-700 dark:text-gray-200",
                                                    "{member.age.map(|a| a.to_string()).unwrap_or_else(|| String::from(\"-\"))}"
                                                }
                                            }
                                            if let Some(ref email) = member.email {
                                                div { class: "flex justify-between",
                                                    span { class: "text-gray-500 dark:text-gray-400", "Email" }
                                                    span { class: "text-gray-700 dark:text-gray-200 text-xs", "{email}" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                Some(Err(ref e)) => rsx! {
                    div { class: "p-4 bg-red-50 dark:bg-red-900/30 border border-red-100 dark:border-red-800 text-red-600 dark:text-red-400 text-sm rounded-lg",
                        "Failed to load team members: {e}"
                    }
                },
            }
        }
    }
}
