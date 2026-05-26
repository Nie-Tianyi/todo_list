use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn PageNoteFound(segments: Vec<String>) -> Element {
    let segments = segments.join("/");
    error!("unknown routing segments: {segments}");

    rsx! {
        div { class: "flex flex-col items-center justify-center min-h-screen bg-gray-50 dark:bg-gray-950 px-4",
            // Decorative background blob
            div { class: "absolute top-1/4 left-1/2 -translate-x-1/2 -translate-y-1/2 w-96 h-96 bg-blue-100 dark:bg-blue-900/30 rounded-full blur-3xl opacity-50 -z-10" }

            // 404 large text
            h1 { class: "text-9xl font-extrabold text-gray-200 dark:text-gray-700 select-none",
                "404"
            }

            // Main heading
            h2 { class: "mt-4 text-3xl font-bold text-gray-800 dark:text-gray-100 text-center",
                "Page Not Found"
            }

            // Description
            p { class: "mt-3 text-gray-500 dark:text-gray-400 text-center max-w-md leading-relaxed",
                "The page you're looking for doesn't exist or has been moved. Check the URL and try again."
            }

            // Back to home button
            Link {
                to: Route::Todos {},
                class: "mt-8 inline-flex items-center gap-2 px-6 py-3 bg-blue-500 hover:bg-blue-600 text-white font-medium rounded-lg transition-colors duration-200 shadow-sm hover:shadow-md",
                // Left arrow icon
                span { class: "text-lg", "\u{2190}" }
                "Back to Home"
            }
        }
    }
}
