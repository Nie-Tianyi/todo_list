use dioxus::prelude::*;

#[component]
pub fn UserSelector(current_user: Signal<String>) -> Element {
    rsx! {
        div { class: "flex items-center gap-2",
            label {
                r#for: "current-user",
                class: "text-sm text-gray-500 whitespace-nowrap",
                "👤"
            }
            input {
                id: "current-user",
                r#type: "text",
                class: "border border-gray-200 rounded-lg px-3 py-1.5 text-sm w-32
                        focus:outline-none focus:ring-2 focus:ring-blue-200 focus:border-blue-300
                        placeholder:text-gray-300 transition-shadow",
                placeholder: "Your name",
                value: "{current_user}",
                oninput: move |e| current_user.set(e.value()),
            }
        }
    }
}
