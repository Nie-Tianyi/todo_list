use crate::auth::AuthContext;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    let auth = use_context::<AuthContext>();
    let navigator = use_navigator();

    rsx! {
        nav { class: "sticky top-0 z-50 w-full bg-white/80 backdrop-blur-md border-b border-gray-100 shadow-sm",
            div { class: "max-w-4xl mx-auto flex items-center justify-between px-6 py-3",
                Link {
                    to: Route::Todos {},
                    class: "text-lg font-bold text-gray-800 hover:text-blue-500 transition-colors duration-200",
                    "Todo App"
                }

                div { class: "flex items-center gap-1",
                    if auth.is_authenticated() {
                        Link {
                            to: Route::Todos {},
                            class: "px-4 py-2 text-sm font-medium text-gray-600 hover:text-blue-500 hover:bg-blue-50 rounded-lg transition-colors duration-200",
                            "Todos"
                        }
                        Link {
                            to: Route::Gantt {},
                            class: "px-4 py-2 text-sm font-medium text-gray-600 hover:text-blue-500 hover:bg-blue-50 rounded-lg transition-colors duration-200",
                            "Gantt"
                        }
                        Link {
                            to: Route::Team {},
                            class: "px-4 py-2 text-sm font-medium text-gray-600 hover:text-blue-500 hover:bg-blue-50 rounded-lg transition-colors duration-200",
                            "Team"
                        }
                        Link {
                            to: Route::Documents {},
                            class: "px-4 py-2 text-sm font-medium text-gray-600 hover:text-blue-500 hover:bg-blue-50 rounded-lg transition-colors duration-200",
                            "Documents"
                        }
                    }

                    if let Some(user) = auth.user() {
                        button {
                            class: "px-4 py-2 text-sm text-gray-500 mx-2 hover:text-blue-500 hover:bg-blue-50 rounded-lg transition-colors duration-200 cursor-pointer",
                            onclick: move |_| {
                                navigator.push(Route::Profile {});
                            },
                            "{user.username}"
                        }
                        button {
                            class: "px-4 py-2 text-sm font-medium text-red-500 hover:text-red-600 hover:bg-red-50 rounded-lg transition-colors duration-200",
                            onclick: move |_| {
                                auth.logout();
                                navigator.push(Route::Login {});
                            },
                            "Logout"
                        }
                    } else {
                        Link {
                            to: Route::Register {},
                            class: "px-4 py-2 text-sm font-medium text-gray-600 hover:text-blue-500 hover:bg-blue-50 rounded-lg transition-colors duration-200",
                            "Register"
                        }
                        Link {
                            to: Route::Login {},
                            class: "px-4 py-2 text-sm font-medium text-blue-500 hover:text-blue-600 hover:bg-blue-50 rounded-lg transition-colors duration-200",
                            "Login"
                        }
                    }
                }
            }
        }

        Outlet::<Route> {}
    }
}
