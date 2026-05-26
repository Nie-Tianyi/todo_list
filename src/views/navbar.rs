use crate::auth::AuthContext;
use crate::DarkModeContext;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    let auth = use_context::<AuthContext>();
    let dark_mode = use_context::<DarkModeContext>();
    let navigator = use_navigator();

    rsx! {
        nav { class: "sticky top-0 z-50 w-full bg-white/80 dark:bg-gray-950/80 backdrop-blur-md border-b border-gray-100 dark:border-gray-800 shadow-sm",
            div { class: "max-w-4xl mx-auto flex items-center justify-between px-6 py-3",
                Link {
                    to: Route::Todos {},
                    class: "text-lg font-bold text-gray-800 dark:text-gray-100 hover:text-blue-500 dark:hover:text-blue-400 transition-colors duration-200",
                    "Todo App"
                }

                div { class: "flex items-center gap-1",
                    if auth.is_authenticated() {
                        Link {
                            to: Route::Todos {},
                            class: "px-4 py-2 text-sm font-medium text-gray-600 dark:text-gray-300 hover:text-blue-500 dark:hover:text-blue-400 hover:bg-blue-50 dark:hover:bg-blue-900/30 rounded-lg transition-colors duration-200",
                            "Todos"
                        }
                        Link {
                            to: Route::Gantt {},
                            class: "px-4 py-2 text-sm font-medium text-gray-600 dark:text-gray-300 hover:text-blue-500 dark:hover:text-blue-400 hover:bg-blue-50 dark:hover:bg-blue-900/30 rounded-lg transition-colors duration-200",
                            "Gantt"
                        }
                        Link {
                            to: Route::Documents {},
                            class: "px-4 py-2 text-sm font-medium text-gray-600 dark:text-gray-300 hover:text-blue-500 dark:hover:text-blue-400 hover:bg-blue-50 dark:hover:bg-blue-900/30 rounded-lg transition-colors duration-200",
                            "Documents"
                        }
                        Link {
                            to: Route::Team {},
                            class: "px-4 py-2 text-sm font-medium text-gray-600 dark:text-gray-300 hover:text-blue-500 dark:hover:text-blue-400 hover:bg-blue-50 dark:hover:bg-blue-900/30 rounded-lg transition-colors duration-200",
                            "Team"
                        }
                    }

                    if let Some(user) = auth.user() {
                        button {
                            class: "px-4 py-2 text-sm text-gray-500 dark:text-gray-400 mx-2 hover:text-blue-500 dark:hover:text-blue-400 hover:bg-blue-50 dark:hover:bg-blue-900/30 rounded-lg transition-colors duration-200 cursor-pointer",
                            onclick: move |_| {
                                navigator.push(Route::Profile {});
                            },
                            "{user.username}"
                        }

                        button {
                            class: "px-4 py-2 text-sm font-medium text-red-500 dark:text-red-400 hover:text-red-600 dark:hover:text-red-300 hover:bg-red-50 dark:hover:bg-red-900/30 rounded-lg transition-colors duration-200",
                            onclick: move |_| {
                                auth.logout();
                                navigator.push(Route::Login {});
                            },
                            "Logout"
                        }
                    } else {
                        Link {
                            to: Route::Register {},
                            class: "px-4 py-2 text-sm font-medium text-gray-600 dark:text-gray-300 hover:text-blue-500 dark:hover:text-blue-400 hover:bg-blue-50 dark:hover:bg-blue-900/30 rounded-lg transition-colors duration-200",
                            "Register"
                        }
                        Link {
                            to: Route::Login {},
                            class: "px-4 py-2 text-sm font-medium text-blue-500 dark:text-blue-400 hover:text-blue-600 dark:hover:text-blue-300 hover:bg-blue-50 dark:hover:bg-blue-900/30 rounded-lg transition-colors duration-200",
                            "Login"
                        }
                    }

                    // Dark mode toggle — always visible
                    button {
                        class: "px-2 py-2 text-sm rounded-lg transition-colors duration-200 cursor-pointer hover:bg-gray-100 dark:hover:bg-gray-700",
                        onclick: move |_| dark_mode.toggle(),
                        if (dark_mode.is_dark)() {
                            span { class: "text-yellow-400", "\u{2600}\u{FE0F}" }
                        } else {
                            span { class: "text-gray-500", "\u{1F319}" }
                        }
                    }
                }
            }
        }

        Outlet::<Route> {}
    }
}
