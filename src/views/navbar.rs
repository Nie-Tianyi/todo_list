use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {

    rsx! {
        nav { class: "sticky top-0 z-50 w-full bg-white/80 backdrop-blur-md border-b border-gray-100 shadow-sm",
            div { class: "max-w-4xl mx-auto flex items-center justify-between px-6 py-3",
                // Brand
                Link {
                    to: Route::Todos {},
                    class: "text-lg font-bold text-gray-800 hover:text-blue-500 transition-colors duration-200",
                    "Todo App"
                }

                // Nav links
                div { class: "flex items-center gap-1",
                    Link {
                        to: Route::Todos {},
                        class: "px-4 py-2 text-sm font-medium text-gray-600 hover:text-blue-500 hover:bg-blue-50 rounded-lg transition-colors duration-200",
                        "Todos"
                    }
                    Link {
                        to: Route::Profile { id: 1 },
                        class: "px-4 py-2 text-sm font-medium text-gray-600 hover:text-blue-500 hover:bg-blue-50 rounded-lg transition-colors duration-200",
                        "Profile"
                    }
                }
            }
        }

        Outlet::<Route> {}
    }
}
