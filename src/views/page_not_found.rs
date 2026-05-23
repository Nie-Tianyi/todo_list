use dioxus::prelude::*;

#[component]
pub fn PageNoteFound(segments: Vec<String>) -> Element {
    let segments = segments.join("/");
    error!("unknown routing segments: {segments}");
    rsx! {
        div { class: "flex flex-col gap-2 items-center justify-center min-h-screen",
            h1 { class: "font-bold text-6xl text-center", "Requested Page Not Found" }
        }
    }
}
