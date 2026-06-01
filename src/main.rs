use dioxus::prelude::*;

use auth::AuthContext;
use views::Archives;
use views::Documents;
use views::Gantt;
use views::Login;
use views::Navbar;
use views::PageNoteFound;
use views::Profile;
use views::Register;
use views::RequireAuth;
use views::Team;
use views::Todos;

mod auth;
mod backend;
mod components;
mod models;
#[cfg(feature = "server")]
mod server;
mod views;

#[derive(Clone, Copy)]
pub struct DarkModeContext {
    pub is_dark: Signal<bool>,
}

impl DarkModeContext {
    pub fn toggle(mut self) {
        let new_val = !(self.is_dark)();
        *self.is_dark.write() = new_val;
    }
}

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
        #[route("/login")]
        Login {},
        #[route("/register")]
        Register {},
        #[layout(RequireAuth)]
            #[route("/")]
            Todos {},
            #[route("/gantt")]
            Gantt {},
            #[route("/documents")]
            Documents {},
            #[route("/profile")]
            Profile {},
            #[route("/team")]
            Team {},
            #[route("/archives")]
            Archives {},
    #[route("/:..segments")]
    PageNoteFound { segments: Vec<String> }
}

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let auth = AuthContext(use_signal(|| None::<auth::AuthState>));
    use_context_provider(|| auth);

    // Dark mode
    let mut is_dark = use_signal(|| false);

    // Read preference from localStorage on mount
    use_effect(move || {
        spawn(async move {
            #[cfg(feature = "web")]
            if let Ok(val) = dioxus::document::eval("localStorage.getItem('darkMode') === 'true'").await
            {
                if let serde_json::Value::Bool(true) = val {
                    *is_dark.write() = true;
                }
            }
        });
    });

    // Sync dark mode to DOM and localStorage
    use_effect(move || {
        let dark = is_dark();
        #[cfg(feature = "web")]
        {
            spawn(async move {
                let _ = dioxus::document::eval(
                    &format!(
                        "document.documentElement.classList.toggle('dark', {dark}); localStorage.setItem('darkMode', '{dark}')"
                    )
                ).await;
            });
        }
    });

    let dark_mode = DarkModeContext { is_dark };
    use_context_provider(|| dark_mode);

    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}
