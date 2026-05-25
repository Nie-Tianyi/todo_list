use dioxus::prelude::*;

use auth::AuthContext;
use views::Login;
use views::Navbar;
use views::PageNoteFound;
use views::Profile;
use views::Register;
use views::RequireAuth;
use views::Todos;

mod auth;
mod components;
mod backend;
mod models;
mod views;

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
            #[route("/profile")]
            Profile {},
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

    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}
