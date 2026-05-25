use crate::auth::AuthContext;
use dioxus::prelude::*;

#[component]
pub fn Profile() -> Element {
    let auth = use_context::<AuthContext>();

    rsx! {
        div { class: "max-w-2xl mx-auto px-6 py-12",
            h1 { class: "text-2xl font-bold text-gray-800 mb-6", "Profile" }

            if let Some(user) = auth.user() {
                div { class: "bg-white rounded-xl border border-gray-100 shadow-sm p-6 space-y-4",
                    div {
                        span { class: "text-sm text-gray-500", "Username" }
                        p { class: "text-lg font-medium text-gray-800", "{user.username}" }
                    }
                    div {
                        span { class: "text-sm text-gray-500", "User ID" }
                        p { class: "text-lg font-medium text-gray-800", "{user.id}" }
                    }
                }
            } else {
                p { class: "text-gray-500", "Not logged in." }
            }
        }
    }
}
